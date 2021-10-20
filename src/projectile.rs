use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteBundle;

// TODO: This needs to be a plugin

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub _p: Projectile,

    #[bundle]
    pub sprite: SpriteBundle
}

pub struct Projectile;