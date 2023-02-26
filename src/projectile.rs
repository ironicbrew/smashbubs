use super::player::*;
use bevy::ecs::bundle::Bundle;
use bevy::{prelude::*, sprite::collide_aabb::*};
use bevy_rapier2d::prelude::{Collider, Velocity, CollisionEvent, ContactForceEvent};

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
        .add_system(projectile_hit_player)
        .add_system(projectile_collided);
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
    atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    mut projectile_query: Query<(Entity, &Transform, &Handle<Image>), With<Projectile>>,
    mut player_query: Query<(&Transform, &Handle<TextureAtlas>, &mut DamageTaken, &mut Health, &mut Velocity, With<Player>)>,
    mut ev_player_damage: EventWriter<PlayerDamageEvent>
) {
    for (projectile_entity, projectile_transform, projectile_image) in
        projectile_query.iter_mut()
    {
        for (player_transform, player_sprite, mut damage_taken, mut health, mut velocity, _) in player_query.iter_mut() {
            let collision = collide(
                projectile_transform.translation,
                images.get(projectile_image).unwrap().size(),
                player_transform.translation,
                atlases.get(player_sprite).unwrap().size,
            );

            if let Some(collision) = collision {
                damage_taken.0 = damage_taken.0 + 10.;
                health.0 = health.0 - 10;

                ev_player_damage.send(PlayerDamageEvent(DamageTaken(damage_taken.0)));

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

fn projectile_collided(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut projectile_query: Query<Entity, With<Projectile>>
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1 , entity2, _) => {
                let projectile_entity = projectile_query.iter_mut().next();
                 if projectile_entity == Some(*entity1) {
                     commands.entity(*entity1).despawn();

                 } else if projectile_entity == Some(*entity2) {
                    commands.entity(*entity2).despawn();
                 }
            },
            _ => {}
        }
    }
}