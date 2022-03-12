//! meowdy

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Output verbose logs
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: "Meowdy!".into(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    if args.debug {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}
