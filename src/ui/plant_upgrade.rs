use bevy::prelude::*;
use crate::states::GameState;
use crate::components::plant_upgrade::{PlantUpgrade, PlantLevel, PlantVarietyTree, PlantHarvestStats};
use crate::components::plant::Plant;
use crate::components::resource::Inventory;

pub struct PlantUpgradeUIPlugin;

impl Plugin for PlantUpgradeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            toggle_plant_upgrade_panel.run_if(in_state(GameState::InGame)),
            update_plant_upgrade_panel.run_if(in_state(GameState::InGame)),
            handle_upgrade_button.run_if(in_state(GameState::InGame)),
            handle_unlock_button.run_if(in_state(GameState::InGame)),
        ));
    }
}

#[derive(Component)]
struct PlantUpgradePanel;

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct UnlockButton;

#[derive(Component)]
struct PlantLevelText;

#[derive(Component)]
struct UpgradeCostText;

#[derive(Component)]
struct UnlockCostText;

#[derive(Component)]
struct PlantVarietyText;

#[derive(Component)]
struct HarvestCountText;

#[derive(Resource, Default)]
struct PlantUpgradeUIState {
    is_visible: bool,
    selected_plant: Option<Entity>,
}

fn toggle_plant_upgrade_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<PlantUpgradeUIState>,
    mut commands: Commands,
    existing_panel: Query<Entity, With<PlantUpgradePanel>>,
) {
    // 按 U 键切换面板显示
    if keyboard.just_pressed(KeyCode::KeyU) {
        ui_state.is_visible = !ui_state.is_visible;

        // 清除现有面板
        for entity in existing_panel.iter() {
            commands.entity(entity).despawn();
        }

        if ui_state.is_visible {
            spawn_plant_upgrade_panel(&mut commands);
        }
    }
}

fn spawn_plant_upgrade_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                right: Val::Px(10.0),
                width: Val::Px(300.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Style {
                gap: Val::Px(10.0),
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            PlantUpgradePanel,
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("植物升级"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // 植物等级信息
            parent.spawn((
                Text::new("等级: 1"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.7)),
                PlantLevelText,
            ));

            // 升级按钮
            parent.spawn((
                Button {
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.5, 0.2)),
                UpgradeButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("升级 (50 能源)"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    UpgradeCostText,
                ));
            });

            // 分隔线
            parent.spawn((
                Node {
                    height: Val::Px(2.0),
                    width: Val::Px(280.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            ));

            // 植物品种信息
            parent.spawn((
                Text::new("已解锁品种: 草"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.9)),
                PlantVarietyText,
            ));

            // 收获统计
            parent.spawn((
                Text::new("总收获次数: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.9, 0.7)),
                HarvestCountText,
            ));

            // 解锁按钮
            parent.spawn((
                Button {
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                UnlockButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("解锁新品种 (100 能源)"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    UnlockCostText,
                ));
            });
        });
}

fn update_plant_upgrade_panel(
    ui_state: Res<PlantUpgradeUIState>,
    plant_query: Query<&PlantUpgrade>,
    variety_tree: Res<PlantVarietyTree>,
    harvest_stats: Res<PlantHarvestStats>,
    inventory: Res<Inventory>,
    mut level_query: Query<&mut Text, With<PlantLevelText>>,
    mut variety_query: Query<&mut Text, With<PlantVarietyText>>,
    mut harvest_query: Query<&mut Text, With<HarvestCountText>>,
) {
    if !ui_state.is_visible {
        return;
    }

    // 更新植物等级显示
    if let Some(mut text) = level_query.iter_mut().next() {
        if let Some(upgrade) = ui_state.selected_plant.and_then(|e| plant_query.get(e).ok()) {
            text.0 = format!("等级: {}", upgrade.level.value());
        } else {
            text.0 = "等级: 1".to_string();
        }
    }

    // 更新品种信息
    if let Some(mut text) = variety_query.iter_mut().next() {
        let varieties: Vec<String> = variety_tree.unlocked_varieties
            .iter()
            .map(|v| format!("{:?}", v))
            .collect();
        text.0 = format!("已解锁品种: {}", varieties.join(", "));
    }

    // 更新收获统计
    if let Some(mut text) = harvest_query.iter_mut().next() {
        text.0 = format!("总收获次数: {}", harvest_stats.get_total_harvests());
    }
}

fn handle_upgrade_button(
    ui_state: Res<PlantUpgradeUIState>,
    mut inventory: ResMut<Inventory>,
    mut plant_query: Query<&mut PlantUpgrade>,
    mut upgrade_cost_query: Query<&mut Text, With<UpgradeCostText>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
) {
    if !ui_state.is_visible {
        return;
    }

    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(selected) = ui_state.selected_plant {
                if let Ok(mut upgrade) = plant_query.get_mut(selected) {
                    let cost = upgrade.get_upgrade_cost();
                    if cost > 0 && inventory.energy >= cost {
                        inventory.energy -= cost;
                        upgrade.apply_upgrade();
                        info!("Plant upgraded to level {:?}", upgrade.level);
                    }
                }
            }
        }
    }

    // 更新升级按钮文本
    if let Some(selected) = ui_state.selected_plant {
        if let Ok(upgrade) = plant_query.get(selected) {
            if let Some(mut text) = upgrade_cost_query.iter_mut().next() {
                let cost = upgrade.get_upgrade_cost();
                if cost > 0 {
                    text.0 = format!("升级 ({} 能源)", cost);
                } else {
                    text.0 = "已达最高级".to_string();
                }
            }
        }
    }
}

fn handle_unlock_button(
    ui_state: Res<PlantUpgradeUIState>,
    mut inventory: ResMut<Inventory>,
    mut variety_tree: ResMut<PlantVarietyTree>,
    harvest_stats: Res<PlantHarvestStats>,
    mut unlock_cost_query: Query<&mut Text, With<UnlockCostText>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UnlockButton>)>,
) {
    if !ui_state.is_visible {
        return;
    }

    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // 简化处理：解锁第一个未解锁的品种
            for plant_type in [crate::components::plant::PlantType::Bush, 
                             crate::components::plant::PlantType::Flower,
                             crate::components::plant::PlantType::Tree,
                             crate::components::plant::PlantType::EnergyFlower] {
                if !variety_tree.is_unlocked(plant_type) {
                    if let Some(condition) = variety_tree.unlock_conditions.get(&plant_type) {
                        if inventory.energy >= condition.energy_cost {
                            inventory.energy -= condition.energy_cost;
                            variety_tree.unlock(plant_type);
                            info!("Unlocked plant variety: {:?}", plant_type);
                            break;
                        }
                    }
                }
            }
        }
    }

    // 更新解锁按钮文本
    if let Some(mut text) = unlock_cost_query.iter_mut().next() {
        // 查找第一个未解锁的品种
        for plant_type in [crate::components::plant::PlantType::Bush, 
                         crate::components::plant::PlantType::Flower,
                         crate::components::plant::PlantType::Tree,
                         crate::components::plant::PlantType::EnergyFlower] {
            if !variety_tree.is_unlocked(plant_type) {
                if let Some(condition) = variety_tree.unlock_conditions.get(&plant_type) {
                    text.0 = format!("解锁 {:?} ({} 能源)", plant_type, condition.energy_cost);
                    break;
                }
            }
        }
    }
}
