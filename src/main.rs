//! meowdy

use animation::Animation;
use assets::Sprites;
use bevy::{log::LogSettings, prelude::*};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;
use clap::Parser;
use control::Facing;
use tracing::instrument;

mod animation;
mod assets;
mod control;
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

#[derive(SystemLabel, Clone, Debug, PartialEq, Eq, Hash)]
enum Label {
    ReadInput,
    ApplyInput,
    Move,
    PrepareAnimation,
    Animate,
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
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_state(GameState::AssetLoading)
    .add_startup_system(camera_setup)
    .add_system_set(
        SystemSet::on_enter(GameState::InGame)
            .with_system(player::spawn_player)
            .before(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
            .with_system(control::read_control_input)
            .label(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
            .after(Label::ReadInput)
            .with_system(control::update_facing)
            .label(Label::ApplyInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
            .after(Label::ReadInput)
            .with_system(control::move_controlled)
            .label(Label::Move),
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
            .after(Label::ApplyInput)
            .with_system(animation::start_stop_player_animation)
            .with_system(animation::update_player_animation)
            .label(Label::PrepareAnimation),
    )
    .add_system_set(
        SystemSet::on_update(GameState::InGame)
            .after(Label::PrepareAnimation)
            .with_system(animation::animate)
            .label(Label::Animate),
    )
    .register_type::<Animation>();

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
