use bevy::ecs::world::WorldBorrowMut;
use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Replay, Ticks};
use bevy::input::gamepad::GamepadEventRaw;
use bevy::app::Events;
use bevy::prelude::*;
use bevy::prelude::{Gamepad};
use gilrs::*;
use std::thread;
use std::time::Duration;

pub fn convert_gamepad_id(gamepad_id: gilrs::GamepadId) -> Gamepad {
    Gamepad(gamepad_id.into())
}

pub fn convert_button(button: gilrs::Button) -> Option<GamepadButtonType> {
    match button {
        gilrs::Button::South => Some(GamepadButtonType::South),
        gilrs::Button::East => Some(GamepadButtonType::East),
        gilrs::Button::North => Some(GamepadButtonType::North),
        gilrs::Button::West => Some(GamepadButtonType::West),
        gilrs::Button::C => Some(GamepadButtonType::C),
        gilrs::Button::Z => Some(GamepadButtonType::Z),
        gilrs::Button::LeftTrigger => Some(GamepadButtonType::LeftTrigger),
        gilrs::Button::LeftTrigger2 => Some(GamepadButtonType::LeftTrigger2),
        gilrs::Button::RightTrigger => Some(GamepadButtonType::RightTrigger),
        gilrs::Button::RightTrigger2 => Some(GamepadButtonType::RightTrigger2),
        gilrs::Button::Select => Some(GamepadButtonType::Select),
        gilrs::Button::Start => Some(GamepadButtonType::Start),
        gilrs::Button::Mode => Some(GamepadButtonType::Mode),
        gilrs::Button::LeftThumb => Some(GamepadButtonType::LeftThumb),
        gilrs::Button::RightThumb => Some(GamepadButtonType::RightThumb),
        gilrs::Button::DPadUp => Some(GamepadButtonType::DPadUp),
        gilrs::Button::DPadDown => Some(GamepadButtonType::DPadDown),
        gilrs::Button::DPadLeft => Some(GamepadButtonType::DPadLeft),
        gilrs::Button::DPadRight => Some(GamepadButtonType::DPadRight),
        gilrs::Button::Unknown => None,
    }
}

pub fn convert_axis(axis: gilrs::Axis) -> Option<GamepadAxisType> {
    match axis {
        gilrs::Axis::LeftStickX => Some(GamepadAxisType::LeftStickX),
        gilrs::Axis::LeftStickY => Some(GamepadAxisType::LeftStickY),
        gilrs::Axis::LeftZ => Some(GamepadAxisType::LeftZ),
        gilrs::Axis::RightStickX => Some(GamepadAxisType::RightStickX),
        gilrs::Axis::RightStickY => Some(GamepadAxisType::RightStickY),
        gilrs::Axis::RightZ => Some(GamepadAxisType::RightZ),
        gilrs::Axis::DPadX => Some(GamepadAxisType::DPadX),
        gilrs::Axis::DPadY => Some(GamepadAxisType::DPadY),
        gilrs::Axis::Unknown => None,
    }
}

pub fn gilrs_event_startup_system(world: &mut World) {
    let world = world.cell();
    let gilrs = world.get_non_send::<Gilrs>().unwrap();
    let mut event = world.get_resource_mut::<Events<GamepadEventRaw>>().unwrap();
    for (id, _) in gilrs.gamepads() {
        event.send(GamepadEventRaw(
            convert_gamepad_id(id),
            GamepadEventType::Connected,
        ));
    }
}

pub fn rumble(gamepad_ids: &[GamepadId]) {
    let mut gilrs = Gilrs::new().unwrap();

    println!("rumbling");
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
    // thread::sleep(Duration::from_secs(1));
    // effect.stop().unwrap();
}

pub fn gilrs_event_system(world: &mut World) {
    let world = world.cell();
    let mut gilrs = world.get_non_send_mut::<Gilrs>().unwrap();
    let mut event = world.get_resource_mut::<Events<GamepadEventRaw>>().unwrap();

    event.update();
    while let Some(gilrs_event) = gilrs.next_event() {
        match gilrs_event.event {
            EventType::Connected => {
                event.send(GamepadEventRaw(
                    convert_gamepad_id(gilrs_event.id),
                    GamepadEventType::Connected,
                ));
            }
            EventType::Disconnected => {
                event.send(GamepadEventRaw(
                    convert_gamepad_id(gilrs_event.id),
                    GamepadEventType::Disconnected,
                ));
            }
            EventType::ButtonChanged(gilrs_button, value, _) => {
                if let Some(button_type) = convert_button(gilrs_button) {
                    event.send(GamepadEventRaw(
                        convert_gamepad_id(gilrs_event.id),
                        GamepadEventType::ButtonChanged(button_type, value),
                    ));
                    
                }
            }
            EventType::AxisChanged(gilrs_axis, value, _) => {
                if let Some(axis_type) = convert_axis(gilrs_axis) {
                    event.send(GamepadEventRaw(
                        convert_gamepad_id(gilrs_event.id),
                        GamepadEventType::AxisChanged(axis_type, value),
                    ));
                }
            }
            _ => (),
        };
    }
    gilrs.inc();
}

#[derive(Default)]
pub struct GilrsPlugin;

impl Plugin for GilrsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        match GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
        {
            Ok(mut gilrs) => {

                let test: Vec<GamepadId> = gilrs.gamepads().map(|(_id, _)| _id).collect();

                let rumble = EffectBuilder::new()
                .add_effect(BaseEffect {
                    kind: BaseEffectType::Strong { magnitude: 60_000 },
                    scheduling: Replay {
                        play_for: Ticks::from_ms(200),
                        ..Default::default()
                    },
                    envelope: Default::default(),
                })
                .gamepads(&test)
                .finish(&mut gilrs)
                .unwrap();

                app
                .insert_non_send_resource(gilrs)
                .insert_non_send_resource(rumble)
                    .add_startup_system_to_stage(
                        StartupStage::PreStartup,
                        gilrs_event_startup_system.exclusive_system(),
                    )
                    .add_system_to_stage(
                        CoreStage::PreUpdate,
                        gilrs_event_system.exclusive_system(),
                    );
            }
            Err(err) => error!("Failed to start Gilrs. {}", err),
        }

    }
}