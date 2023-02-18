use std::ops::Add;

use bevy::prelude::*;

use crate::gamepad::AddPlayerEvent;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites).add_system(add_player).add_event::<AddPlayerEvent>();
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
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(8., 8.),
        2,
        1,
        None,
        None,
    );
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
            available_jumps: AvailableJumps(2),
            lives: Lives(3),
            damage_taken: DamageTaken(0),
            speed: Speed(2.),
            _p: Player
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
    // pub body: RigidBody,
    // pub shape: CollisionShape,
    // pub velocity: Velocity,
    // pub rotation_constraints: RotationConstraints,
}
