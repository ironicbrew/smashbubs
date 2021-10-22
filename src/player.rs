use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use heron::prelude::*;
use rand::seq::SliceRandom;

const PLAYER_SPRITE: &str = "player.png";
const BAT_SPRITE: &str = "bat.png";
const BLOCKY_SPRITE: &str = "blocky.png";
const BLUE_RING_SPRITE: &str = "blue_ring.png";
const CRABTOPUS_SPRITE: &str = "crabtopus.png";
const IRON_SPRITE: &str = "iron.png";
const PERL_SPRITE: &str = "perl.png";
const PIG_SPRITE: &str = "pig.png";
const RAT_SPRITE: &str = "rat.png";
const SLUG_SPRITE: &str = "slug.png";
const TURTLE_SPRITE: &str = "turtle.png";

const AVAILABLE_PLAYER_SPRITES: [&str; 11] = [
    PLAYER_SPRITE,
    BAT_SPRITE,
    BLOCKY_SPRITE,
    BLUE_RING_SPRITE,
    CRABTOPUS_SPRITE,
    IRON_SPRITE,
    PERL_SPRITE,
    PIG_SPRITE,
    RAT_SPRITE,
    SLUG_SPRITE,
    TURTLE_SPRITE,
];

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
    // const randomSprite: Option<&&str> = AVAILABLE_PLAYER_SPRITES.choose(&mut rand::thread_rng());

    let sprite: &str = if let Some(sp) = AVAILABLE_PLAYER_SPRITES.choose(&mut rand::thread_rng()) {
        sp
    } else {
        return;
    };

    commands
        .spawn()
        .insert_bundle(PlayerBundle {
            name: PlayerName("Rob".to_string()),
            damage_taken: DamageTaken(0),
            _p: Player,
            speed: Speed(1.),
            sprite: SpriteBundle {
                material: materials.add(asset_server.load(sprite).into()),
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
