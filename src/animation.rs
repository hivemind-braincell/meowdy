use bevy::prelude::*;

use crate::player::{Player, PlayerDirection};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Animation {
    pub timer: Timer,

    pub current_frame: usize,
    pub start_frame: usize,
    pub frames: usize,
}

pub fn animate_player(
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.current_frame += 1;
            animation.current_frame %= animation.frames;

            sprite.index = animation.start_frame + animation.current_frame;
        }
    }
}

pub fn update_player_animation(mut query: Query<(&mut Player, &mut Animation)>) {
    for (mut player, mut animation) in query.iter_mut() {
        if player.changed {
            player.changed = false;
            match (&player.moving, &player.direction) {
                (true, PlayerDirection::Right) => {
                    animation.current_frame = 0;
                    animation.start_frame = 0;
                    animation.frames = 4;
                }
                (true, PlayerDirection::Left) => {
                    animation.current_frame = 0;
                    animation.start_frame = 4;
                    animation.frames = 4;
                }
                (false, PlayerDirection::Right) => {
                    animation.current_frame = 0;
                    animation.start_frame = 0;
                    animation.frames = 1;
                }
                (false, PlayerDirection::Left) => {
                    animation.current_frame = 0;
                    animation.start_frame = 4;
                    animation.frames = 1;
                }
            }
        }
    }
}
