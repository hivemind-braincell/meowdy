use bevy::prelude::*;

use crate::{
    control::{Controlled, Facing},
    player::Player,
};

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Animation {
    pub timer: Timer,
    pub current_frame: usize,
    pub start_frame: usize,
    pub frames: usize,
}

pub fn start_stop_player_animation(
    mut query: Query<(&mut Animation, &Controlled), (Changed<Controlled>, With<Player>)>,
) {
    for (mut animation, controlled) in query.iter_mut() {
        let moving = !controlled.inputs.is_empty();
        if moving {
            animation.frames = 4;
        } else {
            animation.frames = 1;
        }
    }
}

pub fn update_player_animation(
    mut query: Query<(&mut Animation, &Facing), (Changed<Facing>, With<Player>)>,
) {
    for (mut animation, facing) in query.iter_mut() {
        match facing {
            Facing::Right => {
                animation.current_frame = 0;
                animation.start_frame = 0;
            }
            Facing::Left => {
                animation.current_frame = 0;
                animation.start_frame = 4;
            }
        }
    }
}

pub fn animate(mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.current_frame += 1;
            animation.current_frame %= animation.frames;

            sprite.index = animation.start_frame + animation.current_frame;
        }
    }
}
