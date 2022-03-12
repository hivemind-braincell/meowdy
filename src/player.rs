//! Player-related components and systems.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::{
    animation::Animation,
    assets::Sprites,
    control::{Controlled, Facing, Moves},
};

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Player;

#[instrument(skip(commands, sprites, texture_atlases, rapier_config))]
pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = sprites.player.clone();
    let texture_atlas =
        TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(36., 36.), 4, 2, Vec2::ONE);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 4.;

    commands
        .spawn()
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Moves { speed: 5. })
        .insert(Controlled::default())
        .insert(Facing::Right)
        .insert(Animation {
            timer: Timer::from_seconds(0.1, true),
            current_frame: 0,
            start_frame: 0,
            frames: 1,
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., -12., 2.)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0., -12.).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(36., 36.).into(),
                position: Vec2::new(0., 0.).into(),
                material: ColliderMaterial {
                    friction: 0.,
                    restitution: 0.,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            });
        });
}
