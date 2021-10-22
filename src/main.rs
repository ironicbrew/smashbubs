use bevy::prelude::*;
mod player;
use player::PlayerPlugin;
mod camera;
use camera::CameraPlugin;
mod map;
use map::MapPlugin;
mod gamepad;
use gamepad::GamepadPlugin;
mod _heron;
use _heron::HeronPlugin;
mod window;
use window::WindowPlugin;
mod projectile;

fn main() {
    App::build()
        .add_plugin(WindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(HeronPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamepadPlugin)
        .run();
}
