use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub _p: Projectile,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct Projectile;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(clean_up_offscreen_projectiles.system());
    }
}

fn clean_up_offscreen_projectiles(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    mut query: Query<(Entity, &Transform, With<Projectile>)>,
) {
    for (projectile_entity, transform, _) in query.iter_mut() {
        let translation = transform.translation;
        if translation.y.abs() > window.height / 2. || translation.x.abs() > window.width / 2. {
            commands.entity(projectile_entity).despawn();
        }
    }
}
