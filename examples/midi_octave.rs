use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use soundyrust::*;

fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins.set(AudioPlugin {
		global_volume: GlobalVolume::new(0.2),
		..default()
	}))
	.add_plugins(SoundyPlugin)
	.add_systems(Startup, setup)
	.run();
}

fn setup(mut assets: ResMut<Assets<MidiAudio>>, mut commands: Commands) {
	let audio_handle = assets.add(MidiAudio {
		midi_track: MidiTrack::from_bytes(include_bytes!("../assets/octave.mid")),
		bytes: include_bytes!("../assets/flute.wav"),
		baseline_note: Note::C4,
	});
	commands.spawn((AudioSourceBundle {
		source: audio_handle,
		..default()
	},));
}
