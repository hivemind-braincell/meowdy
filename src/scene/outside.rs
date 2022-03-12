use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::Animation,
    assets::{Images, Sprites},
};

pub fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    images: Res<Images>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    rapier_config: Res<RapierConfiguration>,
) {
    let scale = rapier_config.scale;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 63., 0.)),
            texture: images.buildings.clone(),
            ..Default::default()
        })
        .insert(Name::new("Background"));

    let texture_handle = sprites.groundwide.clone();
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(360., 144.),
        3,
        1,
        Vec2::ONE,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., -63., 1.)),
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.2, true),
            current_frame: 0,
            start_frame: 0,
            frames: 3,
        })
        .insert(Name::new("Ground"));

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
        .insert(Name::new("Top Collider"));

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
        .insert(Name::new("Bottom Collider"));

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
        .insert(Name::new("Left Collider"));

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
        .insert(Name::new("Right Collider"));
}
