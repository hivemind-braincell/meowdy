use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::{
    animation::Animation,
    assets::{Images, Sprites},
    control::{Controlled, Facing, Moves},
    GameState,
};

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct Scenery;

#[derive(Component)]
pub struct Collider;

#[instrument(skip(commands, sprites, images, texture_atlases, rapier_config))]
pub fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    images: Res<Images>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    rapier_config: Res<RapierConfiguration>,
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
}

#[instrument(skip(keyboard_input, app_state))]
pub fn scene_transition(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        info!("transitioning to post office scene");
        app_state.set(GameState::PostOffice).unwrap();
    }
}

#[instrument(skip(commands, player, scenery, colliders))]
pub fn teardown(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    scenery: Query<Entity, With<Scenery>>,
    colliders: Query<Entity, With<Collider>>,
) {
    info!("tearing down outside scene");
    player.for_each(|entity| commands.entity(entity).despawn_recursive());
    scenery.for_each(|entity| commands.entity(entity).despawn_recursive());
    colliders.for_each(|entity| commands.entity(entity).despawn_recursive());
}
