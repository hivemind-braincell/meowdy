//! Player-related components and systems.

use bevy::prelude::*;
use tracing::instrument;

use crate::assets::Sprites;

#[derive(Component)]
pub struct Player;

#[instrument(skip(commands, sprites))]
pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = sprites.player.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        });
}
