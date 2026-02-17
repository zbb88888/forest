use bevy::prelude::*;
use bevy::hierarchy::DespawnRecursiveExt;
use crate::states::GameState;
use crate::components::building::{
    BuildingType, BuildingStats, BuildingPosition, BuildingStatus, Inventory
};
use crate::systems::building::{place_building, upgrade_building, start_building, stop_building};

pub struct BuildingUIPlugin;

impl Plugin for BuildingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            toggle_building_panel,
            update_building_panel,
            handle_place_button,
            handle_upgrade_button,
            handle_start_stop_button,
        ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct BuildingPanel;

#[derive(Component)]
struct BuildingButton {
    building_type: BuildingType,
}

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct StartStopButton;

#[derive(Component)]
struct BuildingCostText;

#[derive(Resource, Default)]
struct BuildingUIState {
    is_visible: bool,
    selected_building: Option<BuildingType>,
    selected_building_entity: Option<Entity>,
}

fn toggle_building_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<BuildingUIState>,
    mut commands: Commands,
    existing_panel: Query<Entity, With<BuildingPanel>>,
) {
    // 按 B 键切换建筑面板
    if keyboard.just_pressed(KeyCode::KeyB) {
        ui_state.is_visible = !ui_state.is_visible;

        // 清除现有面板
        for entity in existing_panel.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if ui_state.is_visible {
            spawn_building_panel(&mut commands);
        }
    }
}

fn spawn_building_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(420.0),
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            BuildingPanel,
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("建筑建造"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // 资源信息
            parent.spawn((
                Text::new("资源:"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.7)),
            ));

            // 分隔线
            parent.spawn((
                Node {
                    height: Val::Px(2.0),
                    width: Val::Px(380.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            ));

            // 建筑列表标题
            parent.spawn((
                Text::new("可建造建筑:"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.9)),
            ));

            // 示例建筑按钮 - 能源收集器
            parent.spawn((
                Button {
                    width: Val::Px(380.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                BuildingButton {
                    building_type: BuildingType::EnergyCollector,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("能源收集器 - 50 能源, 20 金属"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    BuildingCostText,
                ));
            });

            // 示例建筑按钮 - 金属矿
            parent.spawn((
                Button {
                    width: Val::Px(380.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                BuildingButton {
                    building_type: BuildingType::MetalMine,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("金属矿 - 80 能源, 30 金属"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    BuildingCostText,
                ));
            });
        });
}

fn update_building_panel(
    ui_state: Res<BuildingUIState>,
    inventory: Res<Inventory>,
    mut cost_query: Query<&mut Text, With<BuildingCostText>>,
) {
    if !ui_state.is_visible {
        return;
    }

    // 更新建筑成本显示
    for mut text in cost_query.iter_mut() {
        // 这里可以根据选中的建筑更新成本显示
        // 简化处理，暂时不更新
    }
}

fn handle_place_button(
    ui_state: Res<BuildingUIState>,
    mut inventory: ResMut<Inventory>,
    world_map: Res<crate::resources::world::WorldMap>,
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &BuildingButton), Changed<Interaction>>,
) {
    if !ui_state.is_visible {
        return;
    }

    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // 简化处理：在固定位置放置建筑
            let tile_x = 5u32;
            let tile_y = 5u32;

            match place_building(
                &mut commands,
                button.building_type,
                tile_x,
                tile_y,
                &world_map,
                &mut inventory,
            ) {
                Ok(entity) => {
                    info!("成功放置建筑: {:?}", button.building_type);
                    // TODO: 记录选中的建筑实体
                }
                Err(e) => {
                    info!("放置建筑失败: {}", e);
                }
            }
        }
    }
}

fn handle_upgrade_button(
    ui_state: Res<BuildingUIState>,
    mut inventory: ResMut<Inventory>,
    mut building_query: Query<&mut crate::components::building::Building>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
) {
    if !ui_state.is_visible {
        return;
    }

    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(entity) = ui_state.selected_building_entity {
                match upgrade_building(entity, &mut inventory, &mut building_query) {
                    Ok(_) => {
                        info!("建筑升级成功");
                    }
                    Err(e) => {
                        info!("建筑升级失败: {}", e);
                    }
                }
            }
        }
    }
}

fn handle_start_stop_button(
    ui_state: Res<BuildingUIState>,
    mut building_query: Query<&mut crate::components::building::Building>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartStopButton>)>,
) {
    if !ui_state.is_visible {
        return;
    }

    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(entity) = ui_state.selected_building_entity {
                // 检查建筑当前状态
                if let Ok(building) = building_query.get(entity) {
                    if building.is_operational {
                        match stop_building(entity, &mut building_query) {
                            Ok(_) => info!("建筑已停止"),
                            Err(e) => info!("停止建筑失败: {}", e),
                        }
                    } else {
                        match start_building(entity, &mut building_query) {
                            Ok(_) => info!("建筑已启动"),
                            Err(e) => info!("启动建筑失败: {}", e),
                        }
                    }
                }
            }
        }
    }
}
