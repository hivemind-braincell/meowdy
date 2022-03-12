//! Asset loading and handling.

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(path = "sprites/Player.png")]
    pub player: Handle<Image>,
}
