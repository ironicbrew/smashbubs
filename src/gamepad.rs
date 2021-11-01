use gilrs::ff::Effect;
use std::time::Duration;
use gilrs::GamepadId;
use gilrs::Gilrs;
use super::player::*;
use super::projectile::*;
use bevy::prelude::*;
use heron::prelude::*;
use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Replay, Ticks};
use std::thread;

pub const TIME_STEP: f32 = 3.;
const BULLET_SPRITE: &str = "bullet.png";

pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddPlayerEvent>()
            .insert_resource(RumbleTimer(Timer::from_seconds(0., false)))
            .add_system(gamepad_connections.system())
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(player_jump.system())
            .add_system(stop_rumbler.system());
    }
}

pub struct AddPlayerEvent(pub Gamepad);
pub struct RumbleTimer(Timer);

fn gamepad_connections(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut query: Query<(Entity, &Gamepad, With<Player>)>,
    mut ev_add_player: EventWriter<AddPlayerEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                ev_add_player.send(AddPlayerEvent(*id));
            }
            GamepadEventType::Disconnected => {
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
        &mut TextureAtlasSprite,
        &mut Transform,
        &mut Speed,
        &Gamepad,
        With<Player>,
    )>,
) {
    for (mut sprite, mut transform, speed, gamepad, _) in query.iter_mut() {
        let axis_lx = GamepadAxis(*gamepad, GamepadAxisType::LeftStickX);

        let x = if let Some(x) = axes.get(axis_lx) {
            x
        } else {
            return;
        };

        transform.translation.x += x * TIME_STEP;

        if x != 0. {
            face_player_last_direction_moved(sprite, speed.0, transform);
            change_player_direction(speed, x);
        }

        fn face_player_last_direction_moved(
            mut sprite: Mut<TextureAtlasSprite>,
            speed: f32,
            mut transform: Mut<Transform>,
        ) {
            if speed > 0. {
                sprite.index = 0;
            } else {
                sprite.index = 1;
            }
        }

        fn change_player_direction(mut speed: Mut<Speed>, direction: f32) {
            speed.0 = direction;
        }
    }
}

pub fn rumble(gamepad_ids: &[GamepadId]) {
    let mut gilrs = Gilrs::new().unwrap();

    let duration = Ticks::from_ms(1000);
    let effect = EffectBuilder::new()
    .add_effect(BaseEffect {
        kind: BaseEffectType::Strong { magnitude: 60_000 },
        scheduling: Replay {
            play_for: duration,
            ..Default::default()
        },
        envelope: Default::default(),
    })
    .gamepads(gamepad_ids)
    .finish(&mut gilrs)
    .unwrap();
    effect.play().unwrap();
    println!("rumbling over");
    thread::sleep(Duration::from_secs(1));
    effect.stop().unwrap();
}

fn stop_rumbler(mut rumble_timer: ResMut<RumbleTimer>, time: Res<Time>, mut rumble: NonSendMut<Effect>) {

    if rumble_timer.0.tick(time.delta()).just_finished() {
        rumble.stop().unwrap();
    }

}

fn player_fire(
    mut commands: Commands,
    mut gilrs: NonSendMut<Gilrs>,
    mut rumble: NonSendMut<Effect>,
    axes: Res<Axis<GamepadAxis>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut Speed,
        &Gamepad,
        With<Player>,
    )>,
) {

    // TODO: Way too nested, figure out how to break out of this (closure in rust?)
    for (_, transform, _, gamepad, _) in query.iter_mut() {
        let fire_button = GamepadButton(*gamepad, GamepadButtonType::RightTrigger2);

        if buttons.just_pressed(fire_button) {
            let axis_lx = GamepadAxis(*gamepad, GamepadAxisType::RightStickX);
            let axis_ly = GamepadAxis(*gamepad, GamepadAxisType::RightStickY);


            // let mut gilrs = Gilrs::new().unwrap();

            // let mut gamepads = Vec::new();

            // for (_id, gamepad) in gilrs.gamepads() {
            //     gamepads.push(_id);
            // }

            let test: Vec<GamepadId> = gilrs.gamepads().map(|(_id, _)| _id).collect();

            rumble.play().unwrap();

            let rumble_timer = RumbleTimer(Timer::from_seconds(0.2, false));

            commands.insert_resource(rumble_timer);


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
