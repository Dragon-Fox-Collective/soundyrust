use bevy::audio::AddAudioSource;
use bevy::prelude::*;

pub use midi::MidiTrack;
pub use notes::Note;
pub use rustysynth::SoundFont;
pub use source::{MidiAudio, SyncedMidiInfo};

mod midi;
mod notes;
mod source;

pub struct SoundyPlugin;

impl Plugin for SoundyPlugin {
	fn build(&self, app: &mut App) {
		app.add_audio_source::<MidiAudio>();
	}
}
