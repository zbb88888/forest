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
use systems::plant_upgrade::PlantUpgradePlugin;
use systems::crafting::CraftingPlugin;
use systems::building::BuildingPlugin;
use systems::enemy::EnemyPlugin;
use systems::enemy_spawn::{EnemySpawnPlugin, init_enemy_assets, EnemyRenderAssets};
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
use systems::map::{init_map_assets, MapRenderAssets};
use systems::player::{init_player_assets, PlayerRenderAssets};

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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