use std::io::Cursor;
use std::sync::Arc;

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
	.add_systems(Update, check_info)
	.run();
}

fn setup(mut assets: ResMut<Assets<MidiAudio>>, mut commands: Commands) {
	let audio_handle = assets.add(MidiAudio::new(
		MidiTrack::from_bytes(include_bytes!("../assets/fray.mid")),
		Arc::new(SoundFont::new(&mut Cursor::new(include_bytes!("../assets/hl4mgm.sf2"))).unwrap()),
	));
	commands.spawn((AudioSourceBundle {
		source: audio_handle,
		..default()
	},));
}

fn check_info(audios: Query<&Handle<MidiAudio>>, assets: ResMut<Assets<MidiAudio>>) {
	for audio in audios.iter() {
		let audio = assets.get(audio).unwrap();
		let synced_info = audio.synced_info.lock().unwrap();
		println!(
			"Beat: {}, Beats per second: {}",
			synced_info.beat, synced_info.beats_per_second
		);
	}
}
