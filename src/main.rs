//! meowdy

use assets::Sprites;
use bevy::{log::LogSettings, prelude::*};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use clap::Parser;
use tracing::instrument;

mod assets;
mod player;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Enable the debug inspector
    #[clap(short, long)]
    inspector: bool,
    /// Output verbose logs
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    InGame,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::InGame)
        .with_collection::<Sprites>()
        .build(&mut app);

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
    .add_state(GameState::AssetLoading)
    .add_startup_system(camera_setup)
    .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(player::spawn_player));

    if args.inspector {
        info!("adding world inspector plugin");
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

#[instrument(skip(commands))]
fn camera_setup(mut commands: Commands) {
    debug!("spawning orthographic camera bundle");

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 4.;

    commands.spawn_bundle(camera_bundle);
}
