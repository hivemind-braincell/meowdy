//! Motion, position and control.

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use tracing::instrument;

#[derive(Component, Clone, Debug)]
pub struct Moves {
    pub speed: f32,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Controlled {
    pub inputs: HashSet<ControlInput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ControlInput {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Facing {
    Right,
    Left,
}

#[instrument(skip(query, keyboard_input))]
pub fn read_control_input(mut query: Query<&mut Controlled>, keyboard_input: Res<Input<KeyCode>>) {
    for mut controlled in query.iter_mut() {
        controlled.inputs.clear();

        if keyboard_input.pressed(KeyCode::W) {
            controlled.inputs.insert(ControlInput::Up);
        }
        if keyboard_input.pressed(KeyCode::A) {
            controlled.inputs.insert(ControlInput::Left);
        }
        if keyboard_input.pressed(KeyCode::S) {
            controlled.inputs.insert(ControlInput::Down);
        }
        if keyboard_input.pressed(KeyCode::D) {
            controlled.inputs.insert(ControlInput::Right);
        }
    }
}

pub fn update_facing(mut query: Query<(&mut Facing, &Controlled)>) {
    for (mut facing, controlled) in query.iter_mut() {
        match (
            controlled.inputs.contains(&ControlInput::Left),
            controlled.inputs.contains(&ControlInput::Right),
        ) {
            (true, false) => {
                if *facing != Facing::Left {
                    *facing = Facing::Left;
                }
            }
            (false, true) => {
                if *facing != Facing::Right {
                    *facing = Facing::Right;
                }
            }
            _ => (),
        }
    }
}

pub fn move_controlled(mut query: Query<(&mut RigidBodyVelocityComponent, &Controlled, &Moves)>) {
    for (mut rigid_body, controlled, moves) in query.iter_mut() {
        let up = controlled.inputs.contains(&ControlInput::Up);
        let left = controlled.inputs.contains(&ControlInput::Left);
        let down = controlled.inputs.contains(&ControlInput::Down);
        let right = controlled.inputs.contains(&ControlInput::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            rigid_body.linvel.x = x_axis as f32;
        } else {
            rigid_body.linvel.x = 0.;
        }

        if y_axis != 0 {
            rigid_body.linvel.y = y_axis as f32;
        } else {
            rigid_body.linvel.y = 0.;
        }

        if !(x_axis == 0 && y_axis == 0) {
            rigid_body.linvel.normalize_mut();
            rigid_body.linvel *= moves.speed;
        }
    }
}
