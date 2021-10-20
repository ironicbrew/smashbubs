use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use heron::prelude::*;

mod player;
use player::*;

const PLAYER_SPRITE: &str = "player.png";
const BLOCK_SPRITE: &str = "block.png";
const TIME_STEP: f32 = 1.;

// TODO: Add collision detection (map pieces and sides of window)
// TODO: Add Map
// TODO: Add Jump Physics
// TODO: Add sprite sheet that responds to player's last movement
// TODO: Add ability to shoot projectiles
// TODO: Add ability to control with a controller
// TODO: Add ability to have multiple players
// TODO: Add ability to damage other players with projectiles
// TODO: Add other weapons
// TODO: Add haptic control to weapons
// TODO: Ability to shoot direction depending on where pointing with controller
// TODO: Add some indication of where projectile will go
// TODO: Add lives system
// TODO: Add response to getting hit with projectile (damage applied, thrown back based on damage, unable to attack for a moment)
// TODO: Decrease life and respawn if hitting side of map.

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Smash Poopers!".to_string(),
            width: 600.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(add_block.system())
        .add_system(player_movement.system())
        .add_system(player_jump.system())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
        .add_plugin(HelloPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // postition window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(100, 100));
}

fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn().insert_bundle(PlayerBundle {
        name: PlayerName("Rob".to_string()),
        damage_taken: player::DamageTaken(0),
        _p: Player,
        speed: Speed(1.),
        sprite: SpriteBundle {
            material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
            transform: Transform {
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
    })
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Cuboid { half_extends: Vec3::new(8., 8., 1.), border_radius: Some(0.) })
    .insert(RotationConstraints::lock());
}

fn add_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn().insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
            transform: Transform {
                translation: Vec3::new(1.,-16.,1.),
                scale: Vec3::new(24., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
    .insert(RigidBody::Static)
    .insert(CollisionShape::Cuboid { half_extends: Vec3::new(96., 8., 1.), border_radius: Some(0.) });
}

fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<Player>)>,
) {
    if keyboard_input.pressed(KeyCode::Up) {
        if let Ok((mut transform, _)) = query.single_mut() {
            transform.translation.y += 1. * TIME_STEP;
        };
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Speed, With<Player>)>,
) {
    let direction = if keyboard_input.pressed(KeyCode::Right) {
        1.
    } else if keyboard_input.pressed(KeyCode::Left) {
        -1.
    } else {
        0.
    };

    if let Ok((mut transform, speed, _)) = query.single_mut() {
        transform.translation.x += direction * TIME_STEP;

        if direction != 0. {
            face_player_last_direction_moved(speed.0, transform);
            change_player_direction(speed, direction);
        }

        fn face_player_last_direction_moved(speed: f32, mut transform: Mut<Transform>) {
            if speed > 0. {
                transform.rotation = Quat::default();
            } else {
                // TODO: Broken due to use of physics engine influencing the rotation. Need to use different sprite instead
                // transform.rotation = Quat::from_rotation_z(16.);
            }
        }

        fn change_player_direction(mut speed: Mut<Speed>, direction: f32) {
            speed.0 = direction;
        }
    }
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    _query: Query<&PlayerName, With<player::Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // for name in query.iter() {
        //     // println!("hello {}!", name.0);
        // }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_player.system())
            .add_system(greet_people.system());
    }
}
