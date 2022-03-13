//! Asset loading and handling.

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(path = "sprites/Player.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/groundwide.png")]
    pub groundwide: Handle<Image>,
    #[asset(path = "sprites/mainmenuscreen.png")]
    pub mainmenuscreen: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct Images {
    #[asset(path = "images/buildings.png")]
    pub buildings: Handle<Image>,
    #[asset(path = "images/head.png")]
    pub head: Handle<Image>,
    #[asset(path = "images/tail.png")]
    pub tail: Handle<Image>,
    #[asset(path = "images/letter.png")]
    pub letter: Handle<Image>,
    #[asset(path = "images/snakebackground.png")]
    pub snakebackground: Handle<Image>,
    #[asset(path = "images/meowdyend1.png")]
    pub end_1: Handle<Image>,
    #[asset(path = "images/meowdyend2.png")]
    pub end_2: Handle<Image>,
}
