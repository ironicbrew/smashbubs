use bevy::{math::Vec3, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::gamepad::AddPlayerEvent;

pub struct PlayerPlugin;

const SPRITES: [&str; 13] = [
    "pig.png",
    "rat.png",
    "bat.png",
    "blocky.png",
    "blue_ring.png",
    "crabtopus.png",
    "iron.png",
    "perl.png",
    "pig.png",
    "player.png",
    "rat.png",
    "slug.png",
    "turtle.png",
];
const BLOCK_SPRITE: &str = "block.png";

const DEFAULT_PLATFORM_THICKNESS: f32 = 12.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprites)
            .insert_resource(PlayerCount(0))
            .add_startup_system(setup_map)
            .add_startup_system(render_player_ui)
            .add_system(add_player)
            .add_event::<PlayerDamageEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_system(handle_player_death_event)
            .add_system(reset_jumps)
            .add_system(respawn_players_who_leave_window)
            .add_system(update_player_ui)
            .add_event::<AddPlayerEvent>();
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct PlayerMaterials {
    player: Vec<Handle<TextureAtlas>>,
}

fn setup_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut player_materials = PlayerMaterials { player: vec![] };
    for sprite in SPRITES.iter() {
        let texture_handle = asset_server.load(*sprite);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 1, 1, None, None);
        player_materials
            .player
            .push(texture_atlases.add(texture_atlas));
    }
    commands.insert_resource(player_materials);
}

#[derive(Resource)]
struct PlayerCount(pub u32);

#[derive(Component)]
pub struct Map;

fn add_player(
    mut commands: Commands,
    mut ev_add_player: EventReader<AddPlayerEvent>,
    ui_query: Query<(Entity, &UIComponent)>,
    asset_server: Res<AssetServer>,
    player_materials: Res<PlayerMaterials>,
    mut player_count: ResMut<PlayerCount>,
) {
    for event in ev_add_player.iter() {
        commands.spawn(PlayerBundle {
            gamepad: PlayerGamepad(event.0),
            player_index: PlayerIndex(player_count.0),
            sprite: SpriteSheetBundle {
                texture_atlas: player_materials.player[player_count.0 as usize].clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(2., 2.0, 1.),
                    ..default()
                },
                ..default()
            },
            ..default()
        });

        for (ui_entity, ui) in ui_query.iter() {
            if ui.0 == "bottom-container" {
                commands
                    .spawn(UIImageBundle {
                        bundle: ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.), Val::Px(200.)),
                                ..default()
                            },
                            image: asset_server.load(SPRITES[player_count.0 as usize]).into(),
                            background_color: Color::rgba(1., 1., 1., 0.5).into(),
                            ..default()
                        },
                        _ui: UIComponent(String::from("pig")),
                        player_index: PlayerIndex(player_count.0),
                    })
                    .with_children(|parent| {
                        // text
                        parent.spawn(TextBundle::from_sections([
                            TextSection {
                                value: "Health: ".to_string(),
                                style: TextStyle {
                                    font_size: 30.,
                                    color: Color::WHITE,
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    ..default()
                                },
                            },
                            TextSection {
                                value: PlayerBundle::default().health.0.to_string(),
                                style: TextStyle {
                                    font_size: 30.,
                                    color: Color::WHITE,
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    ..default()
                                },
                            },
                        ]));
                        parent.spawn(TextBundle::from_section(
                            PlayerBundle::default().lives.0.to_string(),
                            TextStyle {
                                font_size: 30.,
                                color: Color::WHITE,
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                ..default()
                            },
                        ));
                    })
                    .set_parent(ui_entity);
            }
        }
        player_count.0 = player_count.0 + 1;
    }
}

fn setup_map(mut commands: Commands, windows: ResMut<Windows>, asset_server: Res<AssetServer>) {
    let window = windows.iter().next().unwrap();

    let setup_bottom_block = |commands: &mut Commands| {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(BLOCK_SPRITE),
                transform: Transform {
                    scale: Vec3::new(window.width(), 2., 1.),
                    translation: Vec3::new(
                        0.0,
                        -(window.height() / 2.) + DEFAULT_PLATFORM_THICKNESS,
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Collider::cuboid(1., 4.))
            // .insert(TransformBundle::from(Transform::from_xyz(
            //     10.0,
            //     -(window.height() / 2.) + DEFAULT_PLATFORM_THICKNESS,
            //     1.0,
            // )))
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
pub struct Lives(pub u32);

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct DamageTaken(pub f32);

#[derive(Component)]
pub struct PlayerSpriteSheet(pub SpriteSheetBundle);

#[derive(Component, Clone)]
pub struct PlayerIndex(u32);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub gamepad: PlayerGamepad,
    player_index: PlayerIndex,
    available_jumps: AvailableJumps,
    lives: Lives,
    damage_taken: DamageTaken,
    speed: Speed,
    _p: Player,
    rigid_body: RigidBody,
    additional_mass_properties: AdditionalMassProperties,
    collider: Collider,
    locked_axis: LockedAxes,
    velocity: Velocity,
    active_events: ActiveEvents,
    // active_collision_types: ActiveCollisionTypes,
    gravity_scale: GravityScale,
    health: Health,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            gamepad: PlayerGamepad(Gamepad { id: 1 }),
            player_index: PlayerIndex(0),
            damage_taken: DamageTaken(0.),
            available_jumps: AvailableJumps(2),
            lives: Lives(2),
            _p: Player,
            speed: Speed(1.),
            health: Health(20),
            sprite: SpriteSheetBundle {
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            additional_mass_properties: AdditionalMassProperties::Mass(10.),
            collider: Collider::cuboid(4., 4.),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
            // active_collision_types: ActiveCollisionTypes::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            gravity_scale: GravityScale(10.),
        }
    }
}

fn respawn_players_who_leave_window(
    mut commands: Commands,
    windows: ResMut<Windows>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Lives,
        &mut DamageTaken,
        &mut Health,
    )>,
) {
    if let Some(window) = windows.iter().next() {
        for (player_entity, transform, lives, damage_taken, health) in query.iter_mut() {
            if transform.translation.y.abs() > window.height() / 2.
                || transform.translation.x.abs() > window.width() / 2.
            {
                handle_player_dealth(
                    &mut commands,
                    transform,
                    lives,
                    damage_taken,
                    health,
                    player_entity,
                );
                break;
            }
        }
    }
}

fn handle_player_dealth(
    commands: &mut Commands,
    mut transform: Mut<Transform>,
    mut lives: Mut<Lives>,
    mut damage_taken: Mut<DamageTaken>,
    mut health: Mut<Health>,
    player_entity: Entity,
) {
    lives.0 = lives.0 - 1;
    damage_taken.0 = 0.;
    health.0 = PlayerBundle::default().health.0;

    if lives.0 == 0 {
        commands.entity(player_entity).despawn();
    } else {
        transform.translation = Vec3::new(0., 0., 1.);
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

fn handle_player_death_event(
    mut commands: Commands,
    mut player_dealth_event: EventReader<PlayerDeathEvent>,
    mut players: Query<
        (
            &mut Transform,
            &mut Lives,
            &mut DamageTaken,
            &mut Health,
            &PlayerIndex,
            Entity,
        ),
        With<Player>,
    >,
) {
    for PlayerDeathEvent(event_player_index) in player_dealth_event.iter() {
        for (transform, lives, damage_taken, health, player_index, player_entity) in
            players.iter_mut()
        {
            if event_player_index.0 == player_index.0 {
                handle_player_dealth(
                    &mut commands,
                    transform,
                    lives,
                    damage_taken,
                    health,
                    player_entity,
                );
                break;
            }
        }
    }
}
pub struct PlayerDamageEvent(pub PlayerIndex, pub DamageTaken);
pub struct PlayerDeathEvent(pub PlayerIndex);

fn update_player_ui(
    mut players: Query<(&mut Health, &PlayerIndex, &mut DamageTaken, &Lives), With<Player>>,
    mut ui_query: Query<(&Children, &PlayerIndex), With<PlayerIndex>>,
    mut player_damage_event: EventReader<PlayerDamageEvent>,
    mut player_death_event: EventWriter<PlayerDeathEvent>,
    mut text: Query<&mut Text>,
) {
    for PlayerDamageEvent(damage_event_player_index, damage_event_damage_taken) in
        player_damage_event.iter()
    {
        for (mut player_health, player_index, mut player_damage_taken, player_lives) in
            players.iter_mut()
        {
            if player_index.0 == damage_event_player_index.0 {
                player_health.0 = player_health.0 - damage_event_damage_taken.0 as i32;
                player_damage_taken.0 = player_damage_taken.0 + damage_event_damage_taken.0;
                for (ui_children, ui_player_index) in ui_query.iter_mut() {
                    if damage_event_player_index.0 == ui_player_index.0 {
                        for &child in ui_children.iter() {
                            let mut text = text.get_mut(child).unwrap();
                            if text.sections[0].value == "Health: ".to_string() {
                                if player_health.0 > 0 {
                                    text.sections[1].value = player_health.0.to_string();
                                } else {
                                    if player_lives.0 == 1 {
                                        text.sections[0].value = "".to_string();
                                        text.sections[1].value = "".to_string();
                                    } else {
                                        text.sections[1].value =
                                            PlayerBundle::default().health.0.to_string();
                                    }
                                }
                            } else {
                                if player_health.0 == 0 && player_lives.0 == 1 {
                                    text.sections[0].value = "Dead".to_string();
                                } else {
                                    text.sections[0].value = player_lives.0.to_string();
                                }
                            }
                        }
                    }
                }
                if player_health.0 == 0 {
                    player_death_event.send(PlayerDeathEvent(player_index.clone()));
                }
            }
        }
    }
}

#[derive(Component)]
pub struct UIComponent(String);

#[derive(Bundle)]
struct UIImageBundle<T: Bundle> {
    _ui: UIComponent,
    player_index: PlayerIndex,

    #[bundle]
    bundle: T,
}

fn render_player_ui(mut commands: Commands) {
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
            parent.spawn(UIImageBundle {
                bundle: NodeBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        size: Size::new(Val::Percent(100.), Val::Px(200.)),
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.5).into(),
                    ..default()
                },
                _ui: UIComponent(String::from("bottom-container")),
                player_index: PlayerIndex(100),
            });
        });
}
