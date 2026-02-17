use bevy::prelude::*;

/// 敌人类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    // 机器人敌人
    ScoutBot,          // 侦察机器人
    CombatBot,         // 战斗机器人
    HeavyBot,          // 重型机器人
    EliteBot,          // 精英机器人
    
    // 机器虫敌人
    WorkerBug,         // 工虫
    WarriorBug,        // 战虫
    SpitterBug,        // 喷吐虫
    TankBug,          // 坦克虫
    QueenBug,          // 虫后
    
    // 机器鸟敌人
    ScoutBird,         // 侦察鸟
    AttackBird,        // 攻击鸟
    
    // 敌人建筑
    RobotFortress,     // 机器人堡垒
    AIMotherBase,      // AI母巢
}

impl EnemyType {
    /// 获取敌人的名称
    pub fn name(&self) -> &str {
        match self {
            EnemyType::ScoutBot => "侦察机器人",
            EnemyType::CombatBot => "战斗机器人",
            EnemyType::HeavyBot => "重型机器人",
            EnemyType::EliteBot => "精英机器人",
            EnemyType::WorkerBug => "工虫",
            EnemyType::WarriorBug => "战虫",
            EnemyType::SpitterBug => "喷吐虫",
            EnemyType::TankBug => "坦克虫",
            EnemyType::QueenBug => "虫后",
            EnemyType::ScoutBird => "侦察鸟",
            EnemyType::AttackBird => "攻击鸟",
            EnemyType::RobotFortress => "机器人堡垒",
            EnemyType::AIMotherBase => "AI母巢",
        }
    }

    /// 获取敌人的颜色
    pub fn color(&self) -> Color {
        match self {
            EnemyType::ScoutBot => Color::srgb(0.5, 0.5, 0.7),
            EnemyType::CombatBot => Color::srgb(0.7, 0.5, 0.5),
            EnemyType::HeavyBot => Color::srgb(0.6, 0.4, 0.4),
            EnemyType::EliteBot => Color::srgb(0.8, 0.6, 0.3),
            EnemyType::WorkerBug => Color::srgb(0.4, 0.6, 0.3),
            EnemyType::WarriorBug => Color::srgb(0.6, 0.4, 0.2),
            EnemyType::SpitterBug => Color::srgb(0.5, 0.5, 0.2),
            EnemyType::TankBug => Color::srgb(0.4, 0.3, 0.2),
            EnemyType::QueenBug => Color::srgb(0.7, 0.4, 0.4),
            EnemyType::ScoutBird => Color::srgb(0.6, 0.6, 0.4),
            EnemyType::AttackBird => Color::srgb(0.7, 0.4, 0.3),
            EnemyType::RobotFortress => Color::srgb(0.5, 0.5, 0.5),
            EnemyType::AIMotherBase => Color::srgb(0.8, 0.3, 0.3),
        }
    }

    /// 获取敌人的基础属性
    pub fn base_stats(&self) -> EnemyStats {
        match self {
            // 机器人敌人属性
            EnemyType::ScoutBot => EnemyStats {
                health: 50.0,
                damage: 10.0,
                attack_speed: 1.5,
                movement_speed: 1.2,
                defense: 5.0,
                attack_range: 3.0,
                detection_range: 8.0,
                xp_reward: 20,
            },
            EnemyType::CombatBot => EnemyStats {
                health: 100.0,
                damage: 20.0,
                attack_speed: 1.0,
                movement_speed: 1.0,
                defense: 10.0,
                attack_range: 2.0,
                detection_range: 6.0,
                xp_reward: 40,
            },
            EnemyType::HeavyBot => EnemyStats {
                health: 200.0,
                damage: 30.0,
                attack_speed: 0.8,
                movement_speed: 0.6,
                defense: 20.0,
                attack_range: 2.0,
                detection_range: 5.0,
                xp_reward: 80,
            },
            EnemyType::EliteBot => EnemyStats {
                health: 300.0,
                damage: 40.0,
                attack_speed: 1.2,
                movement_speed: 1.0,
                defense: 25.0,
                attack_range: 3.0,
                detection_range: 10.0,
                xp_reward: 150,
            },
            
            // 机器虫敌人属性
            EnemyType::WorkerBug => EnemyStats {
                health: 30.0,
                damage: 5.0,
                attack_speed: 1.0,
                movement_speed: 1.5,
                defense: 2.0,
                attack_range: 1.0,
                detection_range: 4.0,
                xp_reward: 10,
            },
            EnemyType::WarriorBug => EnemyStats {
                health: 80.0,
                damage: 15.0,
                attack_speed: 1.2,
                movement_speed: 1.3,
                defense: 8.0,
                attack_range: 1.5,
                detection_range: 6.0,
                xp_reward: 30,
            },
            EnemyType::SpitterBug => EnemyStats {
                health: 60.0,
                damage: 20.0,
                attack_speed: 0.8,
                movement_speed: 1.0,
                defense: 5.0,
                attack_range: 5.0,
                detection_range: 7.0,
                xp_reward: 40,
            },
            EnemyType::TankBug => EnemyStats {
                health: 250.0,
                damage: 25.0,
                attack_speed: 0.6,
                movement_speed: 0.5,
                defense: 30.0,
                attack_range: 1.5,
                detection_range: 4.0,
                xp_reward: 100,
            },
            EnemyType::QueenBug => EnemyStats {
                health: 500.0,
                damage: 50.0,
                attack_speed: 0.5,
                movement_speed: 0.3,
                defense: 40.0,
                attack_range: 3.0,
                detection_range: 12.0,
                xp_reward: 300,
            },
            
            // 机器鸟敌人属性
            EnemyType::ScoutBird => EnemyStats {
                health: 25.0,
                damage: 8.0,
                attack_speed: 2.0,
                movement_speed: 2.0,
                defense: 3.0,
                attack_range: 2.0,
                detection_range: 10.0,
                xp_reward: 15,
            },
            EnemyType::AttackBird => EnemyStats {
                health: 40.0,
                damage: 12.0,
                attack_speed: 1.8,
                movement_speed: 1.8,
                defense: 5.0,
                attack_range: 2.5,
                detection_range: 8.0,
                xp_reward: 25,
            },
            
            // 敌人建筑属性
            EnemyType::RobotFortress => EnemyStats {
                health: 1000.0,
                damage: 0.0,
                attack_speed: 0.0,
                movement_speed: 0.0,
                defense: 50.0,
                attack_range: 0.0,
                detection_range: 15.0,
                xp_reward: 200,
            },
            EnemyType::AIMotherBase => EnemyStats {
                health: 2000.0,
                damage: 0.0,
                attack_speed: 0.0,
                movement_speed: 0.0,
                defense: 80.0,
                attack_range: 0.0,
                detection_range: 20.0,
                xp_reward: 500,
            },
        }
    }

    /// 获取敌人的AI行为类型
    pub fn ai_behavior(&self) -> AIBehavior {
        match self {
            EnemyType::ScoutBot => AIBehavior::Patrol,
            EnemyType::CombatBot => AIBehavior::Aggressive,
            EnemyType::HeavyBot => AIBehavior::Guard,
            EnemyType::EliteBot => AIBehavior::Boss,
            
            EnemyType::WorkerBug => AIBehavior::Passive,
            EnemyType::WarriorBug => AIBehavior::Aggressive,
            EnemyType::SpitterBug => AIBehavior::Ranged,
            EnemyType::TankBug => AIBehavior::Guard,
            EnemyType::QueenBug => AIBehavior::Boss,
            
            EnemyType::ScoutBird => AIBehavior::Patrol,
            EnemyType::AttackBird => AIBehavior::Aggressive,
            
            EnemyType::RobotFortress => AIBehavior::Spawn,
            EnemyType::AIMotherBase => AIBehavior::Spawn,
        }
    }

    /// 获取敌人的攻击类型
    pub fn attack_type(&self) -> AttackType {
        match self {
            EnemyType::ScoutBot | EnemyType::CombatBot | 
            EnemyType::HeavyBot | EnemyType::EliteBot => AttackType::Laser,
            
            EnemyType::WorkerBug | EnemyType::WarriorBug | 
            EnemyType::TankBug => AttackType::Melee,
            
            EnemyType::SpitterBug => AttackType::Spit,
            EnemyType::QueenBug => AttackType::Summon,
            
            EnemyType::ScoutBird | EnemyType::AttackBird => AttackType::Melee,
            
            EnemyType::RobotFortress | EnemyType::AIMotherBase => AttackType::None,
        }
    }
}

/// 敌人属性
#[derive(Debug, Clone, Copy)]
pub struct EnemyStats {
    pub health: f32,           // 生命值
    pub damage: f32,           // 伤害
    pub attack_speed: f32,      // 攻击速度
    pub movement_speed: f32,     // 移动速度
    pub defense: f32,          // 防御力
    pub attack_range: f32,      // 攻击范围
    pub detection_range: f32,   // 探测范围
    pub xp_reward: u32,        // 经验奖励
}

/// AI行为类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIBehavior {
    Passive,      // 被动（不主动攻击）
    Patrol,       // 巡逻（在区域内巡逻）
    Aggressive,   // 主动（主动攻击玩家）
    Guard,        // 守卫（保护特定区域）
    Ranged,       // 远程（保持距离攻击）
    Boss,         // Boss（特殊AI）
    Spawn,        // 生成（可以生成其他敌人）
}

/// 攻击类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    None,    // 无（建筑类敌人）
    Melee,   // 近战
    Laser,   // 激光
    Spit,    // 喷吐
    Summon,  // 召唤
}

/// 敌人组件
#[derive(Component, Clone, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub stats: EnemyStats,
    pub current_health: f32,
    pub level: u32,
    pub ai_state: AIState,
    pub attack_cooldown: f32,
    pub target: Option<Entity>,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, level: u32) -> Self {
        let base_stats = enemy_type.base_stats();
        let level_multiplier = 1.0 + (level as f32 * 0.2);

        Self {
            enemy_type,
            stats: EnemyStats {
                health: base_stats.health * level_multiplier,
                damage: base_stats.damage * level_multiplier,
                attack_speed: base_stats.attack_speed,
                movement_speed: base_stats.movement_speed,
                defense: base_stats.defense * level_multiplier,
                attack_range: base_stats.attack_range,
                detection_range: base_stats.detection_range,
                xp_reward: (base_stats.xp_reward as f32 * level_multiplier) as u32,
            },
            current_health: base_stats.health * level_multiplier,
            level,
            ai_state: AIState::Idle,
            attack_cooldown: 0.0,
            target: None,
        }
    }

    /// 检查是否死亡
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0.0
    }

    /// 受到伤害
    pub fn take_damage(&mut self, damage: f32) {
        let actual_damage = (damage - self.stats.defense).max(0.0);
        self.current_health -= actual_damage;
    }

    /// 恢复生命
    pub fn heal(&mut self, amount: f32) {
        self.current_health = (self.current_health + amount).min(self.stats.health);
    }

    /// 获取生命百分比
    pub fn health_percentage(&self) -> f32 {
        self.current_health / self.stats.health
    }
}

/// AI状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    Idle,          // 闲置
    Patrol,        // 巡逻
    Chase,         // 追逐
    Attack,        // 攻击
    Retreat,       // 撤退
    Dead,          // 死亡
}

/// 敌人生成配置
#[derive(Debug, Clone, Resource)]
pub struct EnemySpawnConfig {
    pub scout_bot_chance: f32,
    pub combat_bot_chance: f32,
    pub heavy_bot_chance: f32,
    pub elite_bot_chance: f32,
    pub worker_bug_chance: f32,
    pub warrior_bug_chance: f32,
    pub spitter_bug_chance: f32,
    pub tank_bug_chance: f32,
    pub queen_bug_chance: f32,
}

impl Default for EnemySpawnConfig {
    fn default() -> Self {
        Self {
            scout_bot_chance: 0.2,
            combat_bot_chance: 0.3,
            heavy_bot_chance: 0.1,
            elite_bot_chance: 0.05,
            worker_bug_chance: 0.15,
            warrior_bug_chance: 0.2,
            spitter_bug_chance: 0.15,
            tank_bug_chance: 0.1,
            queen_bug_chance: 0.02,
        }
    }
}

impl EnemySpawnConfig {
    /// 随机生成敌人类型
    pub fn random_enemy_type(&self, rng: &mut impl rand::Rng) -> EnemyType {
        let roll = rng.gen::<f32>();
        let mut cumulative = 0.0;

        let types = [
            (self.scout_bot_chance, EnemyType::ScoutBot),
            (self.combat_bot_chance, EnemyType::CombatBot),
            (self.heavy_bot_chance, EnemyType::HeavyBot),
            (self.elite_bot_chance, EnemyType::EliteBot),
            (self.worker_bug_chance, EnemyType::WorkerBug),
            (self.warrior_bug_chance, EnemyType::WarriorBug),
            (self.spitter_bug_chance, EnemyType::SpitterBug),
            (self.tank_bug_chance, EnemyType::TankBug),
            (self.queen_bug_chance, EnemyType::QueenBug),
        ];

        for (chance, enemy_type) in types {
            cumulative += chance;
            if roll < cumulative {
                return enemy_type;
            }
        }

        EnemyType::CombatBot // 默认返回战斗机器人
    }
}

/// 敌人波次配置
#[derive(Debug, Clone, Resource)]
pub struct EnemyWaveConfig {
    pub current_wave: u32,
    pub enemies_per_wave: u32,
    pub wave_interval: f32,
    pub difficulty_multiplier: f32,
}

impl Default for EnemyWaveConfig {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_per_wave: 5,
            wave_interval: 30.0,
            difficulty_multiplier: 1.0,
        }
    }
}

/// 敌人大本营组件
#[derive(Component, Clone, Debug)]
pub struct EnemyBase {
    pub base_type: EnemyType,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub max_spawn_count: u32,
    pub current_spawn_count: u32,
    pub spawn_range: f32,
    pub active: bool,
}

impl EnemyBase {
    pub fn new(base_type: EnemyType) -> Self {
        let (spawn_interval, max_spawn_count, spawn_range) = match base_type {
            EnemyType::RobotFortress => (15.0, 20, 8.0),
            EnemyType::AIMotherBase => (20.0, 30, 12.0),
            _ => (10.0, 10, 5.0),
        };

        Self {
            base_type,
            spawn_timer: 0.0,
            spawn_interval,
            max_spawn_count,
            current_spawn_count: 0,
            spawn_range,
            active: true,
        }
    }

    /// 检查是否可以生成敌人
    pub fn can_spawn(&self) -> bool {
        self.active && self.current_spawn_count < self.max_spawn_count
    }

    /// 重置生成计数
    pub fn reset_spawn_count(&mut self) {
        self.current_spawn_count = 0;
    }

    /// 获取可以生成的敌人类型
    pub fn get_spawn_types(&self) -> Vec<EnemyType> {
        match self.base_type {
            EnemyType::RobotFortress => vec![
                EnemyType::ScoutBot,
                EnemyType::CombatBot,
                EnemyType::ScoutBird,
            ],
            EnemyType::AIMotherBase => vec![
                EnemyType::CombatBot,
                EnemyType::HeavyBot,
                EnemyType::EliteBot,
                EnemyType::WorkerBug,
                EnemyType::WarriorBug,
                EnemyType::SpitterBug,
            ],
            _ => vec![],
        }
    }
}

/// 敌人位置组件
#[derive(Component, Clone, Debug)]
pub struct EnemyPosition {
    pub tile_x: u32,
    pub tile_y: u32,
}

/// 敌人状态组件
#[derive(Component, Clone, Debug)]
pub struct EnemyStatus {
    pub is_spawned: bool,
    pub is_attacking: bool,
    pub is_retreating: bool,
    pub attack_timer: f32,
}

impl Default for EnemyStatus {
    fn default() -> Self {
        Self {
            is_spawned: false,
            is_attacking: false,
            is_retreating: false,
            attack_timer: 0.0,
        }
    }
}
