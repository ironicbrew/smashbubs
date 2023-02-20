use super::player::*;
use bevy::ecs::bundle::Bundle;
use bevy::{prelude::*, sprite::collide_aabb::*};

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
        app.add_system(clean_up_offscreen_projectiles);
        // .add_system(projectile_hit_player.system())
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

// fn projectile_hit_player(
//     mut commands: Commands,
//     mut projectile_query: Query<(Entity, &Transform, &Sprite, With<Projectile>)>,
//     mut player_query: Query<(&Transform, &Sprite, &mut DamageTaken, &mut Velocity, With<Player>)>,
// ) {
//     for (projectile_entity, projectile_transform, projectile_sprite, _) in
//         projectile_query.iter_mut()
//     {
//         for (player_transform, player_sprite, mut damage_taken, mut velocity, _) in player_query.iter_mut() {
//             let collision = collide(
//                 projectile_transform.translation,
//                 projectile_sprite.size * Vec2::from(projectile_transform.scale * 2.),
//                 player_transform.translation,
//                 player_sprite.size * Vec2::from(player_transform.scale),
//             );

//             if let Some(collision) = collision {
//                 damage_taken.0 = damage_taken.0 + 1.;

//                 match collision {
//                     Collision::Top => {
//                         velocity.linear = Vec3::Y * -(damage_taken.0 * damage_taken.0);
//                     },
//                     Collision::Bottom => {
//                         velocity.linear = Vec3::Y * damage_taken.0 * damage_taken.0;
//                     },
//                     Collision::Left => {
//                         velocity.linear = Vec3::X * damage_taken.0 * damage_taken.0;
//                     },
//                     Collision::Right => {
//                         velocity.linear = Vec3::X * -(damage_taken.0 * damage_taken.0);
//                     }

//                 }

//                 commands.entity(projectile_entity).despawn();
//             }
//         }
//     }
// }

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
