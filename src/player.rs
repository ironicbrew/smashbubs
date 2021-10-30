use super::gamepad::*;
use super::map::*;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use heron::prelude::*;
// use rand::seq::SliceRandom;

// const PLAYER_SPRITE: &str = "player.png";
// const BAT_SPRITE: &str = "bat.png";
// const BLOCKY_SPRITE: &str = "blocky.png";
// const BLUE_RING_SPRITE: &str = "blue_ring.png";
// const CRABTOPUS_SPRITE: &str = "crabtopus.png";
// const IRON_SPRITE: &str = "iron.png";
// const PERL_SPRITE: &str = "perl.png";
const PIG_SPRITE: &str = "pig.png";
// const RAT_SPRITE: &str = "rat.png";
// const SLUG_SPRITE: &str = "slug.png";
// const TURTLE_SPRITE: &str = "turtle.png";

// const AVAILABLE_PLAYER_SPRITES: [&str; 11] = [
//     PLAYER_SPRITE,
//     BAT_SPRITE,
//     BLOCKY_SPRITE,
//     BLUE_RING_SPRITE,
//     CRABTOPUS_SPRITE,
//     IRON_SPRITE,
//     PERL_SPRITE,
//     PIG_SPRITE,
//     RAT_SPRITE,
//     SLUG_SPRITE,
//     TURTLE_SPRITE,
// ];

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_sprites.system())
        .add_system(add_player.system())
            .add_system(respawn_players_who_leave_window.system())
            .add_system(reset_jumps.system());
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub gamepad: Gamepad,
    pub available_jumps: AvailableJumps,
    pub lives: Lives,
    pub damage_taken: DamageTaken,
    pub speed: Speed,
    pub size: Vec2,
    pub _p: Player,

    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub body: RigidBody,
    pub shape: CollisionShape,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
}
pub struct Player;
pub struct PlayerName(pub String);
pub struct DamageTaken(pub f32);
pub struct Lives(pub i8);
pub struct AvailableJumps(pub i8);
pub struct Size(Vec2);

pub struct Speed(pub f32);

pub struct PlayerMaterials {
	player: Handle<TextureAtlas>,
}

pub fn setup_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PIG_SPRITE);
	let player_texture_atlas = TextureAtlas::from_grid(player_texture_handle, Vec2::new(8.0, 8.0), 2, 1);
    commands.insert_resource(PlayerMaterials {
        player: texture_atlases.add(player_texture_atlas),
	});

}

pub fn add_player(
    mut commands: Commands,
    mut ev_add_player: EventReader<AddPlayerEvent>,
    player_materials: Res<PlayerMaterials>
) {

    for event in ev_add_player.iter() {
        commands
            .spawn()
            .insert_bundle(PlayerBundle {
                name: PlayerName("Rob".to_string()),
                gamepad: event.0,
                damage_taken: DamageTaken(0.),
                available_jumps: AvailableJumps(2),
                lives: Lives(2),
                _p: Player,
                size: Vec2::new(8., 8.),
                speed: Speed(1.),
                sprite: SpriteSheetBundle {
                    // material: materials.add(asset_server.load(sprite).into()),
                    texture_atlas: player_materials.player.clone(),
                    transform: Transform {
                        scale: Vec3::new(2., 2., 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                body: RigidBody::Dynamic,
                shape: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 1.),
                    border_radius: Some(0.),
                },
                velocity: Velocity::from_linear(Vec3::Y * 1.),
                rotation_constraints: RotationConstraints::lock()
            });
    }
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Lives,
        &mut DamageTaken,
        &mut Velocity,
        With<Player>,
    )>,
) {
    for (player_entity, mut transform, mut lives, mut damage_taken, mut velocity, _) in query.iter_mut() {
        if transform.translation.y.abs() > window.height / 2.
            || transform.translation.x.abs() > window.width / 2.
        {
            lives.0 = lives.0 - 1;
            damage_taken.0 = 0.;

            if lives.0 == 0 {
                commands.entity(player_entity).despawn();
            } else {
                transform.translation = Vec3::new(0., 0., 1.);
                velocity.linear = Vec3::Y * 1.;
            }
        }
    }
}

fn reset_jumps(mut events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut AvailableJumps, With<Player>)>
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(collider1, collider2) => {
                for (player_entity, mut available_jumps, _) in player_query.iter_mut() {
                    if player_entity == collider1.rigid_body_entity() || player_entity == collider2.rigid_body_entity() {
                        available_jumps.0 = 2;
                    }
                }
            }
            CollisionEvent::Stopped(_, _) => {}
        }
    }
}
