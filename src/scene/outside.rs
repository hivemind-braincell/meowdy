use bevy::prelude::*;

use crate::{
    animation::Animation,
    assets::{Images, Sprites},
};

pub fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    images: Res<Images>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn().insert_bundle(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0., 63., 0.)),
        texture: images.buildings.clone(),
        ..Default::default()
    });

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
        });
}
