use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// 存档系统组件

/// 存档类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaveType {
    Auto,       // 自动存档
    Manual,     // 手动存档
    Quick,      // 快速存档
    Checkpoint, // 检查点存档
}

/// 存档状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaveStatus {
    Success,    // 成功
    Failed,     // 失败
    Corrupted,  // 损坏
    Incompatible, // 不兼容
}

/// 存档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveInfo {
    pub save_id: String,              // 存档ID
    pub save_type: SaveType,          // 存档类型
    pub player_name: String,          // 玩家名称
    pub player_level: u32,            // 玩家等级
    pub game_time: f32,               // 游戏时间（秒）
    pub real_time: f64,               // 真实时间（Unix时间戳）
    pub location: String,             // 位置
    pub version: String,              // 游戏版本
    pub screenshot: Option<String>,   // 截图（Base64编码）
    pub description: String,          // 描述
}

impl SaveInfo {
    pub fn new(
        save_id: String,
        save_type: SaveType,
        player_name: String,
        player_level: u32,
        game_time: f32,
        location: String,
        version: String,
    ) -> Self {
        Self {
            save_id,
            save_type,
            player_name,
            player_level,
            game_time,
            real_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            location,
            version,
            screenshot: None,
            description: String::new(),
        }
    }

    /// 设置截图
    pub fn with_screenshot(mut self, screenshot: String) -> Self {
        self.screenshot = Some(screenshot);
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

/// 存档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub info: SaveInfo,
    pub player_data: PlayerData,
    pub world_data: WorldData,
    pub quest_data: QuestData,
    pub achievement_data: AchievementData,
}

/// 玩家数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub level: u32,
    pub experience: u32,
    pub health: f32,
    pub max_health: f32,
    pub position: (f32, f32),
    pub inventory: Vec<ItemData>,
    pub equipment: EquipmentData,
}

/// 物品数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub item_id: String,
    pub quantity: u32,
}

/// 装备数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentData {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub accessory: Option<String>,
}

/// 世界数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldData {
    pub time: f32,
    pub day: u32,
    pub explored_areas: Vec<String>,
    pub buildings: Vec<BuildingData>,
    pub resources: Vec<ResourceData>,
}

/// 建筑数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingData {
    pub building_id: String,
    pub position: (f32, f32),
    pub level: u32,
    pub health: f32,
}

/// 资源数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceData {
    pub resource_id: String,
    pub amount: u32,
}

/// 任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub active_quests: Vec<String>,
    pub completed_quests: Vec<String>,
    pub failed_quests: Vec<String>,
    pub current_quest: Option<String>,
}

/// 成就数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementData {
    pub unlocked_achievements: Vec<String>,
    pub total_points: u32,
}

/// 存档设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSettings {
    pub auto_save_enabled: bool,          // 自动存档启用
    pub auto_save_interval: f32,          // 自动存档间隔（秒）
    pub max_auto_saves: usize,            // 最大自动存档数
    pub max_manual_saves: usize,          // 最大手动存档数
    pub save_location: String,            // 存档位置
    pub compress_saves: bool,             // 压缩存档
}

impl Default for SaveSettings {
    fn default() -> Self {
        Self {
            auto_save_enabled: true,
            auto_save_interval: 300.0, // 5分钟
            max_auto_saves: 5,
            max_manual_saves: 10,
            save_location: "saves".to_string(),
            compress_saves: true,
        }
    }
}

/// 存档组件
#[derive(Debug, Clone, Component)]
pub struct SaveManager {
    pub settings: SaveSettings,
    pub last_auto_save: Option<f32>,
    pub current_save_id: Option<String>,
}

impl Default for SaveManager {
    fn default() -> Self {
        Self {
            settings: SaveSettings::default(),
            last_auto_save: None,
            current_save_id: None,
        }
    }
}

impl SaveManager {
    /// 更新自动存档计时器
    pub fn update_auto_save_timer(&mut self, current_time: f32) -> bool {
        if !self.settings.auto_save_enabled {
            return false;
        }

        match self.last_auto_save {
            Some(last) => {
                let elapsed = current_time - last;
                if elapsed >= self.settings.auto_save_interval {
                    self.last_auto_save = Some(current_time);
                    true
                } else {
                    false
                }
            }
            None => {
                self.last_auto_save = Some(current_time);
                true
            }
        }
    }

    /// 检查是否应该自动存档
    pub fn should_auto_save(&self, current_time: f32) -> bool {
        if !self.settings.auto_save_enabled {
            return false;
        }

        match self.last_auto_save {
            Some(last) => {
                let elapsed = current_time - last;
                elapsed >= self.settings.auto_save_interval
            }
            None => true,
        }
    }

    /// 生成存档ID
    pub fn generate_save_id(&self, save_type: SaveType) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let prefix = match save_type {
            SaveType::Auto => "auto",
            SaveType::Manual => "manual",
            SaveType::Quick => "quick",
            SaveType::Checkpoint => "checkpoint",
        };

        format!("{}_{}", prefix, timestamp)
    }

    /// 获取存档路径
    pub fn get_save_path(&self, save_id: &str) -> String {
        format!("{}/{}.json", self.settings.save_location, save_id)
    }
}
