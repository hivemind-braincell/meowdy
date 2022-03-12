//! Asset loading and handling.

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(path = "sprites/Player.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/groundwide.png")]
    pub groundwide: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct Images {
    #[asset(path = "images/buildings.png")]
    pub buildings: Handle<Image>,
}
