use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::{
    animation::Animation,
    assets::{Fonts, Images, Sprites},
    control::{Controlled, Facing, Moves},
    GameState, Meowney,
};

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct Scenery;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct MeowneyDisplay;

#[derive(Component)]
pub struct MeowneyError;

#[instrument(skip(commands, sprites, images, fonts, texture_atlases, rapier_config))]
pub fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    images: Res<Images>,
    fonts: Res<Fonts>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    rapier_config: Res<RapierConfiguration>,
    meowney: Res<Meowney>,
) {
    info!("setting up outside scene");

    let scale = rapier_config.scale;

    let player_handle = sprites.player.clone();
    let player_atlas =
        TextureAtlas::from_grid_with_padding(player_handle, Vec2::new(36., 36.), 4, 2, Vec2::ONE);
    let player_atlas_handle = texture_atlases.add(player_atlas);

    commands
        .spawn()
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Moves { speed: 2.5 })
        .insert(Controlled::default())
        .insert(Facing::Right)
        .insert(Animation {
            timer: Timer::from_seconds(0.1, true),
            current_frame: 0,
            start_frame: 0,
            frames: 1,
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0., -42. / scale).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(0.35, 0.5).into(),
                    position: Vec2::new(0., 0.).into(),
                    material: ColliderMaterial {
                        friction: 0.,
                        restitution: 0.,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(ColliderDebugRender::default());
        });

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 63., 0.)),
            texture: images.buildings.clone(),
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .insert(Scenery);

    let ground_andle = sprites.groundwide.clone();
    let ground_atlas =
        TextureAtlas::from_grid_with_padding(ground_andle, Vec2::new(360., 144.), 3, 1, Vec2::ONE);
    let ground_atlas_handle = texture_atlases.add(ground_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: ground_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., -63., 1.)),
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.2, true),
            current_frame: 0,
            start_frame: 0,
            frames: 3,
        })
        .insert(Name::new("Ground"))
        .insert(Scenery);

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(0., 45. / scale).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50., 2. / scale).into(),
            material: ColliderMaterial {
                friction: 0.,
                restitution: 0.,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Name::new("Top Collider"))
        .insert(Collider);

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(0., -136. / scale).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(50., 1. / scale).into(),
            material: ColliderMaterial {
                friction: 0.,
                restitution: 0.,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Name::new("Bottom Collider"))
        .insert(Collider);

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(-181. / scale, 0.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1. / scale, 135. / scale).into(),
            material: ColliderMaterial {
                friction: 0.,
                restitution: 0.,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Name::new("Left Collider"))
        .insert(Collider);

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec2::new(181. / scale, 0.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1. / scale, 135. / scale).into(),
            material: ColliderMaterial {
                friction: 0.,
                restitution: 0.,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Name::new("Right Collider"))
        .insert(Collider);

    let font = fonts.vt323.clone();
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    format!("Meowney: {}", meowney.0),
                    TextStyle {
                        font,
                        font_size: 36.0,
                        color: Color::BLUE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Right,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(Name::new("Meowney Display"))
        .insert(MeowneyDisplay);
}

#[instrument(skip(keyboard_input, app_state))]
pub fn scene_transition(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    meowney: Res<Meowney>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        info!("transitioning to post office scene");
        app_state.set(GameState::PostOffice).unwrap();
    } else if keyboard_input.pressed(KeyCode::E) {
        if meowney.0 >= 20 {
            info!("transitioning to end scene");
            app_state.set(GameState::End).unwrap();
        } else {
            warn!("player does not have enough meowney");
        }
    }
}

#[instrument(skip(commands, player, scenery, colliders, meowney_display))]
pub fn teardown(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    scenery: Query<Entity, With<Scenery>>,
    colliders: Query<Entity, With<Collider>>,
    meowney_display: Query<Entity, With<MeowneyDisplay>>,
) {
    info!("tearing down outside scene");
    player.for_each(|entity| commands.entity(entity).despawn_recursive());
    scenery.for_each(|entity| commands.entity(entity).despawn_recursive());
    colliders.for_each(|entity| commands.entity(entity).despawn_recursive());
    meowney_display.for_each(|entity| commands.entity(entity).despawn_recursive());
}
