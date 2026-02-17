use bevy::prelude::*;
use crate::components::plant::PlantType;

/// 植物等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantLevel {
    Level1, // 基础等级
    Level2, // 进阶等级
    Level3, // 高级等级
    Level4, // 专家等级
    Level5, // 大师等级
}

impl PlantLevel {
    /// 获取等级数值
    pub fn value(&self) -> u8 {
        match self {
            PlantLevel::Level1 => 1,
            PlantLevel::Level2 => 2,
            PlantLevel::Level3 => 3,
            PlantLevel::Level4 => 4,
            PlantLevel::Level5 => 5,
        }
    }

    /// 获取下一等级
    pub fn next_level(&self) -> Option<PlantLevel> {
        match self {
            PlantLevel::Level1 => Some(PlantLevel::Level2),
            PlantLevel::Level2 => Some(PlantLevel::Level3),
            PlantLevel::Level3 => Some(PlantLevel::Level4),
            PlantLevel::Level4 => Some(PlantLevel::Level5),
            PlantLevel::Level5 => None,
        }
    }

    /// 获取升级所需能源
    pub fn upgrade_cost(&self) -> u32 {
        match self {
            PlantLevel::Level1 => 50,
            PlantLevel::Level2 => 100,
            PlantLevel::Level3 => 200,
            PlantLevel::Level4 => 400,
            PlantLevel::Level5 => 0, // 已达最高级
        }
    }

    /// 获取等级加成倍率
    pub fn multiplier(&self) -> f32 {
        match self {
            PlantLevel::Level1 => 1.0,
            PlantLevel::Level2 => 1.2,
            PlantLevel::Level3 => 1.5,
            PlantLevel::Level4 => 2.0,
            PlantLevel::Level5 => 3.0,
        }
    }
}

/// 植物升级属性
#[derive(Debug, Clone, Component)]
pub struct PlantUpgrade {
    pub level: PlantLevel,
    pub growth_speed_bonus: f32,    // 生长速度加成
    pub output_bonus: f32,           // 产出加成
    pub health_bonus: f32,           // 健康度加成
    pub resistance_bonus: f32,       // 抗性加成
}

impl PlantUpgrade {
    /// 创建新的升级组件
    pub fn new() -> Self {
        Self {
            level: PlantLevel::Level1,
            growth_speed_bonus: 0.0,
            output_bonus: 0.0,
            health_bonus: 0.0,
            resistance_bonus: 0.0,
        }
    }

    /// 检查是否可以升级
    pub fn can_upgrade(&self) -> bool {
        self.level.next_level().is_some()
    }

    /// 获取升级所需能源
    pub fn get_upgrade_cost(&self) -> u32 {
        self.level.upgrade_cost()
    }

    /// 应用升级
    pub fn apply_upgrade(&mut self) -> bool {
        if let Some(next_level) = self.level.next_level() {
            self.level = next_level;

            // 根据等级提升属性
            let multiplier = self.level.multiplier();
            self.growth_speed_bonus = (multiplier - 1.0) * 0.5;
            self.output_bonus = (multiplier - 1.0) * 0.8;
            self.health_bonus = (multiplier - 1.0) * 0.3;
            self.resistance_bonus = (multiplier - 1.0) * 0.4;

            true
        } else {
            false
        }
    }

    /// 计算实际生长速度加成
    pub fn calculate_growth_speed(&self, base_speed: f32) -> f32 {
        base_speed * (1.0 + self.growth_speed_bonus)
    }

    /// 计算实际产出加成
    pub fn calculate_output(&self, base_output: f32) -> f32 {
        base_output * (1.0 + self.output_bonus)
    }

    /// 计算实际健康度加成
    pub fn calculate_health(&self, base_health: f32) -> f32 {
        base_health * (1.0 + self.health_bonus)
    }

    /// 计算实际抗性加成
    pub fn calculate_resistance(&self, base_resistance: f32) -> f32 {
        base_resistance * (1.0 + self.resistance_bonus)
    }
}

/// 植物品种树
#[derive(Debug, Clone, Resource)]
pub struct PlantVarietyTree {
    /// 已解锁的植物品种
    pub unlocked_varieties: Vec<PlantType>,
    /// 品种解锁条件
    pub unlock_conditions: std::collections::HashMap<PlantType, UnlockCondition>,
}

/// 品种解锁条件
#[derive(Debug, Clone)]
pub struct UnlockCondition {
    /// 需要达到的植物等级
    pub required_level: PlantLevel,
    /// 需要收获的次数
    pub required_harvests: u32,
    /// 需要消耗的能源
    pub energy_cost: u32,
}

impl UnlockCondition {
    /// 创建新的解锁条件
    pub fn new(required_level: PlantLevel, required_harvests: u32, energy_cost: u32) -> Self {
        Self {
            required_level,
            required_harvests,
            energy_cost,
        }
    }
}

impl Default for PlantVarietyTree {
    fn default() -> Self {
        let mut unlocked_varieties = vec![PlantType::Grass];
        let mut unlock_conditions = std::collections::HashMap::new();

        // 灌木：需要Level 2，收获10次，消耗100能源
        unlock_conditions.insert(
            PlantType::Bush,
            UnlockCondition::new(PlantLevel::Level2, 10, 100),
        );

        // 花朵：需要Level 2，收获5次，消耗50能源
        unlock_conditions.insert(
            PlantType::Flower,
            UnlockCondition::new(PlantLevel::Level2, 5, 50),
        );

        // 树木：需要Level 3，收获20次，消耗200能源
        unlock_conditions.insert(
            PlantType::Tree,
            UnlockCondition::new(PlantLevel::Level3, 20, 200),
        );

        // 能源花：需要Level 4，收获30次，消耗500能源
        unlock_conditions.insert(
            PlantType::EnergyFlower,
            UnlockCondition::new(PlantLevel::Level4, 30, 500),
        );

        Self {
            unlocked_varieties,
            unlock_conditions,
        }
    }
}

impl PlantVarietyTree {
    /// 检查品种是否已解锁
    pub fn is_unlocked(&self, plant_type: PlantType) -> bool {
        self.unlocked_varieties.contains(&plant_type)
    }

    /// 检查是否可以解锁品种
    pub fn can_unlock(&self, plant_type: PlantType, current_level: PlantLevel, harvests: u32, energy: u32) -> bool {
        if let Some(condition) = self.unlock_conditions.get(&plant_type) {
            current_level.value() >= condition.required_level.value()
                && harvests >= condition.required_harvests
                && energy >= condition.energy_cost
        } else {
            false
        }
    }

    /// 解锁品种
    pub fn unlock(&mut self, plant_type: PlantType) -> bool {
        if !self.is_unlocked(plant_type) && self.unlock_conditions.contains_key(&plant_type) {
            self.unlocked_varieties.push(plant_type);
            true
        } else {
            false
        }
    }
}

/// 植物收获统计
#[derive(Debug, Clone, Resource)]
pub struct PlantHarvestStats {
    /// 各类型植物的收获次数
    pub harvest_counts: std::collections::HashMap<PlantType, u32>,
}

impl Default for PlantHarvestStats {
    fn default() -> Self {
        let mut harvest_counts = std::collections::HashMap::new();
        harvest_counts.insert(PlantType::Grass, 0);
        harvest_counts.insert(PlantType::Bush, 0);
        harvest_counts.insert(PlantType::Tree, 0);
        harvest_counts.insert(PlantType::Flower, 0);
        harvest_counts.insert(PlantType::EnergyFlower, 0);
        Self { harvest_counts }
    }
}

impl PlantHarvestStats {
    /// 记录收获
    pub fn record_harvest(&mut self, plant_type: PlantType) {
        *self.harvest_counts.entry(plant_type).or_insert(0) += 1;
    }

    /// 获取收获次数
    pub fn get_harvest_count(&self, plant_type: PlantType) -> u32 {
        *self.harvest_counts.get(&plant_type).unwrap_or(&0)
    }

    /// 获取总收获次数
    pub fn get_total_harvests(&self) -> u32 {
        self.harvest_counts.values().sum()
    }
}
