use std::borrow::Borrow;
use std::collections::VecDeque;
use std::ops::Index;

use augmented_midi::{
	parse_midi_file, MIDIFile, MIDIFileChunk, MIDIFileDivision, MIDITrackEvent, MIDITrackInner,
};
use helgoboss_midi::{ShortMessageFactory, StructuredShortMessage};

#[derive(Debug, Clone)]
pub struct MidiTrackAccumulateEvent {
	pub time: u64,
	pub inner: MidiEvent,
}

#[derive(Debug, Clone)]
pub struct MidiTrack {
	pub events: Vec<MidiTrackAccumulateEvent>,
	pub ticks_per_beat: u16,
}

impl MidiTrack {
	pub fn from_midi_file<
		StringRepr: Borrow<str>,
		Buffer: Borrow<[u8]> + Clone + Index<usize, Output = u8>,
	>(
		file: MIDIFile<StringRepr, Buffer>,
	) -> Self {
		let mut events = Vec::new();
		let mut time = 0_u64;
		let mut tracks: Vec<VecDeque<MIDITrackEvent<Buffer>>> = file
			.chunks
			.iter()
			.filter_map(|chunk| match chunk {
				MIDIFileChunk::Track { events } => Some(events.iter().cloned().collect()),
				_ => None,
			})
			.collect();

		while tracks.iter().any(|track| !track.is_empty()) {
			let next_event_track_index = tracks
				.iter()
				.enumerate()
				.filter_map(|(i, track)| track.front().map(|event| (i, event)))
				.min_by_key(|(_, event)| event.delta_time())
				.map(|(i, _)| i)
				.unwrap();
			let next_event = tracks[next_event_track_index].pop_front().unwrap();
			let inner = match next_event.inner {
				MIDITrackInner::Message(message) => {
					let bytes = Vec::<u8>::from(message);
					MidiEvent::Message(
						StructuredShortMessage::from_bytes((
							bytes[0],
							bytes
								.get(1)
								.copied()
								.unwrap_or_default()
								.try_into()
								.expect("Data 1 high bit set"),
							bytes
								.get(2)
								.copied()
								.unwrap_or_default()
								.try_into()
								.expect("Data 2 high bit set"),
						))
						.expect("Failed to parse MIDI message"),
					)
				}
				MIDITrackInner::Meta(meta) => match meta.meta_type {
					0x51 => {
						let microseconds_per_beat =
							u32::from_be_bytes([0, meta.bytes[0], meta.bytes[1], meta.bytes[2]]);
						let tempo = 60_000_000.0 / microseconds_per_beat as f64;
						MidiEvent::Meta(MidiMetaEvent::Tempo { tempo })
					}
					_ => continue,
				},
			};
			time += next_event.delta_time as u64;
			events.push(MidiTrackAccumulateEvent { time, inner });

			for track in tracks
				.iter_mut()
				.enumerate()
				.filter(|(i, _)| *i != next_event_track_index)
				.map(|(_, track)| track)
			{
				let mut remaining_time = next_event.delta_time;
				for event in track.iter_mut() {
					if remaining_time == 0 {
						break;
					}

					let sub = event.delta_time.min(remaining_time);
					event.delta_time -= sub;
					remaining_time -= sub;
				}
			}
		}

		Self {
			events,
			ticks_per_beat: match file
				.header()
				.expect("MIDI file must have a header chunk")
				.division
			{
				MIDIFileDivision::TicksPerQuarterNote {
					ticks_per_quarter_note,
				} => ticks_per_quarter_note,
				_ => panic!("Invalid MIDI file division"),
			},
		}
	}

	pub fn from_bytes(bytes: &[u8]) -> Self {
		Self::from_midi_file(
			parse_midi_file::<String, Vec<u8>>(bytes)
				.expect("Failed to parse MIDI file")
				.1,
		)
	}
}

#[derive(Debug, Clone)]
pub enum MidiEvent {
	Meta(MidiMetaEvent),
	Message(StructuredShortMessage),
}

#[derive(Debug, Clone)]
pub enum MidiMetaEvent {
	Tempo { tempo: f64 },
}
