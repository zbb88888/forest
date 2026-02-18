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
use systems::enemy::EnemyPlugin;
use systems::enemy_spawn::EnemySpawnPlugin;
use systems::enemy_attack::EnemyAttackPlugin;
use systems::enemy_base::EnemyBasePlugin;
use systems::combat::CombatPlugin;
use systems::player_combat::PlayerCombatPlugin;
use systems::combat_effects::CombatEffectsPlugin;
use systems::defense_tower::DefenseTowerPlugin;
use systems::defense_wall::DefenseWallPlugin;
use systems::defense_range::DefenseRangePlugin;
use systems::quest_manager::QuestManagerPlugin;
use systems::quest_events::QuestEventsPlugin;
use systems::quest_generator::QuestGeneratorPlugin;
use systems::achievement_manager::AchievementManagerPlugin;
use systems::achievement_events::AchievementEventsPlugin;
use systems::achievement_generator::AchievementGeneratorPlugin;
use systems::save_manager::SaveManagerPlugin;
use systems::save_ui::SaveUIPlugin;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    let is_headless = env::var("HEADLESS").is_ok();

    let mut app = App::new();

    if is_headless {
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))));

        app.add_plugins(bevy::log::LogPlugin::default());
        app.add_plugins(bevy::state::app::StatesPlugin);

        println!("Running in HEADLESS mode (MinimalPlugins)");

        app.add_systems(Startup, start_game_headless);
    } else {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dark Forest".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }));

        app.add_plugins((MenuPlugin, HUDPlugin));

        println!("Running in GRAPHICS mode");
    }

    app.init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, systems::time::init_game_time)
        .add_systems(Startup, systems::lighting::init_lighting)
        .add_plugins(PlantUpgradePlugin)
        .add_plugins(PlantUpgradeUIPlugin)
        .add_plugins(CraftingPlugin)
        .add_plugins(CraftingUIPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(BuildingUIPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EnemySpawnPlugin)
        .add_plugins(EnemyAttackPlugin)
        .add_plugins(EnemyBasePlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(PlayerCombatPlugin)
        .add_plugins(CombatEffectsPlugin)
        .add_plugins(DefenseTowerPlugin)
        .add_plugins(DefenseWallPlugin)
        .add_plugins(DefenseRangePlugin)
        .add_plugins(QuestManagerPlugin)
        .add_plugins(QuestEventsPlugin)
        .add_plugins(QuestGeneratorPlugin)
        .add_plugins(AchievementManagerPlugin)
        .add_plugins(AchievementEventsPlugin)
        .add_plugins(AchievementGeneratorPlugin)
        .add_plugins(SaveManagerPlugin)
        .add_plugins(SaveUIPlugin)
        .add_systems(OnEnter(GameState::InGame), (systems::map::setup_map, systems::player::spawn_player).chain())
        .run();
}

fn setup(mut commands: Commands) {
    if std::env::var("HEADLESS").is_err() {
        commands.spawn((
            Camera2d,
            Transform::default(),
            GlobalTransform::default(),
        ));
    }
    println!("Dark Forest Initialized!");
}

fn start_game_headless(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame);
    println!("Headless mode: Auto-starting game...");
}