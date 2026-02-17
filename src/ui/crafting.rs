use bevy::prelude::*;
use crate::states::GameState;
use crate::components::crafting::{RecipeBook, Inventory, MaterialType};
use crate::components::equipment::{EquipmentType, EquipmentRarity};
use crate::systems::crafting::{start_crafting, upgrade_equipment};

pub struct CraftingUIPlugin;

impl Plugin for CraftingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            toggle_crafting_panel,
            update_crafting_panel,
            handle_recipe_button,
            handle_upgrade_button,
        ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct CraftingPanel;

#[derive(Component)]
struct RecipeButton {
    equipment_type: EquipmentType,
    rarity: EquipmentRarity,
}

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct MaterialText {
    material_type: MaterialType,
}

#[derive(Resource, Default)]
struct CraftingUIState {
    is_visible: bool,
    selected_equipment: Option<Entity>,
}

fn toggle_crafting_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<CraftingUIState>,
    mut commands: Commands,
    existing_panel: Query<Entity, With<CraftingPanel>>,
) {
    // 按 C 键切换制造面板
    if keyboard.just_pressed(KeyCode::KeyC) {
        ui_state.is_visible = !ui_state.is_visible;

        // 清除现有面板
        for entity in existing_panel.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if ui_state.is_visible {
            spawn_crafting_panel(&mut commands);
        }
    }
}

fn spawn_crafting_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(10.0),
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            CraftingPanel,
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("装备制造"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // 材料信息
            parent.spawn((
                Text::new("材料:"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.7)),
            ));

            // 能源
            parent.spawn((
                Text::new("能源: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                MaterialText {
                    material_type: MaterialType::Energy,
                },
            ));

            // 金属
            parent.spawn((
                Text::new("金属: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                MaterialText {
                    material_type: MaterialType::Metal,
                },
            ));

            // 土壤
            parent.spawn((
                Text::new("土壤: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.4, 0.2)),
                MaterialText {
                    material_type: MaterialType::Soil,
                },
            ));

            // 水晶
            parent.spawn((
                Text::new("水晶: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.8, 0.9)),
                MaterialText {
                    material_type: MaterialType::Crystal,
                },
            ));

            // 有机物
            parent.spawn((
                Text::new("有机物: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.9, 0.3)),
                MaterialText {
                    material_type: MaterialType::Organic,
                },
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

            // 配方列表
            parent.spawn((
                Text::new("可制造装备:"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.9)),
            ));

            // 示例配方按钮
            parent.spawn((
                Button {
                    width: Val::Px(380.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
                RecipeButton {
                    equipment_type: EquipmentType::LaserGun,
                    rarity: EquipmentRarity::Common,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("激光枪 (普通) - 50 能源, 10 金属"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn update_crafting_panel(
    ui_state: Res<CraftingUIState>,
    inventory: Res<Inventory>,
    recipe_book: Res<RecipeBook>,
    mut material_query: Query<(&mut Text, &MaterialText)>,
) {
    if !ui_state.is_visible {
        return;
    }

    // 更新材料显示
    for (mut text, material) in material_query.iter_mut() {
        let amount = inventory.get_material(material.material_type);
        text.0 = format!("{}: {}", material.material_type.name(), amount);
    }
}

fn handle_recipe_button(
    ui_state: Res<CraftingUIState>,
    mut inventory: ResMut<Inventory>,
    recipe_book: Res<RecipeBook>,
    quality_control: Res<crate::components::crafting::QualityControl>,
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &RecipeButton), Changed<Interaction>>,
) {
    if !ui_state.is_visible {
        return;
    }

    for (interaction, recipe_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // 尝试制造装备
            match start_crafting(
                recipe_button.equipment_type,
                recipe_button.rarity,
                &mut inventory,
                &recipe_book,
                &quality_control,
                &mut rand::thread_rng(),
            ) {
                Ok((equipment, quality)) => {
                    info!(
                        "成功制造 {:?} ({:?}), 品质: {:.2}",
                        recipe_button.equipment_type,
                        equipment.rarity,
                        quality
                    );
                    // TODO: 将装备添加到玩家背包
                }
                Err(e) => {
                    info!("制造失败: {}", e);
                }
            }
        }
    }
}

fn handle_upgrade_button(
    ui_state: Res<CraftingUIState>,
    mut inventory: ResMut<Inventory>,
    upgrade_optimization: Res<crate::components::crafting::UpgradeOptimization>,
    mut equipment_query: Query<&mut crate::components::equipment::Equipment>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
) {
    if !ui_state.is_visible {
        return;
    }

    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // TODO: 获取选中的装备进行升级
            info!("升级装备功能待实现");
        }
    }
}
