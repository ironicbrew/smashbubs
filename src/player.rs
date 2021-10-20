use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteBundle;

// TODO: This needs to be a plugin

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub damage_taken: DamageTaken,
    pub speed: Speed,
    pub _p: Player,

    #[bundle]
    pub sprite: SpriteBundle
}

pub struct Player;
pub struct PlayerName(pub String);
pub struct DamageTaken(pub u32);

#[derive(Debug)]
pub struct Speed(pub f32);