//! meowdy

use bevy::{log::LogSettings, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use clap::Parser;
use tracing::instrument;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Enable the debug inspector
    #[clap(short, long)]
    debug: bool,
    /// Output verbose logs
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    let log_level = if args.verbose { "debug" } else { "info" };
    let filter = format!("meowdy={log_level},wgpu=error,bevy_render=info");

    app.insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: "Meowdy!".into(),
        ..Default::default()
    })
    .insert_resource(LogSettings {
        filter,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(camera_setup);

    if args.debug {
        info!(debug = %args.debug, "adding debug inspector plugin");
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

#[instrument(skip(commands))]
fn camera_setup(mut commands: Commands) {
    debug!("spawning orthographic camera bundle");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
