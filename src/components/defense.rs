use bevy::prelude::*;

/// 防御系统组件

/// 防御塔类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseTowerType {
    ArrowTower,      // 箭塔
    CannonTower,     // 炮塔
    LaserTower,      // 激光塔
    IceTower,        // 冰塔
    PoisonTower,     // 毒塔
    ElectricTower,   // 电塔
}

/// 防御塔属性
#[derive(Debug, Clone, Copy)]
pub struct DefenseTowerStats {
    pub damage: f32,           // 伤害
    pub range: f32,            // 攻击范围
    pub attack_speed: f32,     // 攻击速度
    pub projectile_speed: f32, // 弹道速度
    pub rotation_speed: f32,   // 转向速度
    pub energy_cost: f32,      // 能量消耗
    pub upgrade_cost: f32,     // 升级消耗
    pub level: u32,            // 等级
}

impl Default for DefenseTowerStats {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 100.0,
            attack_speed: 1.0,
            projectile_speed: 300.0,
            rotation_speed: 3.0,
            energy_cost: 100.0,
            upgrade_cost: 50.0,
            level: 1,
        }
    }
}

/// 防御塔组件
#[derive(Component, Clone, Debug)]
pub struct DefenseTower {
    pub tower_type: DefenseTowerType,
    pub stats: DefenseTowerStats,
    pub attack_cooldown: f32,
    pub target: Option<Entity>,
    pub is_active: bool,
}

impl DefenseTower {
    pub fn new(tower_type: DefenseTowerType) -> Self {
        let stats = match tower_type {
            DefenseTowerType::ArrowTower => DefenseTowerStats {
                damage: 10.0,
                range: 150.0,
                attack_speed: 2.0,
                projectile_speed: 400.0,
                rotation_speed: 5.0,
                energy_cost: 50.0,
                upgrade_cost: 25.0,
                level: 1,
            },
            DefenseTowerType::CannonTower => DefenseTowerStats {
                damage: 30.0,
                range: 120.0,
                attack_speed: 0.5,
                projectile_speed: 250.0,
                rotation_speed: 2.0,
                energy_cost: 100.0,
                upgrade_cost: 50.0,
                level: 1,
            },
            DefenseTowerType::LaserTower => DefenseTowerStats {
                damage: 15.0,
                range: 200.0,
                attack_speed: 1.5,
                projectile_speed: 1000.0,
                rotation_speed: 4.0,
                energy_cost: 150.0,
                upgrade_cost: 75.0,
                level: 1,
            },
            DefenseTowerType::IceTower => DefenseTowerStats {
                damage: 5.0,
                range: 100.0,
                attack_speed: 1.0,
                projectile_speed: 300.0,
                rotation_speed: 3.0,
                energy_cost: 75.0,
                upgrade_cost: 35.0,
                level: 1,
            },
            DefenseTowerType::PoisonTower => DefenseTowerStats {
                damage: 8.0,
                range: 130.0,
                attack_speed: 1.2,
                projectile_speed: 300.0,
                rotation_speed: 3.0,
                energy_cost: 80.0,
                upgrade_cost: 40.0,
                level: 1,
            },
            DefenseTowerType::ElectricTower => DefenseTowerStats {
                damage: 12.0,
                range: 110.0,
                attack_speed: 1.8,
                projectile_speed: 500.0,
                rotation_speed: 4.0,
                energy_cost: 120.0,
                upgrade_cost: 60.0,
                level: 1,
            },
        };

        Self {
            tower_type,
            stats,
            attack_cooldown: 0.0,
            target: None,
            is_active: true,
        }
    }

    /// 检查是否可以攻击
    pub fn can_attack(&self) -> bool {
        self.is_active && self.attack_cooldown <= 0.0
    }

    /// 获取攻击间隔
    pub fn get_attack_interval(&self) -> f32 {
        1.0 / self.stats.attack_speed
    }

    /// 升级防御塔
    pub fn upgrade(&mut self) {
        self.stats.level += 1;
        self.stats.damage *= 1.2;
        self.stats.range *= 1.1;
        self.stats.attack_speed *= 1.1;
        self.stats.upgrade_cost *= 1.5;
    }
}

/// 防御墙组件
#[derive(Component, Clone, Debug)]
pub struct DefenseWall {
    pub health: f32,
    pub max_health: f32,
    pub defense: f32,
    pub level: u32,
    pub repair_cost: f32,
}

impl DefenseWall {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            defense: 10.0,
            level: 1,
            repair_cost: 10.0,
        }
    }

    /// 接受伤害
    pub fn take_damage(&mut self, damage: f32) -> f32 {
        let actual_damage = damage - self.defense;
        let actual_damage = actual_damage.max(0.0);
        self.health -= actual_damage;
        actual_damage
    }

    /// 修理
    pub fn repair(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// 升级
    pub fn upgrade(&mut self) {
        self.level += 1;
        self.max_health *= 1.3;
        self.health = self.max_health;
        self.defense *= 1.2;
        self.repair_cost *= 1.2;
    }

    /// 检查是否被摧毁
    pub fn is_destroyed(&self) -> bool {
        self.health <= 0.0
    }
}

/// 防御范围组件
#[derive(Component, Clone, Debug)]
pub struct DefenseRange {
    pub range: f32,
}

impl DefenseRange {
    pub fn new(range: f32) -> Self {
        Self { range }
    }
}

/// 防御效果组件
#[derive(Component, Clone, Debug)]
pub struct DefenseEffect {
    pub effect_type: DefenseEffectType,
    pub duration: f32,
    pub value: f32,
}

/// 防御效果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseEffectType {
    Slow,      // 减速
    Freeze,    // 冰冻
    Poison,    // 中毒
    Burn,      // 燃烧
    Stun,      // 眩晕
}

impl DefenseEffect {
    pub fn new(effect_type: DefenseEffectType, duration: f32, value: f32) -> Self {
        Self {
            effect_type,
            duration,
            value,
        }
    }
}

/// 防御统计
#[derive(Component, Clone, Debug)]
pub struct DefenseStats {
    pub enemies_defeated: u32,
    pub total_damage: f32,
    pub shots_fired: u32,
}

impl Default for DefenseStats {
    fn default() -> Self {
        Self {
            enemies_defeated: 0,
            total_damage: 0.0,
            shots_fired: 0,
        }
    }
}
