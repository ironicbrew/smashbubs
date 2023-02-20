use super::player::*;
use bevy::ecs::bundle::Bundle;
use bevy::{prelude::*, sprite::collide_aabb::*};
use bevy_rapier2d::prelude::{Collider, Velocity};

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub _p: Projectile,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            _p: Projectile,
            sprite: SpriteBundle::default(),
        }
    }
}
#[derive(Component)]
pub struct Projectile;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clean_up_offscreen_projectiles)
        .add_system(projectile_hit_player);
        // .add_system(projectile_hit_map.system());
    }
}

fn clean_up_offscreen_projectiles(
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(Entity, &Transform, With<Projectile>)>,
) {
    if let Some(window) = windows.iter().next() {
        for (projectile_entity, transform, _) in query.iter_mut() {
            let translation = transform.translation;
            if translation.y.abs() > window.height() / 2. || translation.x.abs() > window.width() / 2. {
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

fn projectile_hit_player(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut player_query: Query<(&Transform, &Collider, &mut DamageTaken, &mut Velocity, With<Player>)>,
) {
    for (projectile_entity, projectile_transform) in
        projectile_query.iter_mut()
    {
        for (player_transform, player_sprite, mut damage_taken, mut velocity, _) in player_query.iter_mut() {
            let collision = collide(
                projectile_transform.translation,
                player_sprite.scale(),
                player_transform.translation,
                player_sprite.scale(),
            );

            if let Some(collision) = collision {
                damage_taken.0 = damage_taken.0 + 1.;

                match collision {
                    Collision::Top => {
                        velocity.linvel = Vec2::Y * Vec2 {y: -(damage_taken.0 * damage_taken.0), ..default()}
                    },
                    Collision::Bottom => {
                        velocity.linvel = Vec2::Y * Vec2 {y:damage_taken.0 * damage_taken.0, ..default()}
                    },
                    Collision::Left => {
                        velocity.linvel = Vec2::X * Vec2 {x: damage_taken.0 * damage_taken.0, ..default()}
                    },
                    Collision::Right => {
                        velocity.linvel = Vec2::X * Vec2 {x: -(damage_taken.0 * damage_taken.0), ..default()}
                    },
                    _ => ()

                }

                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

// fn projectile_hit_map(
//     mut commands: Commands,
//     mut projectile_query: Query<(Entity, &Transform, &Sprite, With<Projectile>)>,
//     mut map_query: Query<(&Transform, &Sprite, With<Map>)>,
// ) {
//     for (projectile_entity, projectile_transform, projectile_sprite, _) in
//         projectile_query.iter_mut()
//     {
//         for (map_transform, map_sprite, _) in map_query.iter_mut() {
//             let collision = collide(
//                 projectile_transform.translation,
//                 projectile_sprite.size * Vec2::from(projectile_transform.scale),
//                 map_transform.translation,
//                 map_sprite.size * Vec2::from(map_transform.scale),
//             );

//             if let Some(_) = collision {
//                 commands.entity(projectile_entity).despawn();
//             }
//         }
//     }
// }
