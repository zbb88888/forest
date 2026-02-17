use bevy::prelude::*;

/// 机器人类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotType {
    Harvester,  // 采集机器人
    Builder,    // 建造机器人
    Defender,   // 防御机器人
    Scout,      // 侦察机器人
}

impl RobotType {
    /// 获取机器人的颜色
    pub fn color(&self) -> Color {
        match self {
            RobotType::Harvester => Color::srgb(0.8, 0.6, 0.4),  // 棕色
            RobotType::Builder => Color::srgb(0.6, 0.8, 0.4),   // 绿色
            RobotType::Defender => Color::srgb(0.8, 0.4, 0.4),  // 红色
            RobotType::Scout => Color::srgb(0.4, 0.6, 0.8),     // 蓝色
        }
    }

    /// 获取机器人的移动速度
    pub fn movement_speed(&self) -> f32 {
        match self {
            RobotType::Harvester => 2.0,
            RobotType::Builder => 1.5,
            RobotType::Defender => 2.5,
            RobotType::Scout => 3.0,
        }
    }

    /// 获取机器人的能量消耗速率
    pub fn energy_consumption(&self) -> f32 {
        match self {
            RobotType::Harvester => 0.5,
            RobotType::Builder => 0.8,
            RobotType::Defender => 0.6,
            RobotType::Scout => 0.3,
        }
    }
}

/// 机器人任务类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotTask {
    Idle,           // 空闲
    Harvest,        // 采集
    Build,          // 建造
    Patrol,         // 巡逻
    Repair,         // 维修
    ReturnToBase,   // 返回基地
}

/// 机器人组件
#[derive(Component, Clone, Debug)]
pub struct Robot {
    pub robot_type: RobotType,
    pub current_task: RobotTask,
    pub energy: f32,
    pub max_energy: f32,
    pub efficiency: f32,
    pub target_position: Option<Vec2>,
    pub task_timer: Timer,
}

impl Robot {
    pub fn new(robot_type: RobotType) -> Self {
        let max_energy = match robot_type {
            RobotType::Harvester => 100.0,
            RobotType::Builder => 150.0,
            RobotType::Defender => 120.0,
            RobotType::Scout => 80.0,
        };

        Self {
            robot_type,
            current_task: RobotTask::Idle,
            energy: max_energy,
            max_energy,
            efficiency: 1.0,
            target_position: None,
            task_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }

    /// 检查机器人是否有足够能量执行任务
    pub fn has_energy(&self, amount: f32) -> bool {
        self.energy >= amount
    }

    /// 消耗能量
    pub fn consume_energy(&mut self, amount: f32) {
        self.energy = (self.energy - amount).max(0.0);
    }

    /// 充能
    pub fn recharge(&mut self, amount: f32) {
        self.energy = (self.energy + amount).min(self.max_energy);
    }
}

/// 机器人 AI 组件
#[derive(Component, Clone, Debug)]
pub struct RobotAI {
    pub patrol_radius: f32,
    pub detection_radius: f32,
    pub current_patrol_index: usize,
    pub patrol_points: Vec<Vec2>,
}

impl Default for RobotAI {
    fn default() -> Self {
        Self {
            patrol_radius: 200.0,
            detection_radius: 100.0,
            current_patrol_index: 0,
            patrol_points: Vec::new(),
        }
    }
}

/// 机器人资源携带组件
#[derive(Component, Clone, Debug)]
pub struct RobotInventory {
    pub capacity: u32,
    pub current: u32,
    pub resource_type: Option<crate::components::resource::ResourceType>,
}

impl RobotInventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            current: 0,
            resource_type: None,
        }
    }

    /// 添加资源
    pub fn add(&mut self, amount: u32, resource_type: crate::components::resource::ResourceType) -> bool {
        if self.current + amount <= self.capacity {
            if self.resource_type.is_none() {
                self.resource_type = Some(resource_type);
            }
            self.current += amount;
            true
        } else {
            false
        }
    }

    /// 清空资源
    pub fn clear(&mut self) -> u32 {
        let amount = self.current;
        self.current = 0;
        self.resource_type = None;
        amount
    }

    /// 检查是否已满
    pub fn is_full(&self) -> bool {
        self.current >= self.capacity
    }
}
