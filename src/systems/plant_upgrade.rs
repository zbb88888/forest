use bevy::prelude::*;
use crate::components::plant_upgrade::{PlantUpgrade, PlantVarietyTree, PlantHarvestStats, PlantLevel};
use crate::components::plant::{Plant, PlantType};
use crate::components::resource::Inventory;

/// 植物升级系统插件
pub struct PlantUpgradePlugin;

impl Plugin for PlantUpgradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantVarietyTree>()
            .init_resource::<PlantHarvestStats>()
            .add_systems(Update, (
                check_upgrade_conditions.run_if(in_state(crate::states::GameState::InGame)),
                apply_upgrade_effects.run_if(in_state(crate::states::GameState::InGame)),
            ));
    }
}

/// 检查升级条件
fn check_upgrade_conditions(
    plant_query: Query<(&Plant, &PlantUpgrade)>,
    inventory: Res<Inventory>,
    harvest_stats: Res<PlantHarvestStats>,
) {
    for (plant, upgrade) in plant_query.iter() {
        // 检查是否可以升级
        if upgrade.can_upgrade() {
            let cost = upgrade.get_upgrade_cost();
            if inventory.energy >= cost {
                // 可以升级
                info!("Plant {:?} can be upgraded for {} energy", plant.plant_type, cost);
            }
        }
    }
}

/// 应用升级效果
fn apply_upgrade_effects(
    mut plant_query: Query<(&mut Plant, &PlantUpgrade)>,
) {
    for (mut plant, upgrade) in plant_query.iter_mut() {
        // 应用生长速度加成
        let base_growth_rate = plant.plant_type.base_growth_rate();
        plant.energy_output = upgrade.calculate_output(base_growth_rate * plant.plant_type.energy_multiplier());

        // 应用健康度加成
        if plant.health < 1.0 {
            let max_health = upgrade.calculate_health(1.0);
            plant.health = (plant.health + 0.01 * upgrade.level.multiplier()).min(max_health);
        }
    }
}

/// 升级植物
pub fn upgrade_plant(
    plant_entity: Entity,
    inventory: &mut Inventory,
    plant_query: &mut Query<&mut PlantUpgrade>,
) -> Result<bool, String> {
    // 检查是否有足够的能源
    let Ok(mut upgrade) = plant_query.get_mut(plant_entity) else {
        return Err("Plant upgrade component not found".to_string());
    };

    let cost = upgrade.get_upgrade_cost();
    if cost == 0 {
        return Err("Plant is already at max level".to_string());
    }

    if inventory.energy < cost {
        return Err(format!("Not enough energy. Need {}, have {}", cost, inventory.energy));
    }

    // 扣除能源
    inventory.energy -= cost;

    // 应用升级
    if upgrade.apply_upgrade() {
        info!("Plant upgraded to level {:?}", upgrade.level);
        Ok(true)
    } else {
        // 升级失败，返还能源
        inventory.energy += cost;
        Err("Failed to upgrade plant".to_string())
    }
}

/// 解锁植物品种
pub fn unlock_plant_variety(
    plant_type: PlantType,
    inventory: &mut Inventory,
    harvest_stats: &PlantHarvestStats,
    variety_tree: &mut PlantVarietyTree,
) -> Result<bool, String> {
    // 检查是否已解锁
    if variety_tree.is_unlocked(plant_type) {
        return Err("Plant variety already unlocked".to_string());
    }

    // 获取解锁条件
    let energy_cost = variety_tree.unlock_conditions.get(&plant_type)
        .ok_or_else(|| "Unlock condition not found".to_string())?
        .energy_cost;
    let required_level = variety_tree.unlock_conditions.get(&plant_type)
        .ok_or_else(|| "Unlock condition not found".to_string())?
        .required_level;
    let required_harvests = variety_tree.unlock_conditions.get(&plant_type)
        .ok_or_else(|| "Unlock condition not found".to_string())?
        .required_harvests;

    // 检查是否满足条件
    // 这里简化处理，实际应该检查玩家最高等级
    let current_level = PlantLevel::Level1; // TODO: 从玩家数据获取
    let harvest_count = harvest_stats.get_harvest_count(plant_type);

    if !variety_tree.can_unlock(plant_type, current_level, harvest_count, inventory.energy) {
        return Err(format!(
            "Cannot unlock {:?}. Need level {:?}, {} harvests, and {} energy",
            plant_type,
            required_level,
            required_harvests,
            energy_cost
        ));
    }

    // 扣除能源
    inventory.energy -= energy_cost;

    // 解锁品种
    if variety_tree.unlock(plant_type) {
        info!("Unlocked plant variety: {:?}", plant_type);
        Ok(true)
    } else {
        // 解锁失败，返还能源
        inventory.energy += energy_cost;
        Err("Failed to unlock plant variety".to_string())
    }
}
