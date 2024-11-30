use bevy::audio::{AddAudioSource, AudioPlugin};
use bevy::prelude::*;
use source::WavAudio;

mod midi;
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
		bytes: include_bytes!("../assets/flute.wav"),
	});
	commands.spawn(AudioSourceBundle {
		source: audio_handle,
		..default()
	});
}
