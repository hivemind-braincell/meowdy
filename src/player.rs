//! Player-related components and systems.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::{animation::Animation, assets::Sprites};

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub speed: f32,
    pub sprint_multiplier: f32,

    pub moving: bool,
    pub direction: PlayerDirection,
    pub changed: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.,
            sprint_multiplier: 1.5,
            moving: false,
            direction: PlayerDirection::Right,
            changed: false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerDirection {
    Right,
    Left,
}

#[instrument(skip(commands, sprites, texture_atlases, rapier_config))]
pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = sprites.player.clone();
    let texture_atlas =
        TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(36., 36.), 4, 2, Vec2::ONE);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player::default();

    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 4.;

    commands
        .spawn()
        .insert(Name::new("Player"))
        .insert(player.clone())
        .insert(Animation {
            timer: Timer::from_seconds(0.1, true),
            current_frame: 0,
            start_frame: 0,
            frames: 1,
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0., 0.).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(36., 36.).into(),
                position: Vec2::new(0., 0.).into(),
                material: ColliderMaterial {
                    friction: 0.,
                    restitution: 0.,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            });
        });
}

#[instrument(skip(player, keyboard_input, rapier_config))]
pub fn move_player(
    mut player: Query<(&mut Player, &mut RigidBodyVelocityComponent)>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
) {
    let up = keyboard_input.pressed(KeyCode::W);
    let left = keyboard_input.pressed(KeyCode::A);
    let down = keyboard_input.pressed(KeyCode::S);
    let right = keyboard_input.pressed(KeyCode::D);

    let sprint = keyboard_input.pressed(KeyCode::LShift);

    // The player should turn to the right if:
    //
    // - The right key has just begun being pressed
    // - The left key is not being pressed
    //
    // Or:
    //
    // - The right key was already being pressed
    // - The left key was just released
    let turn_right = (keyboard_input.just_pressed(KeyCode::D) && !left)
        || (right
            && !keyboard_input.just_pressed(KeyCode::D)
            && keyboard_input.just_released(KeyCode::A));

    // Similar for left:
    //
    // - The left key has just begun being pressed
    // - The right key is not being pressed
    //
    // Or:
    //
    // - The left key was already being pressed
    // - The right key was just released
    let turn_left = (keyboard_input.just_pressed(KeyCode::A) && !right)
        || (left
            && !keyboard_input.just_pressed(KeyCode::A)
            && keyboard_input.just_released(KeyCode::D));

    for (mut player, mut rb_vels) in player.iter_mut() {
        if up || left || down || right {
            debug!(?up, ?left, ?down, ?right, ?sprint, "player moving");
            if !player.moving {
                player.moving = true;
                player.changed = true;
            }
        } else {
            if player.moving {
                player.moving = false;
                player.changed = true;
            }
        }

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            rb_vels.linvel.x = x_axis as f32;
            if sprint {}
        } else {
            rb_vels.linvel.x = 0.;
        }

        if y_axis != 0 {
            rb_vels.linvel.y = y_axis as f32;
        } else {
            rb_vels.linvel.y = 0.;
        }

        if !(x_axis == 0 && y_axis == 0) {
            rb_vels.linvel.normalize_mut();
            rb_vels.linvel *= player.speed * rapier_config.scale;

            if sprint {
                rb_vels.linvel *= player.sprint_multiplier;
            }

            debug!(player_velocity = ?rb_vels.linvel, "new player velocity");
        }

        if turn_right {
            debug!("turning right");
            player.direction = PlayerDirection::Right;
            player.changed = true;
        }

        if turn_left {
            debug!("turning left");
            player.direction = PlayerDirection::Left;
            player.changed = true;
        }

        if turn_right && turn_left {
            warn!("turning left and right");
        }
    }
}
