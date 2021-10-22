use bevy::prelude::*;
use heron::prelude::*;
const BLOCK_SPRITE: &str = "block.png";

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_block.system());
    }
}

#[derive(Bundle)]
pub struct MapBundle {
    _m: Map,

    #[bundle]
    sprite: SpriteBundle,
}

pub struct Map;

fn add_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Bottom Platform
    commands
        .spawn()
        .insert_bundle(MapBundle {
            _m: Map,
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
                transform: Transform {
                    translation: Vec3::new(1., -100., 1.),
                    scale: Vec3::new(50., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(192., 4., 1.),
            border_radius: Some(0.),
        });

    // Left Platform
    commands
        .spawn()
        .insert_bundle(MapBundle {
            _m: Map,
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
                transform: Transform {
                    translation: Vec3::new(-110., 0., 1.),
                    scale: Vec3::new(16., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(60., 4., 1.),
            border_radius: Some(0.),
        });

    // Right Platform
    commands
        .spawn()
        .insert_bundle(MapBundle {
            _m: Map,
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
                transform: Transform {
                    translation: Vec3::new(110., 0., 1.),
                    scale: Vec3::new(16., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(60., 4., 1.),
            border_radius: Some(0.),
        });

    // Top Platform
    commands
        .spawn()
        .insert_bundle(MapBundle {
            _m: Map,
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(BLOCK_SPRITE).into()),
                transform: Transform {
                    translation: Vec3::new(0., 100., 1.),
                    scale: Vec3::new(16., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(60., 4., 1.),
            border_radius: Some(0.),
        });
}
