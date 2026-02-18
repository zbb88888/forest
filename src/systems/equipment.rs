use bevy::prelude::*;
use crate::components::equipment::{Equipment, EquipmentType, EquipmentRarity, EquipmentBar};
use crate::components::player::Player;
use crate::components::resource::Inventory;
use rand::Rng;

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_random_equipment, pickup_equipment));
    }
}

/// 生成随机装备
pub fn spawn_random_equipment(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, (With<Player>, Without<Equipment>)>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        let Ok(player_transform) = player_query.single() else { return; };
        let position = player_transform.translation;
        let equipment = generate_random_equipment();

        commands.spawn((
            Sprite {
                color: equipment.rarity.color(),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 1.0),
            equipment.clone(),
        ));

        info!("生成了装备: {} ({})", equipment.equipment_type.name(), format_rarity(equipment.rarity));
    }
}

/// 生成随机装备
fn generate_random_equipment() -> Equipment {
    let mut rng = rand::thread_rng();

    // 随机选择装备类型
    let equipment_types = [
        EquipmentType::LaserGun,
        EquipmentType::PlasmaCannon,
        EquipmentType::EMPBlaster,
        EquipmentType::Railgun,
        EquipmentType::LightArmor,
        EquipmentType::HeavyArmor,
        EquipmentType::EnergyShield,
        EquipmentType::SolarPanel,
        EquipmentType::BatteryPack,
        EquipmentType::TargetingSystem,
    ];

    let equipment_type = equipment_types[rng.gen_range(0..equipment_types.len())];

    // 随机稀有度（加权随机）
    let rarity_roll = rng.gen_range(0..100);
    let rarity = match rarity_roll {
        0..=60 => EquipmentRarity::Common,      // 60%
        61..=85 => EquipmentRarity::Uncommon,    // 25%
        86..=95 => EquipmentRarity::Rare,        // 10%
        96..=99 => EquipmentRarity::Legendary,   // 4%
        _ => EquipmentRarity::Mythic,            // 1%
    };

    Equipment::new(equipment_type, rarity)
}

/// 格式化稀有度名称
fn format_rarity(rarity: EquipmentRarity) -> &'static str {
    match rarity {
        EquipmentRarity::Common => "普通",
        EquipmentRarity::Uncommon => "稀有",
        EquipmentRarity::Rare => "史诗",
        EquipmentRarity::Legendary => "传说",
        EquipmentRarity::Mythic => "神话",
    }
}

/// 拾取装备
pub fn pickup_equipment(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, (With<Player>, Without<Equipment>)>,
    mut equipment_query: Query<(Entity, &Equipment, &Transform), Without<Player>>,
    mut player_equipment_bar: Query<&mut EquipmentBar, With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return; };
    let pickup_range = 50.0;
    let player_pos = player_transform.translation.truncate();

    for (entity, equipment, transform) in equipment_query.iter_mut() {
        let distance = transform.translation.truncate().distance(player_pos);

        if distance < pickup_range {
            let slot = equipment.equipment_type.slot();

            let Ok(mut equipment_bar) = player_equipment_bar.single_mut() else { continue; };
            // 如果该槽位已有装备，先卸下
            if let Some(old_equipment) = equipment_bar.unequip(slot) {
                commands.entity(old_equipment).despawn();
            }

            // 装备新物品
            equipment_bar.equip(slot, entity);
            info!("装备了: {} ({})", equipment.equipment_type.name(), format_rarity(equipment.rarity));

            break;
        }
    }
}

/// 升级装备
pub fn upgrade_equipment(
    _commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut equipment_query: Query<(Entity, &mut Equipment)>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyU) {
        return;
    }

    let upgrade_cost = 50;

    let Ok(mut inventory) = player_inventory.single_mut() else { return; };
    if inventory.energy >= upgrade_cost {
        // 升级第一个找到的装备
        for (_entity, mut equipment) in equipment_query.iter_mut() {
            if equipment.level < 10 {
                equipment.upgrade();
                inventory.energy -= upgrade_cost;
                info!("升级了装备: {} (+{} 级)", equipment.equipment_type.name(), equipment.level);
                break;
            }
        }
    } else {
        info!("能量不足，无法升级装备");
    }
}

/// 显示装备信息
pub fn display_equipment_info(
    equipment_query: Query<&Equipment>,
    player_equipment_bar: Query<&EquipmentBar, With<Player>>,
) {
    let Ok(equipment_bar) = player_equipment_bar.single() else { return; };
    let total_stats = equipment_bar.total_stats(&equipment_query);

    info!("=== 装备属性 ===");
    info!("伤害: {:.1}", total_stats.damage);
    info!("攻击速度: {:.1}", total_stats.attack_speed);
    info!("防御: {:.1}", total_stats.defense);
    info!("能量加成: {:.1}", total_stats.energy_bonus);
    info!("暴击率: {:.1}%", total_stats.crit_chance * 100.0);
    info!("暴击倍率: {:.1}x", total_stats.crit_multiplier);
    info!("===============");
}
