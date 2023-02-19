use bevy::{math::Vec3, prelude::*};
use bevy_rapier2d::{na::Rotation, prelude::*};

use crate::gamepad::AddPlayerEvent;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites)
            .add_system(add_player)
            .add_system(reset_jumps)
            .add_system(respawn_players_who_leave_window)
            .add_event::<AddPlayerEvent>();
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct PlayerMaterials {
    player: Handle<TextureAtlas>,
}

fn setup_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PIG_SPRITE);
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(8., 8.), 2, 1, None, None);
    commands.insert_resource(PlayerMaterials {
        player: texture_atlases.add(player_texture_atlas),
    });
}

fn add_player(
    mut commands: Commands,
    mut ev_add_player: EventReader<AddPlayerEvent>,
    player_materials: Res<PlayerMaterials>,
) {
    for event in ev_add_player.iter() {
        commands.spawn(PlayerBundle {
            gamepad: PlayerGamepad(event.0),
            sprite: SpriteSheetBundle {
                texture_atlas: player_materials.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(2., 2.0, 1.),
                    ..default()
                },
                ..default()
            },
            ..default()
        });

        commands
            .spawn(Collider::cuboid(500.0, 50.0))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS);

        /* Create the bouncing ball. */
        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Restitution::coefficient(0.7))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

#[derive(Component)]
pub struct PlayerGamepad(pub Gamepad);

#[derive(Component)]
pub struct AvailableJumps(pub u32);
#[derive(Component)]
pub struct Lives(u32);
#[derive(Component)]
pub struct DamageTaken(u32);

#[derive(Component)]
pub struct PlayerSpriteSheet(pub SpriteSheetBundle);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub gamepad: PlayerGamepad,
    available_jumps: AvailableJumps,
    lives: Lives,
    damage_taken: DamageTaken,
    speed: Speed,
    _p: Player,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    velocity: Velocity,
    active_collision_types: ActiveCollisionTypes,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            gamepad: PlayerGamepad(Gamepad { id: 1 }),
            damage_taken: DamageTaken(0),
            available_jumps: AvailableJumps(2),
            lives: Lives(2),
            _p: Player,
            speed: Speed(1.),
            sprite: SpriteSheetBundle {
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(4., 4.),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
            active_collision_types: ActiveCollisionTypes::default(),
        }
    }
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    windows: ResMut<Windows>,
    mut query: Query<(Entity, &mut Transform, &mut Lives, &mut DamageTaken)>,
) {
    if let Some(window) = windows.iter().next() {
        for (player_entity, mut transform, mut lives, mut damage_taken) in query.iter_mut() {
            if transform.translation.y.abs() > window.height() / 2.
                || transform.translation.x.abs() > window.width() / 2.
            {
                lives.0 = lives.0 - 1;
                damage_taken.0 = 0;

                if lives.0 == 0 {
                    commands.entity(player_entity).despawn();
                } else {
                    transform.translation = Vec3::new(0., 0., 1.);
                }
            }
        }
    }
}

fn reset_jumps(
    mut events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut AvailableJumps)>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                for (player_entity, mut available_jumps) in player_query.iter_mut() {
                    if *collider1 == player_entity || *collider2 == player_entity {
                        available_jumps.0 = 2
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
