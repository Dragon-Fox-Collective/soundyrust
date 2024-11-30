use std::io::{self, Cursor};

use bevy::{audio::Source, prelude::*, utils::Duration};
use helgoboss_midi::StructuredShortMessage;
use hound::{SampleFormat, WavReader, WavSpec};

use crate::midi::{MidiEvent, MidiTrack};

#[derive(Asset, TypePath)]
pub struct WavAudio {
	pub midi_track: MidiTrack,
	pub bytes: &'static [u8],
}

pub struct WavDecoder {
	midi_track: MidiTrack,
	header: WavSpec,
	samples: Vec<i16>,
	voices: Vec<usize>,
	current_channel: u16,
	beats_per_second: f64,
	ticks_per_beat: f64,
	ticks_per_sample: f64,
	tick: f64,
	event_index: usize,
}

impl WavDecoder {
	fn new<R: io::Read>(midi_track: MidiTrack, reader: WavReader<R>) -> Self {
		let header = reader.spec();

		let samples_per_second = header.sample_rate as f64;
		let beats_per_second = 120.0 / 60.0;
		let ticks_per_beat = midi_track.ticks_per_beat as f64;
		let ticks_per_sample = (ticks_per_beat * beats_per_second) / samples_per_second;

		WavDecoder {
			midi_track,
			header,
			samples: match (header.sample_format, header.bits_per_sample) {
				(SampleFormat::Float, 32) => reader
					.into_samples()
					.map(|value| f32_to_i16(value.unwrap()))
					.collect(),
				(SampleFormat::Int, 8) => reader
					.into_samples()
					.map(|value| i8_to_i16(value.unwrap()))
					.collect(),
				(SampleFormat::Int, 16) => {
					reader.into_samples().map(|value| value.unwrap()).collect()
				}
				(SampleFormat::Int, 24) => reader
					.into_samples()
					.map(|value| i24_to_i16(value.unwrap()))
					.collect(),
				(SampleFormat::Int, 32) => reader
					.into_samples()
					.map(|value| i32_to_i16(value.unwrap()))
					.collect(),
				(sample_format, bits_per_sample) => {
					panic!("Unimplemented wav spec: {sample_format:?}, {bits_per_sample}")
				}
			},
			voices: vec![],
			current_channel: 0,
			beats_per_second,
			ticks_per_beat,
			ticks_per_sample,
			tick: 0.0,
			event_index: 0,
		}
	}
}

impl Iterator for WavDecoder {
	type Item = i16;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current_channel == 0 {
			self.tick += self.ticks_per_sample;

			while let Some(event) = self
				.midi_track
				.events
				.get(self.event_index)
				.filter(|event| event.time <= self.tick as u64)
			{
				match event.inner {
					MidiEvent::Message(StructuredShortMessage::NoteOn { .. }) => {
						self.voices.push(0);
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

		let sample = self.voices.iter().map(|&index| self.samples[index]).sum();

		self.voices = self
			.voices
			.iter()
			.map(|&index| index + 1)
			.filter(|&index| index < self.samples.len())
			.collect();
		self.current_channel = (self.current_channel + 1) % self.header.channels;

		Some(sample)
	}
}

impl Source for WavDecoder {
	fn current_frame_len(&self) -> Option<usize> {
		None
	}

	fn channels(&self) -> u16 {
		self.header.channels
	}

	fn sample_rate(&self) -> u32 {
		self.header.sample_rate
	}

	fn total_duration(&self) -> Option<Duration> {
		None
	}
}

impl Decodable for WavAudio {
	type DecoderItem = <WavDecoder as Iterator>::Item;

	type Decoder = WavDecoder;

	fn decoder(&self) -> Self::Decoder {
		WavDecoder::new(
			self.midi_track.clone(),
			WavReader::new(Cursor::new(self.bytes)).unwrap(),
		)
	}
}

/// Returns a 32 bit WAV float as an i16. WAV floats are typically in the range of
/// [-1.0, 1.0] while i16s are in the range [-32768, 32767]. Note that this
/// function definitely causes precision loss but hopefully this isn't too
/// audiable when actually playing?
fn f32_to_i16(f: f32) -> i16 {
	// prefer to clip the input rather than be excessively loud.
	(f.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
}

/// Returns an 8-bit WAV int as an i16. This scales the sample value by a factor
/// of 256.
fn i8_to_i16(i: i8) -> i16 {
	i as i16 * 256
}

/// Returns a 24 bit WAV int as an i16. Note that this is a 24 bit integer, not a
/// 32 bit one. 24 bit ints are in the range [−8,388,608, 8,388,607] while i16s
/// are in the range [-32768, 32767]. Note that this function definitely causes
/// precision loss but hopefully this isn't too audiable when actually playing?
fn i24_to_i16(i: i32) -> i16 {
	(i >> 8) as i16
}

/// Returns a 32 bit WAV int as an i16. 32 bit ints are in the range
/// [-2,147,483,648, 2,147,483,647] while i16s are in the range [-32768, 32767].
/// Note that this function definitely causes precision loss but hopefully this
/// isn't too audiable when actually playing?
fn i32_to_i16(i: i32) -> i16 {
	(i >> 16) as i16
}
