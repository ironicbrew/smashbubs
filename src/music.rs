use bevy::prelude::*;

pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(play_music);
    }
}

fn play_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let gunsound = asset_server.load("sounds/theme_music.ogg");
    audio.play(gunsound);
}