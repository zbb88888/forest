use bevy::prelude::*;

/// 建筑类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildingType {
    // 资源生产建筑
    EnergyCollector,    // 能源收集器
    MetalMine,         // 金属矿
    CrystalExtractor,   // 水晶提取器
    OrganicFarm,       // 有机农场
    
    // 防御建筑
    Turret,            // 炮塔
    LaserTower,        // 激光塔
    ShieldGenerator,   // 护盾发生器
    
    // 辅助建筑
    Storage,           // 仓库
    RepairStation,     // 维修站
    Radar,             // 雷达
    
    // 特殊建筑
    Teleporter,        // 传送器
    PowerCore,         // 能量核心
}

impl BuildingType {
    /// 获取建筑的名称
    pub fn name(&self) -> &str {
        match self {
            BuildingType::EnergyCollector => "能源收集器",
            BuildingType::MetalMine => "金属矿",
            BuildingType::CrystalExtractor => "水晶提取器",
            BuildingType::OrganicFarm => "有机农场",
            BuildingType::Turret => "炮塔",
            BuildingType::LaserTower => "激光塔",
            BuildingType::ShieldGenerator => "护盾发生器",
            BuildingType::Storage => "仓库",
            BuildingType::RepairStation => "维修站",
            BuildingType::Radar => "雷达",
            BuildingType::Teleporter => "传送器",
            BuildingType::PowerCore => "能量核心",
        }
    }

    /// 获取建筑的颜色
    pub fn color(&self) -> Color {
        match self {
            BuildingType::EnergyCollector => Color::srgb(1.0, 1.0, 0.0),
            BuildingType::MetalMine => Color::srgb(0.8, 0.8, 0.8),
            BuildingType::CrystalExtractor => Color::srgb(0.3, 0.8, 0.9),
            BuildingType::OrganicFarm => Color::srgb(0.3, 0.9, 0.3),
            BuildingType::Turret => Color::srgb(0.9, 0.3, 0.3),
            BuildingType::LaserTower => Color::srgb(0.9, 0.5, 0.5),
            BuildingType::ShieldGenerator => Color::srgb(0.5, 0.5, 0.9),
            BuildingType::Storage => Color::srgb(0.6, 0.6, 0.6),
            BuildingType::RepairStation => Color::srgb(0.9, 0.7, 0.3),
            BuildingType::Radar => Color::srgb(0.3, 0.7, 0.9),
            BuildingType::Teleporter => Color::srgb(0.7, 0.3, 0.9),
            BuildingType::PowerCore => Color::srgb(0.9, 0.9, 0.3),
        }
    }

    /// 获取建筑的基础属性
    pub fn base_stats(&self) -> BuildingStats {
        match self {
            BuildingType::EnergyCollector => BuildingStats {
                production_rate: 1.0,
                storage_capacity: 100,
                defense: 10.0,
                range: 0.0,
                energy_cost: 50,
                metal_cost: 20,
                crystal_cost: 0,
                organic_cost: 0,
                build_time: 5.0,
            },
            BuildingType::MetalMine => BuildingStats {
                production_rate: 0.5,
                storage_capacity: 100,
                defense: 10.0,
                range: 0.0,
                energy_cost: 80,
                metal_cost: 30,
                crystal_cost: 0,
                organic_cost: 0,
                build_time: 8.0,
            },
            BuildingType::CrystalExtractor => BuildingStats {
                production_rate: 0.3,
                storage_capacity: 50,
                defense: 10.0,
                range: 0.0,
                energy_cost: 100,
                metal_cost: 40,
                crystal_cost: 10,
                organic_cost: 0,
                build_time: 10.0,
            },
            BuildingType::OrganicFarm => BuildingStats {
                production_rate: 0.8,
                storage_capacity: 80,
                defense: 10.0,
                range: 0.0,
                energy_cost: 60,
                metal_cost: 20,
                crystal_cost: 0,
                organic_cost: 10,
                build_time: 6.0,
            },
            BuildingType::Turret => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 20.0,
                range: 5.0,
                energy_cost: 100,
                metal_cost: 50,
                crystal_cost: 0,
                organic_cost: 0,
                build_time: 8.0,
            },
            BuildingType::LaserTower => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 30.0,
                range: 8.0,
                energy_cost: 150,
                metal_cost: 80,
                crystal_cost: 20,
                organic_cost: 0,
                build_time: 12.0,
            },
            BuildingType::ShieldGenerator => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 40.0,
                range: 3.0,
                energy_cost: 200,
                metal_cost: 100,
                crystal_cost: 30,
                organic_cost: 0,
                build_time: 15.0,
            },
            BuildingType::Storage => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 500,
                defense: 15.0,
                range: 0.0,
                energy_cost: 80,
                metal_cost: 40,
                crystal_cost: 0,
                organic_cost: 0,
                build_time: 6.0,
            },
            BuildingType::RepairStation => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 15.0,
                range: 3.0,
                energy_cost: 120,
                metal_cost: 60,
                crystal_cost: 10,
                organic_cost: 0,
                build_time: 10.0,
            },
            BuildingType::Radar => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 10.0,
                range: 10.0,
                energy_cost: 100,
                metal_cost: 50,
                crystal_cost: 20,
                organic_cost: 0,
                build_time: 8.0,
            },
            BuildingType::Teleporter => BuildingStats {
                production_rate: 0.0,
                storage_capacity: 0,
                defense: 10.0,
                range: 0.0,
                energy_cost: 300,
                metal_cost: 150,
                crystal_cost: 50,
                organic_cost: 0,
                build_time: 20.0,
            },
            BuildingType::PowerCore => BuildingStats {
                production_rate: 2.0,
                storage_capacity: 200,
                defense: 50.0,
                range: 0.0,
                energy_cost: 500,
                metal_cost: 200,
                crystal_cost: 100,
                organic_cost: 0,
                build_time: 30.0,
            },
        }
    }
}

/// 建筑属性
#[derive(Debug, Clone, Copy)]
pub struct BuildingStats {
    pub production_rate: f32,  // 生产速率
    pub storage_capacity: u32, // 存储容量
    pub defense: f32,          // 防御力
    pub range: f32,            // 范围
    pub energy_cost: u32,      // 能源成本
    pub metal_cost: u32,       // 金属成本
    pub crystal_cost: u32,      // 水晶成本
    pub organic_cost: u32,      // 有机物成本
    pub build_time: f32,       // 建造时间
}

/// 建筑组件
#[derive(Component, Clone, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
    pub stats: BuildingStats,
    pub current_storage: u32,
    pub production_progress: f32,
    pub is_operational: bool,
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        let stats = building_type.base_stats();

        Self {
            building_type,
            level: 1,
            stats,
            current_storage: 0,
            production_progress: 0.0,
            is_operational: false,
        }
    }

    /// 升级建筑
    pub fn upgrade(&mut self) {
        self.level += 1;
        let upgrade_multiplier = 1.0 + (self.level as f32 * 0.2);

        let base_stats = self.building_type.base_stats();
        self.stats = BuildingStats {
            production_rate: base_stats.production_rate * upgrade_multiplier,
            storage_capacity: (base_stats.storage_capacity as f32 * upgrade_multiplier) as u32,
            defense: base_stats.defense * upgrade_multiplier,
            range: base_stats.range,
            energy_cost: base_stats.energy_cost,
            metal_cost: base_stats.metal_cost,
            crystal_cost: base_stats.crystal_cost,
            organic_cost: base_stats.organic_cost,
            build_time: base_stats.build_time,
        };
    }

    /// 生产资源
    pub fn produce(&mut self, delta_time: f32) -> Option<ResourceType> {
        if !self.is_operational {
            return None;
        }

        if self.stats.production_rate <= 0.0 {
            return None;
        }

        self.production_progress += self.stats.production_rate * delta_time;

        if self.production_progress >= 1.0 {
            self.production_progress = 0.0;
            
            // 根据建筑类型确定产出资源
            match self.building_type {
                BuildingType::EnergyCollector => Some(ResourceType::Energy),
                BuildingType::MetalMine => Some(ResourceType::Metal),
                BuildingType::CrystalExtractor => Some(ResourceType::Crystal),
                BuildingType::OrganicFarm => Some(ResourceType::Organic),
                _ => None,
            }
        } else {
            None
        }
    }

    /// 检查是否可以建造
    pub fn can_build(&self, inventory: &Inventory) -> bool {
        if inventory.energy < self.stats.energy_cost {
            return false;
        }
        if inventory.metal < self.stats.metal_cost {
            return false;
        }
        if inventory.crystal < self.stats.crystal_cost {
            return false;
        }
        if inventory.organic < self.stats.organic_cost {
            return false;
        }
        true
    }

    /// 消耗建造资源
    pub fn consume_build_resources(&mut self, inventory: &mut Inventory) -> bool {
        if !self.can_build(inventory) {
            return false;
        }

        inventory.energy -= self.stats.energy_cost;
        inventory.metal -= self.stats.metal_cost;
        inventory.crystal -= self.stats.crystal_cost;
        inventory.organic -= self.stats.organic_cost;

        true
    }
}

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Energy,
    Metal,
    Crystal,
    Organic,
}

/// 建筑库存
#[derive(Debug, Clone, Default, Resource)]
pub struct Inventory {
    pub energy: u32,
    pub metal: u32,
    pub crystal: u32,
    pub organic: u32,
}

/// 建筑位置
#[derive(Component, Clone, Debug)]
pub struct BuildingPosition {
    pub tile_x: u32,
    pub tile_y: u32,
}

/// 建筑状态
#[derive(Component, Clone, Debug)]
pub struct BuildingStatus {
    pub is_constructing: bool,
    pub construction_progress: f32,
    pub is_damaged: bool,
    pub health: f32,
    pub max_health: f32,
}

impl Default for BuildingStatus {
    fn default() -> Self {
        Self {
            is_constructing: true,
            construction_progress: 0.0,
            is_damaged: false,
            health: 100.0,
            max_health: 100.0,
        }
    }
}
