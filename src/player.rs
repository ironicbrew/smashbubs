use bevy::{math::Vec3, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::gamepad::AddPlayerEvent;

pub struct PlayerPlugin;

const PIG_SPRITE: &str = "pig.png";

const DEFAULT_PLATFORM_THICKNESS: f32 = 10.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites)
            .add_startup_system(setup_map)
            .add_startup_system(render_player_ui)
            .add_system(add_player)
            .add_system(reset_jumps)
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

#[derive(Component)]
pub struct Map;

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

fn setup_map(mut commands: Commands, windows: ResMut<Windows>) {
    let window = windows.iter().next().unwrap();

    let setup_bottom_block = |commands: &mut Commands| {
        commands
            .spawn(Collider::cuboid(window.width(), DEFAULT_PLATFORM_THICKNESS))
            .insert(TransformBundle::from(Transform::from_xyz(
                0.0,
                -(window.height() / 2.) + DEFAULT_PLATFORM_THICKNESS,
                1.0,
            )))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Map);
    };

    let setup_top_block = |commands: &mut Commands| {
        commands
            .spawn(Collider::cuboid(window.width(), DEFAULT_PLATFORM_THICKNESS))
            .insert(TransformBundle::from(Transform::from_xyz(
                0.0,
                (window.height() / 2.) - DEFAULT_PLATFORM_THICKNESS,
                1.0,
            )))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Map);
    };
    let setup_left_block = |commands: &mut Commands| {
        commands
            .spawn(Collider::cuboid(
                DEFAULT_PLATFORM_THICKNESS,
                window.height(),
            ))
            .insert(TransformBundle::from(Transform::from_xyz(
                -(window.width() / 2.) + DEFAULT_PLATFORM_THICKNESS,
                0.,
                1.,
            )))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Map);
    };

    let setup_right_block = |commands: &mut Commands| {
        commands
            .spawn(Collider::cuboid(
                DEFAULT_PLATFORM_THICKNESS,
                window.height(),
            ))
            .insert(TransformBundle::from(Transform::from_xyz(
                (window.width() / 2.) - DEFAULT_PLATFORM_THICKNESS,
                0.,
                1.,
            )))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Map);
    };

    let create_platform = |commands: &mut Commands, width: f32, x_coord: f32, y_coord: f32| {
        commands
            .spawn(Collider::cuboid(width, DEFAULT_PLATFORM_THICKNESS))
            .insert(TransformBundle::from(Transform::from_xyz(
                x_coord,
                -(window.height() / 2.) + DEFAULT_PLATFORM_THICKNESS + y_coord,
                1.,
            )))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Map);
    };

    setup_top_block(&mut commands);
    setup_bottom_block(&mut commands);
    setup_left_block(&mut commands);
    setup_right_block(&mut commands);
    create_platform(&mut commands, 200., -300., 100.);
    create_platform(&mut commands, 200., 300., 100.);
    create_platform(&mut commands, 200., 0., 200.);
    create_platform(&mut commands, 200., -300., 300.);
    create_platform(&mut commands, 200., 300., 300.);
    create_platform(&mut commands, 200., 0., 400.);
    create_platform(&mut commands, 200., -300., 500.);
    create_platform(&mut commands, 200., 300., 500.);
}

#[derive(Component)]
pub struct PlayerGamepad(pub Gamepad);

#[derive(Component)]
pub struct AvailableJumps(pub u32);
#[derive(Component)]
pub struct Lives(u32);
#[derive(Component)]
pub struct DamageTaken(pub f32);

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
    rigid_body: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    velocity: Velocity,
    active_collision_types: ActiveCollisionTypes,
    gravity_scale: GravityScale,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            gamepad: PlayerGamepad(Gamepad { id: 1 }),
            damage_taken: DamageTaken(0.),
            available_jumps: AvailableJumps(2),
            lives: Lives(2),
            _p: Player,
            speed: Speed(1.),
            sprite: SpriteSheetBundle {
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(4., 4.),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
            active_collision_types: ActiveCollisionTypes::default(),
            gravity_scale: GravityScale(10.),
        }
    }
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    windows: ResMut<Windows>,
    mut query: Query<(Entity, &mut Transform, &mut Lives, &mut DamageTaken)>,
) {
    if let Some(window) = windows.iter().next() {
        for (player_entity, mut transform, mut lives, mut damage_taken) in query.iter_mut() {
            if transform.translation.y.abs() > window.height() / 2.
                || transform.translation.x.abs() > window.width() / 2.
            {
                lives.0 = lives.0 - 1;
                damage_taken.0 = 0.;

                if lives.0 == 0 {
                    commands.entity(player_entity).despawn();
                } else {
                    transform.translation = Vec3::new(0., 0., 1.);
                }
            }
        }
    }
}

fn reset_jumps(
    mut events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut AvailableJumps)>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                for (player_entity, mut available_jumps) in player_query.iter_mut() {
                    if *collider1 == player_entity || *collider2 == player_entity {
                        available_jumps.0 = 2
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn render_player_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Screen
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        // bottom container
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        size: Size::new(Val::Percent(100.), Val::Px(200.)),
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.5).into(),
                    ..default()
                })
                // character image
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.), Val::Auto),
                                ..default()
                            },
                            image: asset_server.load("pig.png").into(),
                            background_color: Color::rgba(1., 1., 1., 0.5).into(),
                            ..default()
                        })
                        // text
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Pig",
                                TextStyle {
                                    font_size: 30.,
                                    color: Color::WHITE,
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}
