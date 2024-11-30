use bevy::audio::{AddAudioSource, AudioPlugin};
use bevy::prelude::*;
use midi::MidiTrack;
use notes::Note;
use source::WavAudio;

mod midi;
pub mod notes;
mod source;

fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins.set(AudioPlugin {
		global_volume: GlobalVolume::new(0.2),
		..default()
	}))
	.add_audio_source::<WavAudio>()
	.add_systems(Startup, setup)
	.run();
}

fn setup(mut assets: ResMut<Assets<WavAudio>>, mut commands: Commands) {
	let audio_handle = assets.add(WavAudio {
		midi_track: MidiTrack::from_bytes(include_bytes!("../assets/fray.mid")),
		bytes: include_bytes!("../assets/flute.wav"),
		baseline_note: Note::C4,
	});
	commands.spawn((AudioSourceBundle {
		source: audio_handle,
		..default()
	},));
}
