use std::time::Duration;

use bevy::audio::AddAudioSource;
use bevy::prelude::*;

pub use midi::MidiTrack;
pub use notes::Note;
pub use rustysynth::SoundFont;
pub use source::{MidiAudio, MidiBufferMessage, MidiSequencer, SyncedMidiInfo};

mod midi;
mod notes;
mod source;

pub struct SoundyPlugin;

impl Plugin for SoundyPlugin {
	fn build(&self, app: &mut App) {
		app.add_audio_source::<MidiAudio>()
			.insert_resource(MidiTimer(Timer::new(
				Duration::from_secs_f64(1.0 / 44100.0),
				TimerMode::Repeating,
			)))
			.add_systems(PreUpdate, tick_sequencers);
	}
}

fn tick_sequencers(
	audios: Query<&Handle<MidiAudio>>,
	mut sequencers: ResMut<Assets<MidiAudio>>,
	time: Res<Time>,
	mut timer: ResMut<MidiTimer>,
) {
	timer.0.tick(time.delta());
	let ticks = timer.0.times_finished_this_tick();
	for audio in audios.iter() {
		let audio = sequencers.get_mut(audio).unwrap();
		let last_ticks = audio.buffer.lock().unwrap().len();
		println!(
			"Ticks: {} (from {})",
			ticks as usize + last_ticks,
			last_ticks
		);
		audio.sequencer.tick(ticks);
	}
}

#[derive(Resource)]
pub struct MidiTimer(Timer);
