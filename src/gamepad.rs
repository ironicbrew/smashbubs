use crate::player::*;
use crate::projectile::ProjectileBundle;
use bevy_rapier2d::prelude::*;
// use super::projectile::*;
use bevy::prelude::*;

pub const TIME_STEP: f32 = 3.;
const BULLET_SPRITE: &str = "bullet.png";

pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddPlayerEvent>()
            // .insert_resource(RumbleTimer(Timer::from_seconds(0., false)))
            .add_system(gamepad_connections)
            .add_system(player_movement)
            .add_system(player_fire)
            .add_system(player_jump);
        // .add_system(stop_rumbler.system());
    }
}

pub struct AddPlayerEvent(pub Gamepad);
// pub struct RumbleTimer(Timer);

fn gamepad_connections(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut query: Query<(Entity, &PlayerGamepad)>,
    mut ev_add_player: EventWriter<AddPlayerEvent>,
) {
    for GamepadEvent {
        gamepad,
        event_type,
    } in gamepad_evr.iter()
    {
        match event_type {
            GamepadEventType::Connected(_) => {
                ev_add_player.send(AddPlayerEvent(*gamepad));
            }
            GamepadEventType::Disconnected => {
                // Despawn player associated with this gamepad
                for (player_entity, player_gamepad) in query.iter_mut() {
                    if player_gamepad.0 == *gamepad {
                        commands.entity(player_entity).despawn()
                    }
                }
            }
            _ => {}
        }
    }
}

fn player_movement(
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &mut Transform,
            &mut Speed,
            &PlayerGamepad,
        ),
        With<Player>,
    >,
) {
    for (sprite, mut transform, speed, player_gamepad) in query.iter_mut() {
        let axis_lx = GamepadAxis {
            gamepad: player_gamepad.0,
            axis_type: GamepadAxisType::LeftStickX,
        };

        let x = if let Some(x) = axes.get(axis_lx) {
            x
        } else {
            return;
        };

        transform.translation.x += x * TIME_STEP;

        if x != 0. {
            face_player_last_direction_moved(sprite, speed.0);
            change_player_direction(speed, x);
        }

        fn face_player_last_direction_moved(
            mut player_sprite: Mut<TextureAtlasSprite>,
            speed: f32,
        ) {
            if speed > 0. {
                player_sprite.flip_x = false
            } else {
                player_sprite.flip_x = true
            }
        }

        fn change_player_direction(mut speed: Mut<Speed>, direction: f32) {
            speed.0 = direction;
        }
    }
}

// pub fn rumble(gamepad_ids: &[GamepadId]) {
//     let mut gilrs = Gilrs::new().unwrap();

//     let duration = Ticks::from_ms(1000);
//     let effect = EffectBuilder::new()
//     .add_effect(BaseEffect {
//         kind: BaseEffectType::Strong { magnitude: 60_000 },
//         scheduling: Replay {
//             play_for: duration,
//             ..Default::default()
//         },
//         envelope: Default::default(),
//     })
//     .gamepads(gamepad_ids)
//     .finish(&mut gilrs)
//     .unwrap();
//     effect.play().unwrap();
//     println!("rumbling over");
//     thread::sleep(Duration::from_secs(1));
//     effect.stop().unwrap();
// }

// fn stop_rumbler(mut rumble_timer: ResMut<RumbleTimer>, time: Res<Time>, mut rumble: NonSendMut<Effect>) {

//     if rumble_timer.0.tick(time.delta()).just_finished() {
//         rumble.stop().unwrap();
//     }

// }

fn player_fire(
    mut commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(&mut Transform, &PlayerGamepad)>,
    audio: Res<Audio>,
) {
    for (transform, gamepad) in query.iter_mut() {
        let fire_button = GamepadButton {
            gamepad: gamepad.0,
            button_type: GamepadButtonType::RightTrigger2,
        };

        if buttons.just_pressed(fire_button) {
            let axis_lx = GamepadAxis {
                gamepad: gamepad.0,
                axis_type: GamepadAxisType::RightStickX,
            };
            let axis_ly = GamepadAxis {
                gamepad: gamepad.0,
                axis_type: GamepadAxisType::RightStickY,
            };

            // let mut gilrs = Gilrs::new().unwrap();

            // let mut gamepads = Vec::new();

            // for (_id, gamepad) in gilrs.gamepads() {
            //     gamepads.push(_id);
            // }

            // let test: Vec<GamepadId> = gilrs.gamepads().map(|(_id, _)| _id).collect();

            // rumble.play().unwrap();

            // let rumble_timer = RumbleTimer(Timer::from_seconds(0.2, false));

            // commands.insert_resource(rumble_timer);

            //         let effect = EffectBuilder::new()
            //         .add_effect(BaseEffect {
            //             kind: BaseEffectType::Strong { magnitude: 60_000 },
            //             scheduling: Replay {
            //                 play_for: Ticks::from_ms(200),
            //                 ..Default::default()
            //             },
            //             envelope: Default::default(),
            //         })
            //         .gamepads(&[])
            //         .finish(&mut gilrs)
            //         .unwrap();
            //         effect.play().unwrap();

            // thread::sleep(Duration::from_millis(100));
            // effect.stop().unwrap();

            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                let right_stick_pos = Vec2::new(x, y);
                if right_stick_pos.length() > 0.5 {
                    commands
                        .spawn(ProjectileBundle {
                            sprite: SpriteBundle {
                                texture: asset_server.load(BULLET_SPRITE),
                                transform: Transform {
                                    scale: Vec3::new(1., 1., 1.),
                                    translation: Vec3::new(
                                        transform.translation.x + (right_stick_pos.x * 20.),
                                        transform.translation.y + (right_stick_pos.y * 20.),
                                        0.,
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::cuboid(1., 1.))
                        .insert(Friction::coefficient(0.05))
                        .insert(Restitution::coefficient(0.5))
                        .insert(AdditionalMassProperties::Mass(5.))
                        .insert(Velocity::linear(right_stick_pos * 1000.))
                        .insert(Ccd::enabled());
                    let gunsound = asset_server.load("sounds/gun.ogg");
                    audio.play(gunsound);
                }
            }
        }
    }
}

fn player_jump(
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(&mut AvailableJumps, &mut Velocity, &PlayerGamepad)>,
) {
    for (mut available_jumps, mut velocity, gamepad) in query.iter_mut() {
        let jump_button = GamepadButton {
            gamepad: gamepad.0,
            button_type: GamepadButtonType::South,
        };
        if buttons.just_pressed(jump_button) && available_jumps.0 > 0 {
            available_jumps.0 = available_jumps.0 - 1;
            velocity.linvel = Vec2::Y * 400.
        }
    }
}
