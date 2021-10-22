use super::gamepad::*;
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
        app.add_system(add_player.system())
            .add_system(respawn_players_who_leave_window.system());
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub gamepad: Gamepad,
    pub lives: Lives,
    pub damage_taken: DamageTaken,
    pub speed: Speed,
    pub _p: Player,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct Player;
pub struct PlayerName(pub String);
pub struct DamageTaken(pub u32);
pub struct Lives(pub i8);

pub struct Speed(pub f32);

pub fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ev_add_player: EventReader<AddPlayerEvent>,
) {
    let sprite: &str =
        if let Some(sprite) = AVAILABLE_PLAYER_SPRITES.choose(&mut rand::thread_rng()) {
            sprite
        } else {
            return;
        };

    for event in ev_add_player.iter() {
        commands
            .spawn()
            .insert_bundle(PlayerBundle {
                name: PlayerName("Rob".to_string()),
                gamepad: event.0,
                damage_taken: DamageTaken(0),
                lives: Lives(2),
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
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    mut query: Query<(Entity, &mut Transform, &mut Lives, With<Player>)>,
) {
    for (player_entity, mut transform, mut lives, _) in query.iter_mut() {
        if transform.translation.y.abs() > window.height / 2.
            || transform.translation.x.abs() > window.width / 2.
        {
            lives.0 = lives.0 - 1;

            if lives.0 == 0 {
                commands.entity(player_entity).despawn();
            } else {
                transform.translation = Vec3::new(0., 0., 0.);
            }
        }
    }
}
