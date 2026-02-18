use bevy::prelude::*;

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
use systems::map::{init_map_assets, MapRenderAssets};
use systems::player::{init_player_assets, PlayerRenderAssets};
use systems::enemy_spawn::{EnemySpawnPlugin, init_enemy_assets, EnemyRenderAssets};

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    let layer = std::env::var("LAYER")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or(6);

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Dark Forest".into(),
            resolution: (1280, 720).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((MenuPlugin, HUDPlugin))
    .init_state::<GameState>()
    .add_systems(Startup, |mut next_state: ResMut<NextState<GameState>>| {
        next_state.set(GameState::InGame);
    })
    .add_systems(Startup, setup)
    .add_systems(Startup, init_render_assets)
    .add_systems(Startup, systems::time::init_game_time)
    .add_systems(Startup, systems::lighting::init_lighting);

    match layer {
        0 => {
            info!("Running Layer 0: Pure Map");
            app.add_plugins(systems::map::MapPlugin);
        }
        1 => {
            info!("Running Layer 1: Entity Spawning");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
        }
        2 => {
            info!("Running Layer 2: Entity Behavior");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
            app.add_plugins(systems::enemy::EnemyPlugin);
            app.add_plugins(systems::robot::RobotPlugin);
            app.add_plugins(systems::equipment::EquipmentPlugin);
        }
        3 => {
            info!("Running Layer 3: Combat System");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
            app.add_plugins(systems::enemy::EnemyPlugin);
            app.add_plugins(systems::robot::RobotPlugin);
            app.add_plugins(systems::equipment::EquipmentPlugin);
            app.add_plugins(systems::enemy_attack::EnemyAttackPlugin);
            app.add_plugins(systems::player_combat::PlayerCombatPlugin);
            app.add_plugins(systems::combat::CombatPlugin);
            app.add_plugins(systems::combat_effects::CombatEffectsPlugin);
            app.add_plugins(systems::defense_tower::DefenseTowerPlugin);
            app.add_plugins(systems::defense_wall::DefenseWallPlugin);
            app.add_plugins(systems::defense_range::DefenseRangePlugin);
        }
        4 => {
            info!("Running Layer 4: Production & Building");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
            app.add_plugins(systems::enemy::EnemyPlugin);
            app.add_plugins(systems::robot::RobotPlugin);
            app.add_plugins(systems::equipment::EquipmentPlugin);
            app.add_plugins(systems::enemy_attack::EnemyAttackPlugin);
            app.add_plugins(systems::player_combat::PlayerCombatPlugin);
            app.add_plugins(systems::combat::CombatPlugin);
            app.add_plugins(systems::combat_effects::CombatEffectsPlugin);
            app.add_plugins(systems::defense_tower::DefenseTowerPlugin);
            app.add_plugins(systems::defense_wall::DefenseWallPlugin);
            app.add_plugins(systems::defense_range::DefenseRangePlugin);
            app.add_plugins(systems::plant_upgrade::PlantUpgradePlugin);
            app.add_plugins(PlantUpgradeUIPlugin);
            app.add_plugins(systems::crafting::CraftingPlugin);
            app.add_plugins(CraftingUIPlugin);
            app.add_plugins(systems::building::BuildingPlugin);
            app.add_plugins(BuildingUIPlugin);
        }
        50 => {
            info!("Running Layer 50: Quest & Achievement");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
            app.add_plugins(systems::enemy::EnemyPlugin);
            app.add_plugins(systems::robot::RobotPlugin);
            app.add_plugins(systems::equipment::EquipmentPlugin);
            app.add_plugins(systems::enemy_attack::EnemyAttackPlugin);
            app.add_plugins(systems::player_combat::PlayerCombatPlugin);
            app.add_plugins(systems::combat::CombatPlugin);
            app.add_plugins(systems::combat_effects::CombatEffectsPlugin);
            app.add_plugins(systems::defense_tower::DefenseTowerPlugin);
            app.add_plugins(systems::defense_wall::DefenseWallPlugin);
            app.add_plugins(systems::defense_range::DefenseRangePlugin);
            app.add_plugins(systems::plant_upgrade::PlantUpgradePlugin);
            app.add_plugins(PlantUpgradeUIPlugin);
            app.add_plugins(systems::crafting::CraftingPlugin);
            app.add_plugins(CraftingUIPlugin);
            app.add_plugins(systems::building::BuildingPlugin);
            app.add_plugins(BuildingUIPlugin);
            app.add_plugins(systems::quest_manager::QuestManagerPlugin);
            app.add_plugins(systems::quest_events::QuestEventsPlugin);
            app.add_plugins(systems::quest_generator::QuestGeneratorPlugin);
            app.add_plugins(systems::achievement_manager::AchievementManagerPlugin);
            app.add_plugins(systems::achievement_events::AchievementEventsPlugin);
            app.add_plugins(systems::achievement_generator::AchievementGeneratorPlugin);
        }
        999 => {
            info!("Running Layer 999: Full System (Save/Load)");
            app.add_plugins(EnemySpawnPlugin);
            app.add_plugins(systems::enemy_base::EnemyBasePlugin);
            app.add_plugins(systems::plant::PlantPlugin);
            app.add_plugins(systems::enemy::EnemyPlugin);
            app.add_plugins(systems::robot::RobotPlugin);
            app.add_plugins(systems::equipment::EquipmentPlugin);
            app.add_plugins(systems::enemy_attack::EnemyAttackPlugin);
            app.add_plugins(systems::player_combat::PlayerCombatPlugin);
            app.add_plugins(systems::combat::CombatPlugin);
            app.add_plugins(systems::combat_effects::CombatEffectsPlugin);
            app.add_plugins(systems::defense_tower::DefenseTowerPlugin);
            app.add_plugins(systems::defense_wall::DefenseWallPlugin);
            app.add_plugins(systems::defense_range::DefenseRangePlugin);
            app.add_plugins(systems::plant_upgrade::PlantUpgradePlugin);
            app.add_plugins(PlantUpgradeUIPlugin);
            app.add_plugins(systems::crafting::CraftingPlugin);
            app.add_plugins(CraftingUIPlugin);
            app.add_plugins(systems::building::BuildingPlugin);
            app.add_plugins(BuildingUIPlugin);
            app.add_plugins(systems::quest_manager::QuestManagerPlugin);
            app.add_plugins(systems::quest_events::QuestEventsPlugin);
            app.add_plugins(systems::quest_generator::QuestGeneratorPlugin);
            app.add_plugins(systems::achievement_manager::AchievementManagerPlugin);
            app.add_plugins(systems::achievement_events::AchievementEventsPlugin);
            app.add_plugins(systems::achievement_generator::AchievementGeneratorPlugin);
            app.add_plugins(systems::save_manager::SaveManagerPlugin);
            app.add_plugins(systems::save_ui::SaveUIPlugin);
        }
        _ => {
            info!("Running Layer {}: Unknown layer, using default (Sys0)", layer);
        }
    }

    app.add_systems(OnEnter(GameState::InGame), systems::player::spawn_player)
       .add_systems(Update, systems::lighting::update_lighting)
       .run();
}

fn init_render_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let map_assets = init_map_assets(&mut meshes, &mut materials);
    commands.insert_resource(map_assets);

    let player_assets = init_player_assets(&mut meshes, &mut materials);
    commands.insert_resource(player_assets);

    let enemy_assets = init_enemy_assets(&mut meshes, &mut materials);
    commands.insert_resource(enemy_assets);

    info!("Render assets initialized");
}

fn setup(mut commands: Commands) {
    let map_width = 20;
    let map_height = 20;
    let tile_size = 32.0;

    let offset_x = -(map_width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(map_height as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let center_x = offset_x + (map_width / 2) as f32 * tile_size;
    let center_y = offset_y + (map_height / 2) as f32 * tile_size;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(center_x, center_y, 100.0),
        GlobalTransform::default(),
    ));
    println!("Dark Forest Initialized! Camera at ({}, {})", center_x, center_y);
}