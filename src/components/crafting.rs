use bevy::prelude::*;
use crate::components::equipment::{EquipmentType, EquipmentRarity, EquipmentStats};

/// 装备制造配方
#[derive(Debug, Clone, Resource)]
pub struct CraftingRecipe {
    pub equipment_type: EquipmentType,
    pub rarity: EquipmentRarity,
    pub materials: Vec<MaterialRequirement>,
    pub energy_cost: u32,
    pub crafting_time: f32,
    pub unlocked: bool,
}

/// 材料需求
#[derive(Debug, Clone)]
pub struct MaterialRequirement {
    pub material_type: MaterialType,
    pub amount: u32,
}

/// 材料类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialType {
    Energy,   // 能源
    Metal,    // 金属
    Soil,     // 土壤
    Crystal,  // 水晶
    Organic,  // 有机物
}

impl MaterialType {
    /// 获取材料的名称
    pub fn name(&self) -> &str {
        match self {
            MaterialType::Energy => "能源",
            MaterialType::Metal => "金属",
            MaterialType::Soil => "土壤",
            MaterialType::Crystal => "水晶",
            MaterialType::Organic => "有机物",
        }
    }

    /// 获取材料的颜色
    pub fn color(&self) -> Color {
        match self {
            MaterialType::Energy => Color::srgb(1.0, 1.0, 0.0),
            MaterialType::Metal => Color::srgb(0.8, 0.8, 0.8),
            MaterialType::Soil => Color::srgb(0.6, 0.4, 0.2),
            MaterialType::Crystal => Color::srgb(0.3, 0.8, 0.9),
            MaterialType::Organic => Color::srgb(0.3, 0.9, 0.3),
        }
    }
}

impl CraftingRecipe {
    /// 创建新的制造配方
    pub fn new(
        equipment_type: EquipmentType,
        rarity: EquipmentRarity,
        materials: Vec<MaterialRequirement>,
        energy_cost: u32,
        crafting_time: f32,
    ) -> Self {
        Self {
            equipment_type,
            rarity,
            materials,
            energy_cost,
            crafting_time,
            unlocked: false,
        }
    }

    /// 检查是否可以制造
    pub fn can_craft(&self, inventory: &Inventory) -> bool {
        if !self.unlocked {
            return false;
        }

        if inventory.energy < self.energy_cost {
            return false;
        }

        for material in &self.materials {
            let amount = inventory.get_material(material.material_type);
            if amount < material.amount {
                return false;
            }
        }

        true
    }

    /// 解锁配方
    pub fn unlock(&mut self) {
        self.unlocked = true;
    }
}

/// 材料库存
#[derive(Debug, Clone, Default, Resource)]
pub struct Inventory {
    pub energy: u32,
    pub metal: u32,
    pub soil: u32,
    pub crystal: u32,
    pub organic: u32,
}

impl Inventory {
    /// 获取材料数量
    pub fn get_material(&self, material_type: MaterialType) -> u32 {
        match material_type {
            MaterialType::Energy => self.energy,
            MaterialType::Metal => self.metal,
            MaterialType::Soil => self.soil,
            MaterialType::Crystal => self.crystal,
            MaterialType::Organic => self.organic,
        }
    }

    /// 添加材料
    pub fn add_material(&mut self, material_type: MaterialType, amount: u32) {
        match material_type {
            MaterialType::Energy => self.energy += amount,
            MaterialType::Metal => self.metal += amount,
            MaterialType::Soil => self.soil += amount,
            MaterialType::Crystal => self.crystal += amount,
            MaterialType::Organic => self.organic += amount,
        }
    }

    /// 扣除材料
    pub fn remove_material(&mut self, material_type: MaterialType, amount: u32) -> bool {
        let current = self.get_material(material_type);
        if current < amount {
            return false;
        }

        match material_type {
            MaterialType::Energy => self.energy -= amount,
            MaterialType::Metal => self.metal -= amount,
            MaterialType::Soil => self.soil -= amount,
            MaterialType::Crystal => self.crystal -= amount,
            MaterialType::Organic => self.organic -= amount,
        }

        true
    }
}

/// 配方书
#[derive(Debug, Clone, Default, Resource)]
pub struct RecipeBook {
    pub recipes: Vec<CraftingRecipe>,
}

impl RecipeBook {
    /// 添加配方
    pub fn add_recipe(&mut self, recipe: CraftingRecipe) {
        self.recipes.push(recipe);
    }

    /// 获取可制造的配方
    pub fn get_craftable_recipes(&self, inventory: &Inventory) -> Vec<&CraftingRecipe> {
        self.recipes
            .iter()
            .filter(|recipe| recipe.can_craft(inventory))
            .collect()
    }

    /// 获取已解锁的配方
    pub fn get_unlocked_recipes(&self) -> Vec<&CraftingRecipe> {
        self.recipes
            .iter()
            .filter(|recipe| recipe.unlocked)
            .collect()
    }

    /// 解锁配方
    pub fn unlock_recipe(&mut self, equipment_type: EquipmentType, rarity: EquipmentRarity) -> bool {
        for recipe in &mut self.recipes {
            if recipe.equipment_type == equipment_type && recipe.rarity == rarity {
                recipe.unlock();
                return true;
            }
        }
        false
    }
}

/// 制造品质控制
#[derive(Debug, Clone, Copy)]
pub struct QualityControl {
    pub base_quality: f32,
    pub quality_variance: f32,
    pub skill_bonus: f32,
}

impl Default for QualityControl {
    fn default() -> Self {
        Self {
            base_quality: 1.0,
            quality_variance: 0.2,
            skill_bonus: 0.0,
        }
    }
}

impl QualityControl {
    /// 计算最终品质
    pub fn calculate_quality(&self, rng: &mut impl rand::Rng) -> f32 {
        let variance = (rng.gen::<f32>() - 0.5) * 2.0 * self.quality_variance;
        let quality = self.base_quality + variance + self.skill_bonus;
        quality.max(0.5).min(2.0)
    }

    /// 根据品质计算稀有度
    pub fn calculate_rarity(&self, quality: f32) -> EquipmentRarity {
        if quality >= 1.8 {
            EquipmentRarity::Mythic
        } else if quality >= 1.5 {
            EquipmentRarity::Legendary
        } else if quality >= 1.2 {
            EquipmentRarity::Rare
        } else if quality >= 1.0 {
            EquipmentRarity::Uncommon
        } else {
            EquipmentRarity::Common
        }
    }
}

/// 装备升级优化
#[derive(Debug, Clone, Copy)]
pub struct UpgradeOptimization {
    pub upgrade_cost_multiplier: f32,
    pub upgrade_success_rate: f32,
    pub max_upgrade_level: u32,
}

impl Default for UpgradeOptimization {
    fn default() -> Self {
        Self {
            upgrade_cost_multiplier: 1.0,
            upgrade_success_rate: 1.0,
            max_upgrade_level: 10,
        }
    }
}

impl UpgradeOptimization {
    /// 计算升级成本
    pub fn calculate_upgrade_cost(&self, base_cost: u32, current_level: u32) -> u32 {
        let multiplier = self.upgrade_cost_multiplier * (1.0 + current_level as f32 * 0.2);
        (base_cost as f32 * multiplier) as u32
    }

    /// 检查升级是否成功
    pub fn check_upgrade_success(&self, rng: &mut impl rand::Rng) -> bool {
        rng.gen::<f32>() < self.upgrade_success_rate
    }

    /// 计算升级属性加成
    pub fn calculate_upgrade_bonus(&self, level: u32) -> f32 {
        1.0 + level as f32 * 0.1
    }
}
