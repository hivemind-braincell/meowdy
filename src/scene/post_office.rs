use bevy::prelude::*;
use rand::Rng;
use tracing::instrument;

use crate::{assets::Images, GameState, Meowney};

const ARENA_WIDTH: i32 = 5;
const ARENA_HEIGHT: i32 = 5;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct MoveTimer(pub Timer);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SnakeSegment;

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

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

pub struct SnakeGrowth;

pub struct SpawnFood;

pub struct SnakeGameOver;

#[instrument(skip(commands, images, segments, spawn_writer))]
pub fn setup(
    mut commands: Commands,
    images: Res<Images>,
    mut segments: ResMut<SnakeSegments>,
    mut spawn_writer: EventWriter<SpawnFood>,
) {
    info!("setting up post office scene");

    commands.insert_resource(MoveTimer(Timer::from_seconds(0.2, true)));

    segments.0 = vec![
        commands
            .spawn()
            .insert(Name::new("Snake Head"))
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 2 })
            .insert_bundle(SpriteBundle {
                texture: images.head.clone(),
                ..Default::default()
            })
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }, images),
    ];

    info!("sending spawn food event");
    spawn_writer.send(SpawnFood);
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

pub fn move_snake(
    segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<SnakeGameOver>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut move_timer: ResMut<MoveTimer>,
    time: Res<Time>,
) {
    move_timer.0.tick(time.delta());

    if move_timer.0.finished() {
        if let Some((head_entity, head)) = heads.iter_mut().next() {
            let segment_positions = segments
                .0
                .iter()
                .map(|e| *positions.get_mut(*e).unwrap())
                .collect::<Vec<_>>();
            let mut head_pos = positions.get_mut(head_entity).unwrap();

            match &head.direction {
                Direction::Left => head_pos.x -= 1,
                Direction::Down => head_pos.y -= 1,
                Direction::Up => head_pos.y += 1,
                Direction::Right => head_pos.x += 1,
            };

            if head_pos.x < -ARENA_WIDTH
                || head_pos.y < -ARENA_HEIGHT
                || head_pos.x > ARENA_WIDTH
                || head_pos.y > ARENA_HEIGHT
            {
                warn!(?head_pos, "game over, snake hit side of arena");
                game_over_writer.send(SnakeGameOver);
            }

            if segment_positions.contains(&head_pos) {
                warn!(?head_pos, "game over, snake hit own tail");
                game_over_writer.send(SnakeGameOver);
            }

            segment_positions
                .iter()
                .zip(segments.0.iter().skip(1))
                .for_each(|(pos, segment)| {
                    *positions.get_mut(*segment).unwrap() = *pos;
                });
            last_tail_position.0 = Some(*segment_positions.last().unwrap());
        }
    }
}

fn spawn_segment(mut commands: Commands, position: Position, images: Res<Images>) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: images.tail.clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .id()
}

#[derive(Component)]
pub struct Food;

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<SnakeGrowth>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(SnakeGrowth);
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<SnakeGrowth>,
    mut spawn_writer: EventWriter<SpawnFood>,
    images: Res<Images>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            last_tail_position.0.unwrap(),
            images,
        ));
        spawn_writer.send(SpawnFood);
    }
}

#[instrument(skip(commands, images, spawn_reader))]
pub fn food_spawner(
    mut commands: Commands,
    images: Res<Images>,
    mut spawn_reader: EventReader<SpawnFood>,
) {
    if spawn_reader.iter().next().is_some() {
        info!("spawning food");

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-ARENA_WIDTH..=ARENA_WIDTH);
        let y = rng.gen_range(-ARENA_HEIGHT..=ARENA_HEIGHT);

        let position = Position { x, y };

        debug!(?position, "spawning food");

        commands
            .spawn_bundle(SpriteBundle {
                texture: images.letter.clone(),
                ..Default::default()
            })
            .insert(Food)
            .insert(position);
    }
}

pub fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    let scale = 24.;

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x as f32 * scale, pos.y as f32 * scale, 0.);
    }
}

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<SnakeGameOver>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    mut app_state: ResMut<State<GameState>>,
    mut meowney: ResMut<Meowney>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter() {
            commands.entity(ent).despawn();
        }
        let mut earned = 0;
        for ent in segments.iter() {
            commands.entity(ent).despawn();
            earned += 1;
        }
        earned -= 2;
        meowney.0 += earned;

        info!(meowney = %meowney.0, "meowney updated");

        app_state.set(GameState::Outside).unwrap();
    }
}
