use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct MainMenu;

pub fn setup(mut commands: Commands) {
    info!("entered menu");

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
                app_state.push(GameState::Outside).unwrap();
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub fn teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}
