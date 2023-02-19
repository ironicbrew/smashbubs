use std::ops::Add;

use bevy::{ecs::component, math::Vec3, prelude::*};
use rapier2d::prelude::{RigidBody, RigidBodyBuilder};

use crate::gamepad::AddPlayerEvent;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites)
            .add_system(add_player)
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
    }
}

#[derive(Component)]
pub struct PlayerGamepad(pub Gamepad);

#[derive(Component)]
struct AvailableJumps(u32);
#[derive(Component)]
struct Lives(u32);
#[derive(Component)]
struct DamageTaken(u32);

#[derive(Component)]
pub struct PlayerSpriteSheet(pub SpriteSheetBundle);

#[derive(Component)]
pub struct PlayerPhysics {
    pub rigid_body: RigidBody,
    // pub collision_shape: CollisionShape,
    // pub velocity: Velocity,
    // pub rotation_constraints: RotationConstraints,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub gamepad: PlayerGamepad,
    available_jumps: AvailableJumps,
    lives: Lives,
    damage_taken: DamageTaken,
    speed: Speed,
    _p: Player,

    #[bundle]
    pub sprite: SpriteSheetBundle,

    pub player_physics: PlayerPhysics,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            gamepad: PlayerGamepad(Gamepad {id: 1}),
            damage_taken: DamageTaken(0),
            available_jumps: AvailableJumps(2),
            lives: Lives(2),
            _p: Player,
            speed: Speed(1.),
            sprite: SpriteSheetBundle {
                ..Default::default()
            },
            player_physics: PlayerPhysics {
                rigid_body: RigidBodyBuilder::dynamic().build(),
                // collision_shape: CollisionShape::Cuboid {
                //     half_extends: Vec3::new(8., 8., 1.),
                //     border_radius: Some(0.),
                // },
                // velocity: Velocity::from_linear(Vec3::Y * 100.),
                // rotation_constraints: RotationConstraints::lock(),
            },
        }
    }
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Lives,
        &mut DamageTaken,
        &mut PlayerPhysics,
    )>,
) {
    if let Some(window) = windows.iter().next() {
        for (player_entity, mut transform, mut lives, mut damage_taken, mut player_physics) in
            query.iter_mut()
        {
            if transform.translation.y.abs() > window.height() / 2.
                || transform.translation.x.abs() > window.width() / 2.
            {
                lives.0 = lives.0 - 1;
                damage_taken.0 = 0;

                if lives.0 == 0 {
                    commands.entity(player_entity).despawn();
                } else {
                    transform.translation = Vec3::new(0., 0., 1.);
                    // player_physics.velocity.linear = Vec3::Y * 100.;
                }
            }
        }
    }
}
