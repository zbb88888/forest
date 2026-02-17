use bevy::prelude::*;
use crate::components::crafting::{
    CraftingRecipe, RecipeBook, QualityControl, UpgradeOptimization,
    MaterialType
};
use crate::components::equipment::{Equipment, EquipmentStats, EquipmentType, EquipmentRarity};
use crate::components::resource::Inventory;
use rand::Rng;

/// 装备制造系统插件
pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RecipeBook>()
            .init_resource::<Inventory>()
            .init_resource::<QualityControl>()
            .init_resource::<UpgradeOptimization>()
            .add_systems(Startup, initialize_recipes)
            .add_systems(Update, (
                update_crafting_status.run_if(in_state(crate::states::GameState::InGame)),
                check_crafting_conditions.run_if(in_state(crate::states::GameState::InGame)),
            ));
    }
}

/// 初始化配方
fn initialize_recipes(mut recipe_book: ResMut<RecipeBook>) {
    use crate::components::crafting::MaterialRequirement;

    // 激光枪配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::LaserGun,
        EquipmentRarity::Common,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 10,
            },
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 20,
            },
        ],
        50,
        5.0,
    ));

    // 等离子炮配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::PlasmaCannon,
        EquipmentRarity::Uncommon,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 20,
            },
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 40,
            },
            MaterialRequirement {
                material_type: MaterialType::Crystal,
                amount: 5,
            },
        ],
        100,
        10.0,
    ));

    // 轨道炮配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::Railgun,
        EquipmentRarity::Rare,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 30,
            },
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 60,
            },
            MaterialRequirement {
                material_type: MaterialType::Crystal,
                amount: 10,
            },
        ],
        200,
        15.0,
    ));

    // 轻型护甲配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::LightArmor,
        EquipmentRarity::Common,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 15,
            },
            MaterialRequirement {
                material_type: MaterialType::Organic,
                amount: 10,
            },
        ],
        40,
        5.0,
    ));

    // 重型护甲配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::HeavyArmor,
        EquipmentRarity::Uncommon,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 30,
            },
            MaterialRequirement {
                material_type: MaterialType::Organic,
                amount: 20,
            },
        ],
        80,
        10.0,
    ));

    // 能量护盾配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::EnergyShield,
        EquipmentRarity::Rare,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Crystal,
                amount: 15,
            },
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 50,
            },
        ],
        150,
        12.0,
    ));

    // 太阳能板配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::SolarPanel,
        EquipmentRarity::Common,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Crystal,
                amount: 5,
            },
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 10,
            },
        ],
        30,
        5.0,
    ));

    // 电池包配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::BatteryPack,
        EquipmentRarity::Uncommon,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 30,
            },
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 15,
            },
        ],
        60,
        8.0,
    ));

    // 瞄准系统配方
    recipe_book.add_recipe(CraftingRecipe::new(
        EquipmentType::TargetingSystem,
        EquipmentRarity::Rare,
        vec![
            MaterialRequirement {
                material_type: MaterialType::Crystal,
                amount: 10,
            },
            MaterialRequirement {
                material_type: MaterialType::Metal,
                amount: 20,
            },
            MaterialRequirement {
                material_type: MaterialType::Energy,
                amount: 40,
            },
        ],
        120,
        10.0,
    ));
}

/// 更新制造状态
fn update_crafting_status(
    time: Res<Time>,
    mut crafting_query: Query<&mut CraftingStatus>,
) {
    for mut status in crafting_query.iter_mut() {
        if status.is_crafting {
            status.progress += time.delta_secs();

            if status.progress >= status.total_time {
                status.is_crafting = false;
                status.progress = 0.0;
                status.completed = true;
            }
        }
    }
}

/// 检查制造条件
fn check_crafting_conditions(
    recipe_book: Res<RecipeBook>,
    inventory: Res<Inventory>,
) {
    let craftable = recipe_book.get_craftable_recipes(&inventory);
    if !craftable.is_empty() {
        info!("可制造的装备数量: {}", craftable.len());
    }
}

/// 制造状态组件
#[derive(Component, Clone, Debug)]
pub struct CraftingStatus {
    pub is_crafting: bool,
    pub progress: f32,
    pub total_time: f32,
    pub completed: bool,
}

impl Default for CraftingStatus {
    fn default() -> Self {
        Self {
            is_crafting: false,
            progress: 0.0,
            total_time: 0.0,
            completed: false,
        }
    }
}

/// 开始制造装备
pub fn start_crafting(
    equipment_type: EquipmentType,
    rarity: EquipmentRarity,
    inventory: &mut Inventory,
    recipe_book: &RecipeBook,
    quality_control: &QualityControl,
    rng: &mut impl Rng,
) -> Result<(Equipment, f32), String> {
    // 查找配方
    let recipe = recipe_book
        .recipes
        .iter()
        .find(|r| r.equipment_type == equipment_type && r.rarity == rarity)
        .ok_or_else(|| "配方未找到".to_string())?;

    // 检查配方是否解锁
    if !recipe.unlocked {
        return Err("配方未解锁".to_string());
    }

    // 检查能源是否足够
    if inventory.energy < recipe.energy_cost {
        return Err(format!("能源不足，需要 {}", recipe.energy_cost));
    }

    // 检查材料是否足够
    for material in &recipe.materials {
        let amount = inventory.get_material(material.material_type);
        if amount < material.amount {
            return Err(format!(
                "{} 不足，需要 {}",
                material.material_type.name(),
                material.amount
            ));
        }
    }

    // 扣除能源和材料
    inventory.energy -= recipe.energy_cost;
    for material in &recipe.materials {
        inventory.remove_material(material.material_type, material.amount);
    }

    // 计算品质
    let quality = quality_control.calculate_quality(rng);
    let actual_rarity = quality_control.calculate_rarity(quality);

    // 创建装备
    let mut equipment = Equipment::new(equipment_type, actual_rarity);

    // 根据品质调整属性
    let quality_multiplier = quality;
    let base_stats = equipment_type.base_stats();
    equipment.stats = EquipmentStats {
        damage: base_stats.damage * quality_multiplier,
        attack_speed: base_stats.attack_speed,
        defense: base_stats.defense * quality_multiplier,
        energy_bonus: base_stats.energy_bonus * quality_multiplier,
        crit_chance: base_stats.crit_chance * quality_multiplier.min(1.5),
        crit_multiplier: base_stats.crit_multiplier,
    };

    info!(
        "制造了 {:?} ({:?}), 品质: {:.2}",
        equipment_type, actual_rarity, quality
    );

    Ok((equipment, quality))
}

/// 升级装备
pub fn upgrade_equipment(
    equipment: &mut Equipment,
    inventory: &mut Inventory,
    upgrade_optimization: &UpgradeOptimization,
    rng: &mut impl Rng,
) -> Result<bool, String> {
    // 检查是否达到最大等级
    if equipment.level >= upgrade_optimization.max_upgrade_level {
        return Err("已达到最大升级等级".to_string());
    }

    // 计算升级成本
    let base_cost = 50u32;
    let cost = upgrade_optimization.calculate_upgrade_cost(base_cost, equipment.level);

    // 检查能源是否足够
    if inventory.energy < cost {
        return Err(format!("能源不足，需要 {}", cost));
    }

    // 扣除能源
    inventory.energy -= cost;

    // 检查升级是否成功
    if upgrade_optimization.check_upgrade_success(rng) {
        equipment.upgrade();
        info!("装备升级成功，当前等级: {}", equipment.level);
        Ok(true)
    } else {
        info!("装备升级失败，能源已消耗");
        Ok(false)
    }
}
