//! Player-related components and systems.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tracing::instrument;

use crate::assets::Sprites;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub speed: f32,
    pub sprint_multiplier: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.,
            sprint_multiplier: 1.5,
        }
    }
}

#[instrument(skip(commands, sprites, texture_atlases, rapier_config))]
pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = sprites.player.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player::default();

    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 4.;

    commands
        .spawn()
        .insert(player.clone())
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
        .insert_bundle((RigidBodyPositionSync::Discrete, Name::new("Player"), player))
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(64., 64.).into(),
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
    mut player: Query<(&Player, &mut RigidBodyVelocityComponent)>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
) {
    let up = keyboard_input.any_pressed([KeyCode::W]);
    let left = keyboard_input.any_pressed([KeyCode::A]);
    let down = keyboard_input.any_pressed([KeyCode::S]);
    let right = keyboard_input.any_pressed([KeyCode::D]);

    let sprint = keyboard_input.any_pressed([KeyCode::LShift]);

    if up || left || down || right || sprint {
        debug!(
            ?up,
            ?left,
            ?down,
            ?right,
            ?sprint,
            "received player keyboard input"
        );
    }

    for (player, mut rb_vels) in player.iter_mut() {
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            rb_vels.linvel.x = player.speed * (x_axis as f32) * rapier_config.scale;

            if sprint {
                rb_vels.linvel.x *= player.sprint_multiplier;
            }
        } else {
            rb_vels.linvel.x = 0.;
        }

        if y_axis != 0 {
            rb_vels.linvel.y = player.speed * (y_axis as f32) * rapier_config.scale;

            if sprint {
                rb_vels.linvel.y *= player.sprint_multiplier;
            }
        } else {
            rb_vels.linvel.y = 0.;
        }

        if !(x_axis == 0 && y_axis == 0) {
            debug!(player_velocity = ?rb_vels.linvel, "new player velocity");
        }
    }
}
