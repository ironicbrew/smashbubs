use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub damage_taken: DamageTaken,
    pub _p: Player,

    #[bundle]
    pub sprite: SpriteBundle
}

pub struct Player;
pub struct PlayerName(pub String);
pub struct DamageTaken(pub u32);