use bevy::prelude::*;
use heron::prelude::*;
const BLOCK_SPRITE: &str = "block.png";

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(add_block.system());
    }
}

fn add_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
            transform: Transform {
                translation: Vec3::new(1., -100., 1.),
                scale: Vec3::new(24., 24., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(96., 96., 1.),
            border_radius: Some(0.),
        });
}