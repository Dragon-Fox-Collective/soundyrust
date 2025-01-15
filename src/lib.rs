use bevy::audio::AddAudioSource;
use bevy::prelude::*;

pub use midi::MidiTrack;
pub use notes::Note;
pub use rustysynth::SoundFont;
pub use source::{MidiAudio, MidiBufferMessage, MidiTrackAudio, SyncedMidiInfo};

mod midi;
mod notes;
mod source;

pub struct SoundyPlugin;

impl Plugin for SoundyPlugin {
	fn build(&self, app: &mut App) {
		app.add_audio_source::<MidiAudio>()
			.add_systems(PreUpdate, tick_sequencers)
			.add_systems(PostUpdate, clear_old_buffer_events);
	}
}

fn tick_sequencers(mut audios: ResMut<Assets<MidiAudio>>, time: Res<Time>) {
	for (_id, audio) in audios.iter_mut() {
		audio.tick(time.delta());
	}
}

fn clear_old_buffer_events(mut audios: ResMut<Assets<MidiAudio>>) {
	for (_id, audio) in audios.iter_mut() {
		audio.clear_old_buffer_events();
	}
}
