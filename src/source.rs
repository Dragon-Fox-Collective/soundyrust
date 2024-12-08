use std::collections::VecDeque;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use bevy::utils::hashbrown::HashMap;
use bevy::{audio::Source, prelude::*, utils::Duration};
use num_enum::TryFromPrimitive;
use rustysynth::SoundFont;

use crate::midi::{MidiEvent, MidiTrack};

#[derive(Asset, TypePath)]
pub struct MidiAudio {
	midi_track: MidiTrack,
	soundfont: Arc<SoundFont>,
	/// Channel => Note => Voice
	channels: HashMap<u8, Channel>,
	num_audio_channels: u16,
	current_audio_channel: u16,
	samples_per_second: f64,
	ticks_per_sample: f64,
	beats_per_second: f64,
	tick: f64,
	event_index: usize,
	preset_index: HashMap<(u8, u8), usize>,
	buffer: Arc<Mutex<VecDeque<i16>>>,
	buffer_events: Vec<(Instant, MidiBufferMessage)>,
	buffer_event_now: Instant,
}

impl MidiAudio {
	pub fn new(midi_track: MidiTrack, soundfont: Arc<SoundFont>) -> Self {
		let mut buffer_events = Vec::new();

		let samples_per_second = 44100.0;
		let beats_per_second = 120.0 / 60.0;
		let ticks_per_beat = midi_track.ticks_per_beat as f64;
		let ticks_per_sample = (ticks_per_beat * beats_per_second) / samples_per_second;
		buffer_events.push((
			Instant::now(),
			MidiBufferMessage::TempoChange { beats_per_second },
		));
		let channels = (0..16)
			.map(|i| {
				(
					i,
					Channel {
						bank_number: if i == 9 { 128 } else { 0 },
						patch_number: 0,
						voices: HashMap::new(),
					},
				)
			})
			.collect();

		let preset_index = soundfont
			.get_presets()
			.iter()
			.enumerate()
			.map(|(index, preset)| {
				(
					(
						preset.get_bank_number() as u8,
						preset.get_patch_number() as u8,
					),
					index,
				)
			})
			.collect();

		Self {
			midi_track,
			soundfont,
			channels,
			num_audio_channels: 2,
			current_audio_channel: 0,
			samples_per_second,
			ticks_per_sample,
			beats_per_second,
			tick: 0.0,
			event_index: 0,
			preset_index,
			buffer: Arc::new(Mutex::new(VecDeque::new())),
			buffer_events,
			buffer_event_now: Instant::now(),
		}
	}

	pub fn from_bytes(track_bytes: &[u8], soundfont_bytes: &[u8]) -> Self {
		let midi_track = MidiTrack::from_bytes(track_bytes);
		let soundfont = Arc::new(SoundFont::new(&mut Cursor::new(soundfont_bytes)).unwrap());
		Self::new(midi_track, soundfont)
	}

	pub fn with_channel_patch(
		mut self,
		channel_number: u8,
		bank_number: u8,
		patch_number: u8,
	) -> Self {
		self.channels.insert(
			channel_number,
			Channel {
				bank_number,
				patch_number,
				voices: HashMap::new(),
			},
		);
		self
	}

	pub fn tick(&mut self, delta: Duration) {
		self.buffer_event_now += delta;

		let ticks = delta.as_secs_f64() * self.samples_per_second;
		let max_ticks = self.samples_per_second
			- self.buffer.lock().unwrap().len() as f64 / self.num_audio_channels as f64;
		let ticks = ticks.min(max_ticks) as usize;

		let mut buffer = VecDeque::with_capacity(ticks);
		for _ in 0..ticks * self.num_audio_channels as usize {
			self.tick_once(&mut buffer);
		}

		let buffer = buffer
			.into_iter()
			.enumerate()
			.filter_map(|(i, message)| match message {
				MidiBufferMessage::Audio(sample) => Some(sample),
				_ => {
					self.buffer_events.push((
						self.buffer_event_now
							+ Duration::from_secs_f64(i as f64 / self.samples_per_second),
						message,
					));
					None
				}
			});
		self.buffer.lock().unwrap().extend(buffer);
	}

	fn tick_once(&mut self, buffer: &mut VecDeque<MidiBufferMessage>) {
		if self.current_audio_channel == 0 {
			self.tick += self.ticks_per_sample;

			while let Some(event) = self
				.midi_track
				.events
				.get(self.event_index)
				.filter(|event| event.time <= self.tick as u64)
			{
				match event.inner {
					MidiEvent::NoteOn {
						channel,
						note,
						velocity,
					} => {
						if let Some(voice) = self.create_voice(channel, note, velocity) {
							if let Some(channel) = self.channels.get_mut(&channel) {
								channel.voices.insert(note, voice);
							}
						}
					}
					MidiEvent::NoteOff { channel, note } => {
						if let Some(channel) = self.channels.get_mut(&channel) {
							channel.voices.remove(&note);
						}
					}
					MidiEvent::SetTempo {
						tempo: beats_per_minute,
					} => {
						self.beats_per_second = beats_per_minute / 60.0;
						buffer.push_back(MidiBufferMessage::TempoChange {
							beats_per_second: self.beats_per_second,
						});
						self.ticks_per_sample = (self.midi_track.ticks_per_beat as f64
							* self.beats_per_second)
							/ self.samples_per_second;
					}
				}
				self.event_index += 1;

				if self.event_index >= self.midi_track.events.len() {
					self.event_index = 0;
					self.tick = 0.0;
				}
			}
		}

		let sample = self
			.channels
			.values()
			.flat_map(|channel| channel.voices.values())
			.map(|voice| voice.sample(self.soundfont.get_wave_data(), self.current_audio_channel))
			.sum::<i32>()
			.clamp(i16::MIN as i32, i16::MAX as i32) as i16;

		if self.current_audio_channel == 0 {
			self.channels
				.values_mut()
				.flat_map(|channel| channel.voices.values_mut())
				.for_each(Voice::tick);
		}
		self.current_audio_channel = (self.current_audio_channel + 1) % self.num_audio_channels;

		buffer.push_back(MidiBufferMessage::Audio(sample));
	}

	fn create_voice(&self, channel_index: u8, note: u8, velocity: u8) -> Option<Voice> {
		let note = note as i32;
		let velocity = velocity as i32;

		let channel = &self.channels[&channel_index];
		let &preset_index = self
			.preset_index
			.get(&(channel.bank_number, channel.patch_number))?;
		let preset = &self.soundfont.get_presets()[preset_index];
		let preset_regions = preset
			.get_regions()
			.iter()
			.filter(|region| region.contains(note, velocity));
		let instruments = preset_regions
			.map(|region| &self.soundfont.get_instruments()[region.get_instrument_id()]);
		let instrument_regions = instruments.flat_map(|instrument| {
			instrument
				.get_regions()
				.iter()
				.filter(|region| region.contains(note, velocity))
		});
		let samples = instrument_regions
			.map(|region| &self.soundfont.get_sample_headers()[region.get_sample_id()]);
		let samples = samples
			.map(|sample| VoiceSample {
				speed: 2_f32.powf(
					(note as f32 - sample.get_original_pitch() as f32
						+ sample.get_pitch_correction() as f32 / 100.0)
						/ 12.0,
				),
				current_sample: sample.get_start() as f64,
				end_sample: sample.get_end() as f64,
				sample_type: sample.get_sample_type().try_into().unwrap(),
			})
			.collect::<Vec<_>>();
		if samples.is_empty() {
			return None;
		}
		Some(Voice { samples })
	}

	pub fn clear_old_buffer_events(&mut self) {
		self.buffer_events
			.retain(|(time, _)| *time > self.buffer_event_now);
	}

	pub fn beats_per_second(&self) -> f64 {
		self.beats_per_second
	}
}

pub struct MidiDecoder {
	buffer: Arc<Mutex<VecDeque<i16>>>,
	num_audio_channels: u16,
	samples_per_second: u32,
}

impl Iterator for MidiDecoder {
	type Item = i16;

	fn next(&mut self) -> Option<Self::Item> {
		self.buffer.lock().unwrap().pop_front().or(Some(0))
	}
}

impl Source for MidiDecoder {
	fn current_frame_len(&self) -> Option<usize> {
		if self.buffer.lock().unwrap().is_empty() {
			Some(1)
		} else {
			None
		}
	}

	fn channels(&self) -> u16 {
		self.num_audio_channels
	}

	fn sample_rate(&self) -> u32 {
		self.samples_per_second
	}

	fn total_duration(&self) -> Option<Duration> {
		None
	}
}

impl Decodable for MidiAudio {
	type DecoderItem = <MidiDecoder as Iterator>::Item;

	type Decoder = MidiDecoder;

	fn decoder(&self) -> Self::Decoder {
		MidiDecoder {
			buffer: self.buffer.clone(),
			num_audio_channels: self.num_audio_channels,
			samples_per_second: self.samples_per_second as u32,
		}
	}
}

struct Voice {
	samples: Vec<VoiceSample>,
}

impl Voice {
	fn tick(&mut self) {
		self.samples.iter_mut().for_each(VoiceSample::tick);
	}

	fn sample(&self, wave_data: &[i16], current_audio_channel: u16) -> i32 {
		self.samples
			.iter()
			.filter(|sample| sample.current_sample < sample.end_sample) // Remove this once loops are implemented
			.filter(|sample| {
				sample.sample_type == SampleType::Mono || {
					if current_audio_channel == 0 {
						sample.sample_type == SampleType::Left
					} else {
						sample.sample_type == SampleType::Right
					}
				}
			})
			.map(|sample| sample.current_sample)
			.map(|sample| {
				// This seems like such a hassle... Do we really need to interpolate?
				let floor = wave_data[sample.floor() as usize] as f32;
				let ceil = wave_data[sample.ceil() as usize] as f32;
				let fraction = sample.fract() as f32;
				(ceil * fraction + floor * (1.0 - fraction)) as i32
			})
			.sum::<i32>()
	}
}

struct VoiceSample {
	speed: f32,
	current_sample: f64,
	end_sample: f64,
	sample_type: SampleType,
}

impl VoiceSample {
	fn tick(&mut self) {
		self.current_sample += self.speed as f64;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
enum SampleType {
	Mono = 1,
	Right = 2,
	Left = 4,
	// There's also a "linked" type but I'm unsure when this would be used, usually `link` is just the other stereo channel
}

struct Channel {
	bank_number: u8,
	patch_number: u8,
	voices: HashMap<u8, Voice>,
}

#[derive(Default, Clone)]
pub struct SyncedMidiInfo {
	pub beat: f64,
	pub beats_per_second: f64,
}

pub enum MidiBufferMessage {
	Audio(i16),
	TempoChange { beats_per_second: f64 },
}
