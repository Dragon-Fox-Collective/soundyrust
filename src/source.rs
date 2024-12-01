use std::sync::Arc;

use bevy::utils::hashbrown::HashMap;
use bevy::{audio::Source, prelude::*, utils::Duration};
use helgoboss_midi::StructuredShortMessage;
use rustysynth::SoundFont;

use crate::midi::{MidiEvent, MidiTrack};
use crate::notes::Note;

#[derive(Asset, TypePath)]
pub struct MidiAudio {
	pub midi_track: MidiTrack,
	pub soundfont: Arc<SoundFont>,
}

pub struct MidiDecoder {
	midi_track: MidiTrack,
	soundfont: Arc<SoundFont>,
	voices: Vec<Voice>,
	channels: Vec<Channel>,
	num_audio_channels: u16,
	current_audio_channel: u16,
	samples_per_second: f64,
	ticks_per_sample: f64,
	tick: f64,
	event_index: usize,
	preset_index: HashMap<(u8, u8), usize>,
}

impl MidiDecoder {
	fn new(midi_track: MidiTrack, soundfont: Arc<SoundFont>) -> Self {
		let samples_per_second = 44100.0;
		let beats_per_second = 120.0 / 60.0;
		let ticks_per_beat = midi_track.ticks_per_beat as f64;
		let ticks_per_sample = (ticks_per_beat * beats_per_second) / samples_per_second;

		let channels = (0..16)
			.map(|i| Channel {
				bank_number: if i == 9 { 128 } else { 0 },
				patch_number: 0,
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

		MidiDecoder {
			midi_track,
			soundfont,
			voices: vec![],
			channels,
			num_audio_channels: 2,
			current_audio_channel: 0,
			samples_per_second,
			ticks_per_sample,
			tick: 0.0,
			event_index: 0,
			preset_index,
		}
	}
}

impl Iterator for MidiDecoder {
	type Item = i16;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current_audio_channel == 0 {
			self.tick += self.ticks_per_sample;

			while let Some(event) = self
				.midi_track
				.events
				.get(self.event_index)
				.filter(|event| event.time <= self.tick as u64)
			{
				match event.inner {
					MidiEvent::Message(StructuredShortMessage::NoteOn {
						channel,
						key_number,
						velocity,
					}) => {
						let channel: usize = channel.into();
						let channel = &self.channels[channel];
						let Some(&preset_index) = self
							.preset_index
							.get(&(channel.bank_number, channel.patch_number))
						else {
							continue;
						};
						let preset = &self.soundfont.get_presets()[preset_index];
						let Some(preset_region) = preset
							.get_regions()
							.iter()
							.find(|region| region.contains(key_number.into(), velocity.into()))
						else {
							continue;
						};
						let instrument =
							&self.soundfont.get_instruments()[preset_region.get_instrument_id()];
						let Some(instrument_region) = instrument
							.get_regions()
							.iter()
							.find(|region| region.contains(key_number.into(), velocity.into()))
						else {
							continue;
						};
						self.voices.push(Voice {
							speed: Note::from_position(key_number.into()).frequency
								/ Note::from_position(instrument_region.get_root_key() as u8)
									.frequency,
							current_sample: 0.0,
							start_sample: instrument_region.get_sample_start() as f64,
							end_sample: instrument_region.get_sample_end() as f64,
							num_audio_channels: self.num_audio_channels,
						});
					}
					_ => {}
				}
				self.event_index += 1;

				if self.event_index >= self.midi_track.events.len() {
					self.event_index = 0;
					self.tick = 0.0;
				}
			}
		}

		let sample = self
			.voices
			.iter()
			.map(|voice| {
				let current_sample = voice.current_sample(self.current_audio_channel);
				let wave_data = self.soundfont.get_wave_data();
				let floor = wave_data[current_sample.floor() as usize];
				let ceil = wave_data[current_sample.ceil() as usize];
				let fraction = current_sample.fract() as f32;
				ceil as f32 * fraction + floor as f32 * (1.0 - fraction)
			})
			.sum::<f32>() as i16;

		if self.current_audio_channel == 0 {
			self.voices.iter_mut().for_each(Voice::tick);
			self.voices.retain(Voice::alive);
		}
		self.current_audio_channel = (self.current_audio_channel + 1) % self.num_audio_channels;

		Some(sample)
	}
}

impl Source for MidiDecoder {
	fn current_frame_len(&self) -> Option<usize> {
		None
	}

	fn channels(&self) -> u16 {
		self.num_audio_channels
	}

	fn sample_rate(&self) -> u32 {
		self.samples_per_second as u32
	}

	fn total_duration(&self) -> Option<Duration> {
		None
	}
}

impl Decodable for MidiAudio {
	type DecoderItem = <MidiDecoder as Iterator>::Item;

	type Decoder = MidiDecoder;

	fn decoder(&self) -> Self::Decoder {
		MidiDecoder::new(self.midi_track.clone(), self.soundfont.clone())
	}
}

struct Voice {
	speed: f32,
	current_sample: f64,
	start_sample: f64,
	end_sample: f64,
	num_audio_channels: u16,
}

impl Voice {
	fn tick(&mut self) {
		self.current_sample += self.speed as f64;
	}

	fn alive(&self) -> bool {
		self.current_sample(0) < self.end_sample
	}

	fn current_sample(&self, current_audio_channel: u16) -> f64 {
		self.start_sample
			+ self.current_sample * self.num_audio_channels as f64
			+ current_audio_channel as f64
	}
}

struct Channel {
	bank_number: u8,
	patch_number: u8,
}
