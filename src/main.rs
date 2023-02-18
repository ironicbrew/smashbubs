use bevy::{prelude::*};
use player::PlayerPlugin;
mod player;
use gamepad::GamepadPlugin;
mod gamepad;
mod camera;
use camera::CameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamepadPlugin)
        .run();
}
