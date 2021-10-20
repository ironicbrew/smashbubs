use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use heron::prelude::*;

mod player;
use player::*;
mod projectile;
use projectile::*;

const PLAYER_SPRITE: &str = "player.png";
const BLOCK_SPRITE: &str = "block.png";
const BULLET_SPRITE: &str = "bullet.png";
const TIME_STEP: f32 = 1.;

// TODO: Add ability to control with a controller
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
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Smash Poopers!".to_string(),
            width: 600.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -1000., 0.0)))
        .add_startup_system(setup.system())
        .add_startup_system(add_block.system())
        .add_system(player_movement.system())
        .add_system(player_jump.system())
        .add_system(add_projectile.system())
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
    commands
        .spawn()
        .insert_bundle(PlayerBundle {
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
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(8., 8., 1.),
            border_radius: Some(0.),
        })
        .insert(Velocity::from_linear(Vec3::Y * 0.))
        .insert(RotationConstraints::lock());
}

fn add_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
            transform: Transform {
                translation: Vec3::new(1., -100., 1.),
                scale: Vec3::new(24., 24., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(96., 96., 1.),
            border_radius: Some(0.),
        });
}

fn add_projectile(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, With<Player>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((&transform, _)) = query.single_mut() {
            commands
                .spawn()
                .insert_bundle(ProjectileBundle {
                    _p: Projectile,
                    sprite: SpriteBundle {
                        material: materials.add(asset_server.load(BULLET_SPRITE).into()),
                        transform: Transform {
                            scale: Vec3::new(2., 2., 1.),
                            translation: Vec3::new(transform.translation.x + 2., transform.translation.y, 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                })
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(2., 2., 1.),
                    border_radius: Some(0.),
                })
                .insert(PhysicMaterial {
                    restitution: 0.1,
                    density: 0., // Define the density. Higher value means heavier.
                    friction: 0., // Define the friction. Higher value means higher friction.
                })
                .insert(Velocity::from_linear(Vec3::X * 1000.));
        };
    }
}

fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Player>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        if let Ok((mut velocity, _)) = query.single_mut() {
            velocity.linear = Vec3::Y * 400.;
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
