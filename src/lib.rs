use bevy::audio::{AddAudioSource, AudioOutput};
use bevy::prelude::*;

pub use midi::MidiTrack;
pub use notes::Note;
use rodio::cpal::BufferSize;
use rodio::OutputStream;
pub use rustysynth::SoundFont;
pub use source::{MidiAudio, SyncedMidiInfo};

mod midi;
mod notes;
mod source;

#[derive(Default)]
pub struct SoundyPlugin {
	pub buffer_size: Option<u32>,
}

impl Plugin for SoundyPlugin {
	fn build(&self, app: &mut App) {
		app.add_audio_source::<MidiAudio>();

		if let Some(buffer_size) = self.buffer_size {
			let stream_handle =
				match OutputStream::try_default_with_buffer_size(BufferSize::Fixed(buffer_size)) {
					Ok((stream, stream_handle)) => {
						std::mem::forget(stream);
						Some(stream_handle)
					}
					Err(err) => {
						warn!("No audio device found. Or it broke. {}", err);
						None
					}
				};
			app.insert_resource(AudioOutput { stream_handle });
		}
	}
}
