use bevy::prelude::*;

/// 植物类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantType {
    Grass,      // 草
    Bush,       // 灌木
    Tree,       // 树木
    Flower,     // 花朵
    EnergyFlower, // 能源花
}

impl PlantType {
    /// 获取植物的颜色
    pub fn color(&self) -> Color {
        match self {
            PlantType::Grass => Color::srgb(0.3, 0.9, 0.3),
            PlantType::Bush => Color::srgb(0.2, 0.7, 0.2),
            PlantType::Tree => Color::srgb(0.1, 0.5, 0.1),
            PlantType::Flower => Color::srgb(0.9, 0.5, 0.9),
            PlantType::EnergyFlower => Color::srgb(0.5, 0.9, 0.9),
        }
    }

    /// 获取植物的基础生长速率
    pub fn base_growth_rate(&self) -> f32 {
        match self {
            PlantType::Grass => 1.0,
            PlantType::Bush => 0.8,
            PlantType::Tree => 0.5,
            PlantType::Flower => 0.7,
            PlantType::EnergyFlower => 0.6,
        }
    }

    /// 获取植物的能源产出倍率
    pub fn energy_multiplier(&self) -> f32 {
        match self {
            PlantType::Grass => 1.0,
            PlantType::Bush => 1.5,
            PlantType::Tree => 2.0,
            PlantType::Flower => 1.3,
            PlantType::EnergyFlower => 3.0,
        }
    }
}

/// 植物组件
#[derive(Component, Clone, Debug)]
pub struct Plant {
    pub plant_type: PlantType,
    pub growth_stage: u8,      // 生长阶段 (0-5)
    pub health: f32,          // 健康度 (0.0 - 1.0)
    pub maturity: f32,         // 成熟度 (0.0 - 1.0)
    pub water_level: f32,     // 水分等级 (0.0 - 1.0)
    pub nutrient_level: f32,   // 营养等级 (0.0 - 1.0)
    pub max_stages: u8,       // 最大生长阶段
    pub energy_output: f32,    // 能源产出速率
}

impl Plant {
    pub fn new(plant_type: PlantType) -> Self {
        let base_growth_rate = plant_type.base_growth_rate();
        let energy_multiplier = plant_type.energy_multiplier();

        Self {
            plant_type,
            growth_stage: 0,
            health: 1.0,
            maturity: 0.0,
            water_level: 0.5,
            nutrient_level: 0.5,
            max_stages: 5,
            energy_output: base_growth_rate * energy_multiplier,
        }
    }

    /// 检查植物是否可收获
    pub fn is_harvestable(&self) -> bool {
        self.growth_stage >= self.max_stages && self.maturity >= 1.0
    }

    /// 计算收获奖励
    pub fn calculate_harvest_reward(&self) -> u32 {
        if !self.is_harvestable() {
            return 0;
        }

        let base_reward = match self.plant_type {
            PlantType::Grass => 10,
            PlantType::Bush => 20,
            PlantType::Tree => 50,
            PlantType::Flower => 15,
            PlantType::EnergyFlower => 30,
        };

        // 根据健康度和成熟度计算最终奖励
        let multiplier = self.health * self.maturity;
        (base_reward as f32 * multiplier) as u32
    }
}

/// 可种植标记组件
#[derive(Component)]
pub struct Plantable;

/// 可收获标记组件
#[derive(Component)]
pub struct Harvestable;

/// 植物生长系统
#[derive(Component, Clone, Debug)]
pub struct Growable {
    pub base_growth_rate: f32,
    pub current_stage: u8,
    pub max_stages: u8,
    pub growth_progress: f32,
}

impl Growable {
    pub fn new(base_growth_rate: f32, max_stages: u8) -> Self {
        Self {
            base_growth_rate,
            current_stage: 0,
            max_stages,
            growth_progress: 0.0,
        }
    }
}
