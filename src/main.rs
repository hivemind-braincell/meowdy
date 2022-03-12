//! meowdy

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: "Meowdy!".into(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.run();
}
