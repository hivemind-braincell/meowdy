//! meowdy

#![allow(clippy::type_complexity)]

use animation::Animation;
use assets::{Fonts, Images, Sprites};
use bevy::{log::LogSettings, prelude::*};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use clap::Parser;
use scene::post_office::{LastTailPosition, SnakeGameOver, SnakeGrowth, SnakeSegments, SpawnFood};
use tracing::instrument;

mod animation;
mod assets;
mod control;
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
    PostOffice,
    End,
}

#[derive(SystemLabel, Clone, Debug, PartialEq, Eq, Hash)]
enum Label {
    ReadInput,
    ApplyInput,
    Move,
    Eat,
    Grow,
    SpawnFood,
    PrepareAnimation,
    Animate,
}

#[derive(Default, Debug)]
pub struct Meowney(pub u32);

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<Sprites>()
        .with_collection::<Images>()
        .with_collection::<Fonts>()
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
    .insert_resource(SnakeSegments::default())
    .insert_resource(LastTailPosition::default())
    .insert_resource(Meowney::default())
    .add_event::<SnakeGrowth>()
    .add_event::<SpawnFood>()
    .add_event::<SnakeGameOver>()
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
            .with_system(scene::outside::setup)
            .before(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Outside)
            .with_system(control::read_control_input.label(Label::ReadInput))
            .with_system(scene::outside::scene_transition.label(Label::ReadInput))
            .with_system(
                control::update_facing
                    .after(Label::ReadInput)
                    .label(Label::ApplyInput),
            )
            .with_system(
                control::move_controlled
                    .after(Label::ApplyInput)
                    .label(Label::Move),
            )
            .with_system(
                animation::start_stop_player_animation
                    .after(Label::ApplyInput)
                    .label(Label::PrepareAnimation),
            )
            .with_system(
                animation::update_player_animation
                    .after(Label::ApplyInput)
                    .label(Label::PrepareAnimation),
            )
            .with_system(
                animation::animate
                    .after(Label::PrepareAnimation)
                    .label(Label::Animate),
            ),
    )
    .add_system_set(SystemSet::on_exit(GameState::Outside).with_system(scene::outside::teardown))
    .add_system_set(
        SystemSet::on_enter(GameState::PostOffice)
            .with_system(scene::post_office::setup)
            .before(Label::ReadInput),
    )
    .add_system_set(
        SystemSet::on_update(GameState::PostOffice)
            .with_system(scene::post_office::update_head_direction.label(Label::ApplyInput)),
    )
    .add_system_set(
        SystemSet::on_update(GameState::PostOffice)
            .with_system(
                scene::post_office::move_snake
                    .after(Label::ApplyInput)
                    .label(Label::Move),
            )
            .with_system(
                scene::post_office::snake_eating
                    .label(Label::Eat)
                    .after(Label::Move),
            )
            .with_system(
                scene::post_office::snake_growth
                    .label(Label::Grow)
                    .after(Label::Eat),
            )
            .with_system(
                scene::post_office::food_spawner
                    .label(Label::SpawnFood)
                    .after(Label::Grow),
            )
            .with_system(scene::post_office::game_over.after(Label::SpawnFood)),
    )
    .add_system_set(SystemSet::on_enter(GameState::End).with_system(scene::end::setup))
    .add_system_set(SystemSet::on_update(GameState::End).with_system(scene::end::switch_end_scene))
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new().with_system(scene::post_office::position_translation),
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

#[derive(Component)]
pub struct Camera;

#[instrument(skip(commands))]
fn set_up_camera(mut commands: Commands) {
    info!("spawning orthographic camera bundle");

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 4.;

    commands
        .spawn_bundle(camera_bundle)
        .insert(Name::new("Camera"))
        .insert(Camera);
}

#[instrument(skip(rapier_config))]
fn set_up_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 36.;

    info!(gravity = ?rapier_config.gravity, scale = ?rapier_config.scale, "configured rapier");
}
