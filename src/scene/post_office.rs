use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::assets::{Images, Sprites};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Down => Self::Up,
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

#[instrument(skip(commands, images))]
pub fn setup(mut commands: Commands, images: Res<Images>) {
    info!("setting up post office scene");

    commands
        .spawn()
        .insert(Name::new("Snake Head"))
        .insert(SnakeHead {
            direction: Direction::Up,
        })
        .insert(Position { x: 3, y: 2 })
        .insert_bundle(SpriteBundle {
            texture: images.head.clone(),
            ..Default::default()
        });
}

pub fn update_head_direction(
    mut query: Query<&mut SnakeHead>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut head in query.iter_mut() {
        let dir = if keyboard_input.pressed(KeyCode::A) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn move_snake(mut query: Query<(Entity, &SnakeHead, &mut Position)>) {
    for (_entity, head, mut head_pos) in query.iter_mut() {
        match &head.direction {
            Direction::Left => head_pos.x -= 1,
            Direction::Down => head_pos.y -= 1,
            Direction::Up => head_pos.y += 1,
            Direction::Right => head_pos.x += 1,
        };
    }
}

pub fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    let scale = 24.;

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32 * scale, pos.y as f32 * scale, 0.);
    }
}

pub fn teardown(mut commands: Commands) {
    info!("tearing down post office scene");
}
