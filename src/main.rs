use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use std::time::Duration;
use std::env;

mod states;
mod systems;
mod components;
mod resources;
mod ui;
mod utils;

use states::GameState;
use ui::menu::MenuPlugin;
use ui::hud::HUDPlugin;
use ui::plant_upgrade::PlantUpgradeUIPlugin;
use ui::crafting::CraftingUIPlugin;
use ui::building::BuildingUIPlugin;
use systems::plant_upgrade::PlantUpgradePlugin;
use systems::crafting::CraftingPlugin;
use systems::building::BuildingPlugin;

fn main() {
    let is_headless = env::var("HEADLESS").is_ok();

    let mut app = App::new();

    if is_headless {
        // Headless mode: Use MinimalPlugins to avoid any rendering dependencies
        // MinimalPlugins already includes ScheduleRunnerPlugin, so we configure it via set()
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))));

        // Add necessary non-rendering plugins
        app.add_plugins(bevy::log::LogPlugin::default());
        app.add_plugins(bevy::state::app::StatesPlugin); // Required for state management

        println!("Running in HEADLESS mode (MinimalPlugins)");

        // In headless mode, auto-start the game
        app.add_systems(Startup, start_game_headless);
    } else {
        // Normal mode: Window + Rendering
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dark Forest".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }));

        // Add UI plugins only in graphics mode
        app.add_plugins((MenuPlugin, HUDPlugin));

        println!("Running in GRAPHICS mode");
    }

    app.init_state::<GameState>()
        .add_systems(Startup, setup)
        // Initialize game time and lighting
        .add_systems(Startup, systems::time::init_game_time)
        .add_systems(Startup, systems::lighting::init_lighting)
        // Add plant upgrade system
        .add_plugins(PlantUpgradePlugin)
        .add_plugins(PlantUpgradeUIPlugin)
        // Add crafting system
        .add_plugins(CraftingPlugin)
        .add_plugins(CraftingUIPlugin)
        // Add building system
        .add_plugins(BuildingPlugin)
        .add_plugins(BuildingUIPlugin)
        // Move map and player setup to InGame state
        .add_systems(OnEnter(GameState::InGame), (systems::map::setup_map, systems::player::spawn_player).chain())
        // Run systems only in InGame state
        .add_systems(Update, (
            systems::time::update_time,
            systems::lighting::update_lighting,
            systems::player::move_player_randomly,
            systems::plant::plant_seed,
            systems::plant::grow_plants,
            systems::plant::harvest_plants,
            systems::plant::plant_decay,
            systems::robot::spawn_robot,
            systems::robot::robot_ai_system,
            systems::equipment::spawn_random_equipment,
            systems::equipment::pickup_equipment,
            systems::equipment::upgrade_equipment,
            systems::equipment::display_equipment_info,
        ).run_if(in_state(GameState::InGame)))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera is only needed in graphics mode
    if std::env::var("HEADLESS").is_err() {
        commands.spawn(Camera2d::default());
    }
    println!("Dark Forest Initialized!");
}

fn start_game_headless(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame);
    println!("Headless mode: Auto-starting game...");
}
