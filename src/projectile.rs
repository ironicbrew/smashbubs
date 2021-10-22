use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub _p: Projectile,

    #[bundle]
    pub sprite: SpriteBundle
}

pub struct Projectile;