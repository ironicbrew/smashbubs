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

// ! Code cleanup / plugin refactor needed
// TODO: Add multiple players
// TODO: Everything disappears once it hits the side of the window
// TODO: Bullet collision detection => {remove bullet, deal damage to target if is player}
// TODO: Ability to say which direction a bullet shoots
// TODO: Add side of window collision tracking reset with life minus
// TODO: Add double jump limit with collision reset tracking
// TODO: Add sprite sheet that responds to player's last movement
// TODO: Add ability to have multiple players
// TODO: Add ability to damage other players with projectiles
// TODO: Add other weapons
// TODO: Add haptic control to weapons
// TODO: Ability to shoot direction depending on where pointing with controller
// TODO: Add some indication of where projectile will go
// TODO: Add lives left display UI
// TODO: Add response to getting hit with projectile (damage applied, thrown back based on damage, unable to attack for a moment)
// TODO: Decrease life and respawn if hitting side of map.

//? Charater with ability to teleport
//? Character with ability to create platforms (gun that creates platforms?)

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

// App::build()
// .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
// .insert_resource(WindowDescriptor {
//     title: "Smash Poopers!".to_string(),
//     width: 600.,
//     height: 600.,
//     ..Default::default()
// })
// .add_plugins(DefaultPlugins)
// .add_plugin(PhysicsPlugin::default())
// .insert_resource(Gravity::from(Vec3::new(0.0, -1000., 0.0)))
// .add_startup_system(setup.system())
// .add_startup_system(add_block.system())
// .add_system(player_movement.system())
// .add_system(player_jump.system())
// .add_system(add_projectile.system())
// .add_system(gamepad_connections.system())
// .add_system(gamepad_input.system())
// .add_plugin(HelloPlugin)
// .add_plugin(WorldInspectorPlugin::new())
// .run();
