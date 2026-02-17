use bevy::prelude::*;
use rand::Rng;

/// 战斗系统组件

/// 攻击类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatAttackType {
    Melee,    // 近战
    Ranged,   // 远程
    Magic,    // 魔法
}

/// 攻击效果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Physical,  // 物理伤害
    Energy,    // 能量伤害
    Explosive, // 爆炸伤害
    Laser,     // 激光伤害
    Corrosive, // 腐蚀伤害
}

/// 攻击属性
#[derive(Debug, Clone, Copy)]
pub struct AttackAttributes {
    pub damage: f32,        // 基础伤害
    pub damage_type: DamageType, // 伤害类型
    pub attack_range: f32,   // 攻击范围
    pub attack_speed: f32,   // 攻击速度
    pub critical_chance: f32, // 暴击率
    pub critical_multiplier: f32, // 暴击倍率
    pub penetration: f32,    // 穿透力
}

impl Default for AttackAttributes {
    fn default() -> Self {
        Self {
            damage: 10.0,
            damage_type: DamageType::Physical,
            attack_range: 1.0,
            attack_speed: 1.0,
            critical_chance: 0.05,
            critical_multiplier: 1.5,
            penetration: 0.0,
        }
    }
}

/// 防御属性
#[derive(Debug, Clone, Copy)]
pub struct DefenseAttributes {
    pub physical_resistance: f32, // 物理抗性
    pub energy_resistance: f32,   // 能量抗性
    pub explosive_resistance: f32, // 爆炸抗性
    pub laser_resistance: f32,    // 激光抗性
    pub corrosive_resistance: f32, // 腐蚀抗性
    pub dodge_chance: f32,         // 闪避率
    pub block_chance: f32,         // 格挡率
    pub block_reduction: f32,      // 格挡减伤
}

impl Default for DefenseAttributes {
    fn default() -> Self {
        Self {
            physical_resistance: 0.0,
            energy_resistance: 0.0,
            explosive_resistance: 0.0,
            laser_resistance: 0.0,
            corrosive_resistance: 0.0,
            dodge_chance: 0.0,
            block_chance: 0.0,
            block_reduction: 0.5,
        }
    }
}

/// 战斗组件
#[derive(Component, Clone, Debug)]
pub struct Combat {
    pub attack: AttackAttributes,
    pub defense: DefenseAttributes,
    pub attack_cooldown: f32,
    pub is_attacking: bool,
    pub target: Option<Entity>,
}

impl Combat {
    pub fn new() -> Self {
        Self {
            attack: AttackAttributes::default(),
            defense: DefenseAttributes::default(),
            attack_cooldown: 0.0,
            is_attacking: false,
            target: None,
        }
    }

    /// 检查是否可以攻击
    pub fn can_attack(&self) -> bool {
        self.attack_cooldown <= 0.0
    }

    /// 获取实际伤害（考虑抗性）
    pub fn calculate_damage(&self, target_defense: &DefenseAttributes) -> f32 {
        let base_damage = self.attack.damage;
        let resistance = match self.attack.damage_type {
            DamageType::Physical => target_defense.physical_resistance,
            DamageType::Energy => target_defense.energy_resistance,
            DamageType::Explosive => target_defense.explosive_resistance,
            DamageType::Laser => target_defense.laser_resistance,
            DamageType::Corrosive => target_defense.corrosive_resistance,
        };

        // 计算抗性减伤
        let damage_after_resistance = base_damage * (1.0 - resistance.min(0.8));

        // 计算穿透减伤
        let damage_after_penetration = damage_after_resistance + self.attack.penetration;

        damage_after_penetration.max(0.0)
    }

    /// 计算是否暴击
    pub fn is_critical(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f32>() < self.attack.critical_chance
    }

    /// 获取暴击伤害
    pub fn get_critical_damage(&self) -> f32 {
        self.attack.damage * self.attack.critical_multiplier
    }
}

/// 伤害事件
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub damage: f32,
    pub damage_type: DamageType,
    pub is_critical: bool,
}

/// 治疗事件
#[derive(Event, Debug, Clone)]
pub struct HealEvent {
    pub target: Entity,
    pub amount: f32,
}

/// 死亡事件
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
}

/// 战斗效果组件
#[derive(Component, Clone, Debug)]
pub struct CombatEffect {
    pub effect_type: CombatEffectType,
    pub duration: f32,
    pub timer: f32,
    pub value: f32,
}

/// 战斗效果类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEffectType {
    Burn,        // 燃烧
    Freeze,      // 冰冻
    Poison,      // 中毒
    Slow,        // 减速
    Stun,        // 眩晕
    Shield,      // 护盾
}

impl CombatEffect {
    pub fn new(effect_type: CombatEffectType, duration: f32, value: f32) -> Self {
        Self {
            effect_type,
            duration,
            timer: 0.0,
            value,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 战斗统计
#[derive(Component, Clone, Debug)]
pub struct CombatStats {
    pub damage_dealt: f32,
    pub damage_taken: f32,
    pub enemies_defeated: u32,
    pub critical_hits: u32,
    pub total_attacks: u32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            damage_dealt: 0.0,
            damage_taken: 0.0,
            enemies_defeated: 0,
            critical_hits: 0,
            total_attacks: 0,
        }
    }
}
