use bevy::{prelude::*};
use player::PlayerPlugin;
mod player;
use gamepad::GamepadPlugin;
mod gamepad;
mod camera;
use camera::CameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod gilrs_plugin;
use gilrs_plugin::GilrsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(GilrsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamepadPlugin)
        .run();
}
