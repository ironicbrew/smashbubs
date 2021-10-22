use super::player::*;
use super::projectile::*;
use bevy::prelude::*;
use heron::prelude::*;

pub const TIME_STEP: f32 = 1.;
const BULLET_SPRITE: &str = "bullet.png";

pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gamepad_connections.system())
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(player_jump.system());
    }
}

struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn player_movement(
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Speed, With<Player>)>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);

    let x = if let Some(x) = axes.get(axis_lx) {
        x
    } else {
        return;
    };

    if let Ok((_, mut transform, speed, _)) = query.single_mut() {
        transform.translation.x += x * TIME_STEP;

        if x != 0. {
            face_player_last_direction_moved(speed.0, transform);
            change_player_direction(speed, x);
        }

        fn face_player_last_direction_moved(speed: f32, mut transform: Mut<Transform>) {
            if speed > 0. {
                transform.rotation = Quat::default();
            } else {
                // ! Broken due to use of physics engine influencing the rotation. Need to use different sprite instead
                // transform.rotation = Quat::from_rotation_z(16.);
            }
        }

        fn change_player_direction(mut speed: Mut<Speed>, direction: f32) {
            speed.0 = direction;
        }
    }
}

fn player_fire(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Speed, With<Player>)>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let fire_button = GamepadButton(gamepad, GamepadButtonType::RightTrigger2);

    if buttons.just_pressed(fire_button) {
        if let Ok((_, transform, _, _)) = query.single_mut() {
            commands
                .spawn()
                .insert_bundle(ProjectileBundle {
                    _p: Projectile,
                    sprite: SpriteBundle {
                        material: materials.add(asset_server.load(BULLET_SPRITE).into()),
                        transform: Transform {
                            scale: Vec3::new(2., 2., 1.),
                            translation: Vec3::new(
                                transform.translation.x + 2.,
                                transform.translation.y,
                                0.,
                            ),
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
                    density: 0.,  // Define the density. Higher value means heavier.
                    friction: 0., // Define the friction. Higher value means higher friction.
                })
                .insert(Velocity::from_linear(Vec3::X * 1000.));
        }
    }
}

fn player_jump(
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Speed, With<Player>)>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    let jump_button = GamepadButton(gamepad, GamepadButtonType::South);

    if buttons.just_pressed(jump_button) {
        // Jump
        if let Ok((mut velocity, _, _, _)) = query.single_mut() {
            velocity.linear = Vec3::Y * 400.;
        };
    }
}
