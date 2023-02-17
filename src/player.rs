use std::ops::Add;

use bevy::prelude::*;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites).add_system(add_player).add_event::<AddPlayerEvent>();
    }
}

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
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(8., 8.),
        2,
        1,
        Some(Vec2::new(0., 0.)),
        Some(Vec2::new(0., 0.)),
    );
    commands.insert_resource(PlayerMaterials {
        player: texture_atlases.add(player_texture_atlas),
    });
}

pub struct AddPlayerEvent(pub Gamepad);

fn add_player(
    mut commands: Commands,
    mut ev_add_player: EventReader<AddPlayerEvent>,
    player_materials: Res<PlayerMaterials>,
) {
    for _event in ev_add_player.iter() {
        commands.spawn(PlayerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: player_materials.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(2., 2.0, 1.),
                    ..default()
                },
                ..default()
            }
        });
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    // pub gamepad: Gamepad,
    // pub available_jumps: AvailableJumps,
    // pub lives: Lives,
    // pub damage_taken: DamageTaken,
    // pub speed: Speed,
    // pub _p: Player,

    pub sprite: SpriteSheetBundle,
    // pub body: RigidBody,
    // pub shape: CollisionShape,
    // pub velocity: Velocity,
    // pub rotation_constraints: RotationConstraints,
}
