use bevy::prelude::*;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites);
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
