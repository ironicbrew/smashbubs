use super::player::*;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

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
            if translation.y.abs() > window.height() / 2.
                || translation.x.abs() > window.width() / 2.
            {
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

fn projectile_collided(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile)>,
    mut player_query: Query<(Entity, &mut DamageTaken, &mut Health), With<Player>>,
    mut ev_player_damage: EventWriter<PlayerDamageEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {

                for (player_entity, mut damage_taken, mut health) in player_query.iter_mut() {
                    for (projectile_entity, _p) in projectile_query.iter() {
                        if player_entity == *entity1 && projectile_entity == *entity2 {
                            print!("hit1");
                            damage_taken.0 = damage_taken.0 + 10.;
                            health.0 = health.0 - 10;
                            ev_player_damage.send(PlayerDamageEvent(DamageTaken(damage_taken.0)));
                        } else if player_entity == *entity2 && projectile_entity == *entity1 {
                            print!("hit1");
                            damage_taken.0 = damage_taken.0 + 10.;
                            health.0 = health.0 - 10;
                            ev_player_damage.send(PlayerDamageEvent(DamageTaken(damage_taken.0)));
                        }
                        if projectile_entity == *entity1 {
                            commands.entity(*entity1).despawn();
                        } else if projectile_entity == *entity2 {
                            commands.entity(*entity2).despawn();
                        }
                    }
                }

                // fn query_contains_entity(query: &Query<Entity, With<Projectile>>, entity: &Entity) -> bool {
                //     for query_entity in query.iter() {
                //         if query_entity == *entity {
                //             println!("{:?}, {:?}", query_entity, *entity);
                //             return true;
                //         }
                //     }
                //     false
                // }
            }
            _ => {}
        }
    }
}
