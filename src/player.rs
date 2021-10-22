use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use heron::prelude::*;

const PLAYER_SPRITE: &str = "player.png";

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_player.system());
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub damage_taken: DamageTaken,
    pub speed: Speed,
    pub _p: Player,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct Player;
pub struct PlayerName(pub String);
pub struct DamageTaken(pub u32);

pub struct Speed(pub f32);

fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert_bundle(PlayerBundle {
            name: PlayerName("Rob".to_string()),
            damage_taken: DamageTaken(0),
            _p: Player,
            speed: Speed(1.),
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
                transform: Transform {
                    scale: Vec3::new(2., 2., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(8., 8., 1.),
            border_radius: Some(0.),
        })
        .insert(Velocity::from_linear(Vec3::Y * 0.))
        .insert(RotationConstraints::lock());
}
