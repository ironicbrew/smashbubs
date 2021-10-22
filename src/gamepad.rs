use super::player::*;
use super::projectile::*;
use bevy::prelude::*;
use heron::prelude::*;

pub const TIME_STEP: f32 = 1.;
const BULLET_SPRITE: &str = "bullet.png";

pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddPlayerEvent>()
            .add_system(gamepad_connections.system())
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(player_jump.system());
    }
}

pub struct AddPlayerEvent(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut query: Query<(Entity, &Gamepad, With<Player>)>,
    mut ev_add_player: EventWriter<AddPlayerEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                ev_add_player.send(AddPlayerEvent(*id));
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // Despawn player associated with this gamepad
                for (player_entity, gamepad, _) in query.iter_mut() {
                    if gamepad == id {
                        commands.entity(player_entity).despawn()
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
    mut query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut Speed,
        &Gamepad,
        With<Player>,
    )>,
) {
    for (_, mut transform, speed, gamepad, _) in query.iter_mut() {
        let axis_lx = GamepadAxis(*gamepad, GamepadAxisType::LeftStickX);

        let x = if let Some(x) = axes.get(axis_lx) {
            x
        } else {
            return;
        };

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
    axes: Res<Axis<GamepadAxis>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut Speed,
        &Gamepad,
        With<Player>,
    )>,
) {
    // TODO: Way too nested, figure out how to break out of this
    for (_, transform, _, gamepad, _) in query.iter_mut() {
        let fire_button = GamepadButton(*gamepad, GamepadButtonType::RightTrigger2);

        if buttons.just_pressed(fire_button) {
            let axis_lx = GamepadAxis(*gamepad, GamepadAxisType::RightStickX);
            let axis_ly = GamepadAxis(*gamepad, GamepadAxisType::RightStickY);

            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                let right_stick_pos = Vec3::new(x, y, 0.);
                if right_stick_pos.length() > 0.1 {
                    commands
                        .spawn()
                        .insert_bundle(ProjectileBundle {
                            _p: Projectile,
                            sprite: SpriteBundle {
                                material: materials.add(asset_server.load(BULLET_SPRITE).into()),
                                transform: Transform {
                                    scale: Vec3::new(2., 2., 1.),
                                    translation: Vec3::new(
                                        transform.translation.x + right_stick_pos.x,
                                        transform.translation.y + right_stick_pos.y,
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
                            restitution: 0.,
                            density: 1., // Define the density. Higher value means heavier.
                            friction: 0., // Define the friction. Higher value means higher friction.
                        })
                        .insert(Velocity::from_linear(right_stick_pos * 1000.));
                }
            }
        }
    }
}

fn player_jump(
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(
        &mut Velocity,
        &mut AvailableJumps,
        &mut Transform,
        &mut Speed,
        &Gamepad,
        With<Player>,
    )>,
) {
    for (mut velocity, mut available_jumps, _, _, gamepad, _) in query.iter_mut() {
        let jump_button = GamepadButton(*gamepad, GamepadButtonType::South);
        if buttons.just_pressed(jump_button) && available_jumps.0 > 0 {
            velocity.linear = Vec3::Y * 400.;
            available_jumps.0 = available_jumps.0 - 1;
        }
    }
}
