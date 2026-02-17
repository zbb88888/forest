use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// 任务系统组件

/// 任务类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestType {
    Main,      // 主线任务
    Side,      // 支线任务
    Daily,     // 日常任务
    Event,     // 活动任务
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,    // 未开始
    InProgress,    // 进行中
    Completed,     // 已完成
    Failed,        // 已失败
}

/// 任务目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestObjectiveType {
    Kill,          // 击杀敌人
    Collect,       // 收集物品
    Build,         // 建造建筑
    Defend,        // 防御位置
    Explore,       // 探索区域
    Survive,       // 生存时间
    Craft,         // 制作物品
    Harvest,       // 收获植物
    Upgrade,       // 升级建筑
}

/// 任务目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub objective_type: QuestObjectiveType,
    pub target_id: Option<String>,      // 目标ID
    pub target_count: u32,              // 目标数量
    pub current_count: u32,             // 当前进度
    pub description: String,            // 描述
}

impl QuestObjective {
    pub fn new(
        objective_type: QuestObjectiveType,
        target_id: Option<String>,
        target_count: u32,
        description: String,
    ) -> Self {
        Self {
            objective_type,
            target_id,
            target_count,
            current_count: 0,
            description,
        }
    }

    /// 检查是否完成
    pub fn is_completed(&self) -> bool {
        self.current_count >= self.target_count
    }

    /// 更新进度
    pub fn update_progress(&mut self, amount: u32) {
        self.current_count = (self.current_count + amount).min(self.target_count);
    }

    /// 获取完成百分比
    pub fn get_progress_percentage(&self) -> f32 {
        if self.target_count == 0 {
            1.0
        } else {
            self.current_count as f32 / self.target_count as f32
        }
    }
}

/// 任务奖励
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestReward {
    pub experience: u32,
    pub gold: u32,
    pub items: Vec<String>,
    pub resources: Vec<(String, u32)>,
}

impl QuestReward {
    pub fn new(experience: u32, gold: u32) -> Self {
        Self {
            experience,
            gold,
            items: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn with_item(mut self, item_id: String) -> Self {
        self.items.push(item_id);
        self
    }

    pub fn with_resource(mut self, resource_id: String, amount: u32) -> Self {
        self.resources.push((resource_id, amount));
        self
    }
}

/// 任务组件
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub quest_type: QuestType,
    pub status: QuestStatus,
    pub title: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestReward,
    pub prerequisites: Vec<String>,  // 前置任务ID
    pub level_requirement: u32,      // 等级要求
    pub time_limit: Option<f32>,      // 时间限制（秒）
    pub time_remaining: Option<f32>,  // 剩余时间
    pub auto_complete: bool,          // 自动完成
}

impl Quest {
    pub fn new(
        id: String,
        quest_type: QuestType,
        title: String,
        description: String,
        rewards: QuestReward,
    ) -> Self {
        Self {
            id,
            quest_type,
            status: QuestStatus::NotStarted,
            title,
            description,
            objectives: Vec::new(),
            rewards,
            prerequisites: Vec::new(),
            level_requirement: 1,
            time_limit: None,
            time_remaining: None,
            auto_complete: false,
        }
    }

    /// 添加目标
    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    /// 添加前置任务
    pub fn with_prerequisite(mut self, quest_id: String) -> Self {
        self.prerequisites.push(quest_id);
        self
    }

    /// 设置等级要求
    pub fn with_level_requirement(mut self, level: u32) -> Self {
        self.level_requirement = level;
        self
    }

    /// 设置时间限制
    pub fn with_time_limit(mut self, time_limit: f32) -> Self {
        self.time_limit = Some(time_limit);
        self.time_remaining = Some(time_limit);
        self
    }

    /// 设置自动完成
    pub fn with_auto_complete(mut self, auto_complete: bool) -> Self {
        self.auto_complete = auto_complete;
        self
    }

    /// 开始任务
    pub fn start(&mut self) {
        if self.status == QuestStatus::NotStarted {
            self.status = QuestStatus::InProgress;
            self.time_remaining = self.time_limit;
        }
    }

    /// 完成任务
    pub fn complete(&mut self) {
        if self.status == QuestStatus::InProgress {
            self.status = QuestStatus::Completed;
        }
    }

    /// 失败任务
    pub fn fail(&mut self) {
        if self.status == QuestStatus::InProgress {
            self.status = QuestStatus::Failed;
        }
    }

    /// 检查是否可以接受
    pub fn can_accept(&self, player_level: u32, completed_quests: &[String]) -> bool {
        if self.status != QuestStatus::NotStarted {
            return false;
        }

        if player_level < self.level_requirement {
            return false;
        }

        for prereq in &self.prerequisites {
            if !completed_quests.contains(prereq) {
                return false;
            }
        }

        true
    }

    /// 检查是否完成
    pub fn is_completed(&self) -> bool {
        self.objectives.iter().all(|obj| obj.is_completed())
    }

    /// 更新任务进度
    pub fn update_progress(&mut self, objective_type: QuestObjectiveType, target_id: Option<&str>, amount: u32) {
        if self.status != QuestStatus::InProgress {
            return;
        }

        for objective in &mut self.objectives {
            if objective.objective_type == objective_type {
                if target_id.is_none() || objective.target_id.as_deref() == target_id {
                    objective.update_progress(amount);
                }
            }
        }

        // 检查是否自动完成
        if self.auto_complete && self.is_completed() {
            self.complete();
        }
    }

    /// 更新时间限制
    pub fn update_time_limit(&mut self, delta: f32) {
        if let Some(remaining) = &mut self.time_remaining {
            *remaining -= delta;
            if *remaining <= 0.0 {
                self.fail();
            }
        }
    }
}

/// 任务日志组件
#[derive(Debug, Clone, Component)]
pub struct QuestLog {
    pub active_quests: Vec<String>,      // 进行中的任务ID
    pub completed_quests: Vec<String>,   // 已完成的任务ID
    pub failed_quests: Vec<String>,      // 已失败的任务ID
    pub current_quest: Option<String>,   // 当前追踪的任务ID
}

impl Default for QuestLog {
    fn default() -> Self {
        Self {
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            failed_quests: Vec::new(),
            current_quest: None,
        }
    }
}

impl QuestLog {
    /// 添加任务
    pub fn add_quest(&mut self, quest_id: String) {
        if !self.active_quests.contains(&quest_id) {
            self.active_quests.push(quest_id.clone());
            if self.current_quest.is_none() {
                self.current_quest = Some(quest_id);
            }
        }
    }

    /// 完成任务
    pub fn complete_quest(&mut self, quest_id: &str) {
        if let Some(index) = self.active_quests.iter().position(|id| id == quest_id) {
            self.active_quests.remove(index);
            self.completed_quests.push(quest_id.to_string());

            if self.current_quest.as_deref() == Some(quest_id) {
                self.current_quest = self.active_quests.first().cloned();
            }
        }
    }

    /// 失败任务
    pub fn fail_quest(&mut self, quest_id: &str) {
        if let Some(index) = self.active_quests.iter().position(|id| id == quest_id) {
            self.active_quests.remove(index);
            self.failed_quests.push(quest_id.to_string());

            if self.current_quest.as_deref() == Some(quest_id) {
                self.current_quest = self.active_quests.first().cloned();
            }
        }
    }

    /// 设置当前追踪的任务
    pub fn set_current_quest(&mut self, quest_id: Option<String>) {
        if let Some(id) = &quest_id {
            if self.active_quests.contains(id) {
                self.current_quest = quest_id;
            }
        } else {
            self.current_quest = None;
        }
    }

    /// 检查任务是否已完成
    pub fn is_quest_completed(&self, quest_id: &str) -> bool {
        self.completed_quests.contains(&quest_id.to_string())
    }

    /// 检查任务是否已失败
    pub fn is_quest_failed(&self, quest_id: &str) -> bool {
        self.failed_quests.contains(&quest_id.to_string())
    }

    /// 检查任务是否在进行中
    pub fn is_quest_active(&self, quest_id: &str) -> bool {
        self.active_quests.contains(&quest_id.to_string())
    }
}

/// 任务统计组件
#[derive(Debug, Clone, Component)]
pub struct QuestStats {
    pub total_quests_completed: u32,
    pub total_quests_failed: u32,
    pub total_experience_gained: u32,
    pub total_gold_gained: u32,
}

impl Default for QuestStats {
    fn default() -> Self {
        Self {
            total_quests_completed: 0,
            total_quests_failed: 0,
            total_experience_gained: 0,
            total_gold_gained: 0,
        }
    }
}
