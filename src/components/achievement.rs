use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// 成就系统组件

/// 成就类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementType {
    Combat,       // 战斗成就
    Exploration,  // 探索成就
    Building,     // 建造成就
    Resource,     // 资源成就
    Quest,        // 任务成就
    Social,       // 社交成就
    Milestone,    // 里程碑成就
    Special,      // 特殊成就
}

/// 成就状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementStatus {
    Hidden,       // 隐藏（未解锁且未显示）
    Revealed,     // 已揭示（未解锁但已显示）
    Unlocked,     // 已解锁
}

/// 成就条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCondition {
    // 战斗条件
    KillEnemy(String, u32),           // 击杀指定敌人X次
    DealDamage(f32),                 // 造成X点伤害
    SurviveTime(f32),                // 生存X秒

    // 探索条件
    ExploreArea(String),             // 探索指定区域
    DiscoverAllAreas,                // 发现所有区域

    // 建造条件
    BuildBuilding(String, u32),      // 建造指定建筑X个
    UpgradeBuilding(String, u32),    // 升级指定建筑X次

    // 资源条件
    CollectResource(String, u32),    // 收集指定资源X个
    ReachResourceAmount(String, u32), // 拥有指定资源X个

    // 任务条件
    CompleteQuest(String),           // 完成指定任务
    CompleteQuests(u32),             // 完成X个任务
    CompleteDailyQuests(u32),        // 完成X个日常任务

    // 社交条件
    ReachLevel(u32),                 // 达到X级

    // 里程碑条件
    PlayTime(f32),                   // 游戏时间X秒
    LoginDays(u32),                  // 登录X天

    // 特殊条件
    Custom(String, u32),             // 自定义条件
}

/// 成就奖励
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    pub title: String,               // 称号
    pub experience: u32,            // 经验值
    pub gold: u32,                   // 金币
    pub items: Vec<String>,         // 物品
}

impl AchievementReward {
    pub fn new(title: String) -> Self {
        Self {
            title,
            experience: 0,
            gold: 0,
            items: Vec::new(),
        }
    }

    pub fn with_experience(mut self, experience: u32) -> Self {
        self.experience = experience;
        self
    }

    pub fn with_gold(mut self, gold: u32) -> Self {
        self.gold = gold;
        self
    }

    pub fn with_item(mut self, item_id: String) -> Self {
        self.items.push(item_id);
        self
    }
}

/// 成就组件
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub achievement_type: AchievementType,
    pub status: AchievementStatus,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub condition: AchievementCondition,
    pub reward: AchievementReward,
    pub hidden: bool,               // 是否隐藏
    pub points: u32,                 // 成就点数
}

impl Achievement {
    pub fn new(
        id: String,
        achievement_type: AchievementType,
        title: String,
        description: String,
        icon: String,
        condition: AchievementCondition,
        reward: AchievementReward,
    ) -> Self {
        Self {
            id,
            achievement_type,
            status: AchievementStatus::Hidden,
            title,
            description,
            icon,
            condition,
            reward,
            hidden: false,
            points: 10,
        }
    }

    /// 设置为隐藏
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// 设置成就点数
    pub fn with_points(mut self, points: u32) -> Self {
        self.points = points;
        self
    }

    /// 解锁成就
    pub fn unlock(&mut self) {
        if self.status != AchievementStatus::Unlocked {
            self.status = AchievementStatus::Unlocked;
        }
    }

    /// 揭示成就
    pub fn reveal(&mut self) {
        if self.status == AchievementStatus::Hidden {
            self.status = AchievementStatus::Revealed;
        }
    }

    /// 检查是否已解锁
    pub fn is_unlocked(&self) -> bool {
        self.status == AchievementStatus::Unlocked
    }

    /// 检查是否已揭示
    pub fn is_revealed(&self) -> bool {
        self.status == AchievementStatus::Revealed || self.is_unlocked()
    }
}

/// 成就日志组件
#[derive(Debug, Clone, Component)]
pub struct AchievementLog {
    pub unlocked_achievements: Vec<String>,  // 已解锁的成就ID
    pub total_points: u32,                    // 总成就点数
    pub current_streak: u32,                  // 当前连续解锁数
    pub max_streak: u32,                      // 最大连续解锁数
}

impl Default for AchievementLog {
    fn default() -> Self {
        Self {
            unlocked_achievements: Vec::new(),
            total_points: 0,
            current_streak: 0,
            max_streak: 0,
        }
    }
}

impl AchievementLog {
    /// 解锁成就
    pub fn unlock_achievement(&mut self, achievement_id: String, points: u32) {
        if !self.unlocked_achievements.contains(&achievement_id) {
            self.unlocked_achievements.push(achievement_id.clone());
            self.total_points += points;
            self.current_streak += 1;
            self.max_streak = self.max_streak.max(self.current_streak);
        }
    }

    /// 检查成就是否已解锁
    pub fn is_achievement_unlocked(&self, achievement_id: &str) -> bool {
        self.unlocked_achievements.contains(&achievement_id.to_string())
    }

    /// 获取已解锁成就数量
    pub fn get_unlocked_count(&self) -> usize {
        self.unlocked_achievements.len()
    }

    /// 重置连续解锁数
    pub fn reset_streak(&mut self) {
        self.current_streak = 0;
    }
}

/// 成就统计组件
#[derive(Debug, Clone, Component)]
pub struct AchievementStats {
    pub total_achievements: u32,        // 总成就数
    pub unlocked_achievements: u32,      // 已解锁成就数
    pub total_points: u32,              // 总成就点数
    pub earned_points: u32,             // 已获得成就点数
}

impl Default for AchievementStats {
    fn default() -> Self {
        Self {
            total_achievements: 0,
            unlocked_achievements: 0,
            total_points: 0,
            earned_points: 0,
        }
    }
}

impl AchievementStats {
    /// 更新成就统计
    pub fn update(&mut self, total: u32, unlocked: u32, total_points: u32, earned_points: u32) {
        self.total_achievements = total;
        self.unlocked_achievements = unlocked;
        self.total_points = total_points;
        self.earned_points = earned_points;
    }

    /// 获取解锁百分比
    pub fn get_unlock_percentage(&self) -> f32 {
        if self.total_achievements == 0 {
            0.0
        } else {
            (self.unlocked_achievements as f32 / self.total_achievements as f32) * 100.0
        }
    }
}
