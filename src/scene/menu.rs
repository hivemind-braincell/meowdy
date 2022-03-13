use bevy::prelude::*;
use tracing::instrument;

use crate::{animation::Animation, assets::Sprites, GameState};

#[derive(Component)]
pub struct MainMenu;

#[instrument(skip(commands, sprites, texture_atlases))]
pub fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("entered menu");

    let texture_handle = sprites.mainmenuscreen.clone();
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(360., 270.),
        2,
        1,
        Vec2::ONE,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.5, true),
            current_frame: 0,
            start_frame: 0,
            frames: 2,
        })
        .insert(MainMenu)
        .insert(Name::new("Main Menu Background"));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            visibility: Visibility {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn click_item(query: Query<&Interaction>, mut app_state: ResMut<State<GameState>>) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Clicked => {
                info!("main menu clicked");
                app_state.set(GameState::Outside).unwrap();
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

#[instrument(skip(commands, query))]
pub fn teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    info!("tearing down main menu");
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}
