use super::player::*;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub _p: Projectile,
    pub lives: Lives,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            _p: Projectile,
            lives: Lives(4),
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
    mut projectile_query: Query<(Entity, &Projectile, &mut Lives)>,
    mut player_query: Query<(Entity, &PlayerIndex), With<Player>>,
    mut ev_player_damage: EventWriter<PlayerDamageEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                for (player_entity, player_index) in
                    player_query.iter_mut()
                {
                    for (projectile_entity, _p, mut lives) in projectile_query.iter_mut() {
                        if player_entity == *entity1 && projectile_entity == *entity2 {
                            ev_player_damage.send(PlayerDamageEvent(
                                player_index.clone(),
                                DamageTaken(10.),
                            ));
                        } else if player_entity == *entity2 && projectile_entity == *entity1 {
                            ev_player_damage.send(PlayerDamageEvent(
                                player_index.clone(),
                                DamageTaken(10.),
                            ));
                        }

                        if lives.0 == 0 {
                            if projectile_entity == *entity1 {
                                commands.entity(*entity1).despawn();
                            } else if projectile_entity == *entity2 {
                                commands.entity(*entity2).despawn();
                            }
                        } else {
                            lives.0 = lives.0 - 1;
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
