//! meowdy

#![allow(clippy::type_complexity)]

use animation::Animation;
use assets::{Images, Sprites};
use bevy::{log::LogSettings, prelude::*};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use clap::Parser;
use tracing::instrument;

mod animation;
mod assets;
mod control;
mod player;
mod scene;

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
pub enum GameState {
    AssetLoading,
    MainMenu,
    Outside,
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
        .continue_to_state(GameState::MainMenu)
        .with_collection::<Sprites>()
        .with_collection::<Images>()
        .build(&mut app);

    let log_level = if args.verbose { "debug" } else { "info" };
    let filter = format!("meowdy={log_level},wgpu=error,bevy_render=info");

    app.insert_resource(WindowDescriptor {
        title: "Meowdy!".into(),
        width: 1440.,
        height: 1080.,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(LogSettings {
        filter,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_state(GameState::AssetLoading)
    .add_startup_system(set_up_camera)
    .add_startup_system(set_up_physics)
    .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(scene::menu::setup))
    .add_system_set(
        SystemSet::on_update(GameState::MainMenu)
            .with_system(scene::menu::click_item)
            .with_system(animation::animate),
    )
    .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(scene::menu::teardown))
    .add_system_set(
        SystemSet::on_enter(GameState::Outside)
            .with_system(player::spawn_player)
            .with_system(scene::outside::setup)
            .before(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .with_system(control::read_control_input)
            .label(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .after(Label::ReadInput)
            .with_system(control::update_facing)
            .label(Label::ApplyInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .after(Label::ReadInput)
            .with_system(control::move_controlled)
            .label(Label::Move),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .after(Label::ApplyInput)
            .with_system(animation::start_stop_player_animation)
            .with_system(animation::update_player_animation)
            .label(Label::PrepareAnimation),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .after(Label::PrepareAnimation)
            .with_system(animation::animate)
            .label(Label::Animate),
    )
    .register_type::<Animation>();

    if args.inspector {
        info!("adding world inspector plugin");
        app.add_plugin(WorldInspectorPlugin::new());

        info!("adding rapier render plugin");
        app.add_plugin(RapierRenderPlugin);
    }

    app.run();
}

#[instrument(skip(commands))]
fn set_up_camera(mut commands: Commands) {
    info!("spawning orthographic camera bundle");

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 4.;

    commands
        .spawn_bundle(camera_bundle)
        .insert(Name::new("Camera"));
}

#[instrument(skip(rapier_config))]
fn set_up_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 36.;

    info!(gravity = ?rapier_config.gravity, scale = ?rapier_config.scale, "configured rapier");
}
