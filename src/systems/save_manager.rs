use bevy::prelude::*;
use crate::components::save::{
    SaveManager, SaveType, SaveStatus, SaveInfo, SaveData,
    PlayerData, WorldData, QuestData, AchievementData
};

/// 存档管理系统插件
pub struct SaveManagerPlugin;

impl Plugin for SaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_auto_save,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 处理自动存档
fn handle_auto_save(
    time: Res<Time>,
    mut save_manager_query: Query<&mut SaveManager>,
) {
    if let Ok(mut save_manager) = save_manager_query.get_single_mut() {
        let current_time = time.elapsed_secs();

        if save_manager.should_auto_save(current_time) {
            info!("触发自动存档");
            // 实际存档操作需要在主线程中执行
            // 这里只是触发存档事件
        }
    }
}

/// 保存游戏
pub fn save_game(
    world: &World,
    save_type: SaveType,
    description: Option<String>,
) -> Result<String, String> {
    // 获取存档管理器
    let save_manager = world.get_resource::<SaveManager>()
        .ok_or("存档管理器不存在")?;

    // 生成存档ID
    let save_id = save_manager.generate_save_id(save_type);

    // 收集游戏数据
    let save_data = collect_save_data(world, &save_id, save_type, description)?;

    // 序列化数据
    let serialized = serde_json::to_string(&save_data)
        .map_err(|e| format!("序列化失败: {}", e))?;

    // 获取存档路径
    let save_path = save_manager.get_save_path(&save_id);

    // 确保存档目录存在
    std::fs::create_dir_all(&save_manager.settings.save_location)
        .map_err(|e| format!("创建存档目录失败: {}", e))?;

    // 写入文件
    std::fs::write(&save_path, serialized)
        .map_err(|e| format!("写入存档文件失败: {}", e))?;

    info!("游戏保存成功: {}", save_id);
    Ok(save_id)
}

/// 加载游戏
pub fn load_game(
    world: &mut World,
    save_id: &str,
) -> Result<SaveData, String> {
    // 获取存档管理器
    let save_manager = world.get_resource::<SaveManager>()
        .ok_or("存档管理器不存在")?;

    // 获取存档路径
    let save_path = save_manager.get_save_path(save_id);

    // 读取文件
    let serialized = std::fs::read_to_string(&save_path)
        .map_err(|e| format!("读取存档文件失败: {}", e))?;

    // 反序列化数据
    let save_data: SaveData = serde_json::from_str(&serialized)
        .map_err(|e| format!("反序列化失败: {}", e))?;

    // 检查版本兼容性
    check_version_compatibility(&save_data.info.version)?;

    // 应用存档数据
    apply_save_data(world, &save_data)?;

    info!("游戏加载成功: {}", save_id);
    Ok(save_data)
}

/// 收集存档数据
fn collect_save_data(
    world: &World,
    save_id: &str,
    save_type: SaveType,
    description: Option<String>,
) -> Result<SaveData, String> {
    // 收集玩家数据
    let player_data = collect_player_data(world)?;

    // 收集世界数据
    let world_data = collect_world_data(world)?;

    // 收集任务数据
    let quest_data = collect_quest_data(world)?;

    // 收集成就数据
    let achievement_data = collect_achievement_data(world)?;

    // 创建存档信息
    let mut info = SaveInfo::new(
        save_id.to_string(),
        save_type,
        "Player".to_string(), // 实际应该从玩家系统获取
        player_data.level,
        world_data.time,
        "Forest".to_string(), // 实际应该从地图系统获取
        "0.8.0".to_string(), // 游戏版本
    );

    if let Some(desc) = description {
        info = info.with_description(desc);
    }

    Ok(SaveData {
        info,
        player_data,
        world_data,
        quest_data,
        achievement_data,
    })
}

/// 收集玩家数据
fn collect_player_data(world: &World) -> Result<PlayerData, String> {
    // 实际实现需要从玩家系统获取数据
    Ok(PlayerData {
        level: 1,
        experience: 0,
        health: 100.0,
        max_health: 100.0,
        position: (0.0, 0.0),
        inventory: Vec::new(),
        equipment: crate::components::save::EquipmentData {
            weapon: None,
            armor: None,
            accessory: None,
        },
    })
}

/// 收集世界数据
fn collect_world_data(world: &World) -> Result<WorldData, String> {
    // 实际实现需要从世界系统获取数据
    Ok(WorldData {
        time: 0.0,
        day: 1,
        explored_areas: Vec::new(),
        buildings: Vec::new(),
        resources: Vec::new(),
    })
}

/// 收集任务数据
fn collect_quest_data(world: &World) -> Result<QuestData, String> {
    // 实际实现需要从任务系统获取数据
    Ok(QuestData {
        active_quests: Vec::new(),
        completed_quests: Vec::new(),
        failed_quests: Vec::new(),
        current_quest: None,
    })
}

/// 收集成就数据
fn collect_achievement_data(world: &World) -> Result<AchievementData, String> {
    // 实际实现需要从成就系统获取数据
    Ok(AchievementData {
        unlocked_achievements: Vec::new(),
        total_points: 0,
    })
}

/// 应用存档数据
fn apply_save_data(
    world: &mut World,
    save_data: &SaveData,
) -> Result<(), String> {
    // 应用玩家数据
    apply_player_data(world, &save_data.player_data)?;

    // 应用世界数据
    apply_world_data(world, &save_data.world_data)?;

    // 应用任务数据
    apply_quest_data(world, &save_data.quest_data)?;

    // 应用成就数据
    apply_achievement_data(world, &save_data.achievement_data)?;

    Ok(())
}

/// 应用玩家数据
fn apply_player_data(
    world: &mut World,
    player_data: &PlayerData,
) -> Result<(), String> {
    // 实际实现需要应用到玩家系统
    info!("应用玩家数据: 等级={}, 位置={:?}", player_data.level, player_data.position);
    Ok(())
}

/// 应用世界数据
fn apply_world_data(
    world: &mut World,
    world_data: &WorldData,
) -> Result<(), String> {
    // 实际实现需要应用到世界系统
    info!("应用世界数据: 时间={}, 天数={}", world_data.time, world_data.day);
    Ok(())
}

/// 应用任务数据
fn apply_quest_data(
    world: &mut World,
    quest_data: &QuestData,
) -> Result<(), String> {
    // 实际实现需要应用到任务系统
    info!("应用任务数据: 进行中={}, 已完成={}", 
        quest_data.active_quests.len(), 
        quest_data.completed_quests.len());
    Ok(())
}

/// 应用成就数据
fn apply_achievement_data(
    world: &mut World,
    achievement_data: &AchievementData,
) -> Result<(), String> {
    // 实际实现需要应用到成就系统
    info!("应用成就数据: 解锁={}, 点数={}", 
        achievement_data.unlocked_achievements.len(), 
        achievement_data.total_points);
    Ok(())
}

/// 检查版本兼容性
fn check_version_compatibility(version: &str) -> Result<(), String> {
    let current_version = "0.8.0";

    // 简单版本检查
    if version != current_version {
        return Err(format!("存档版本({})不兼容当前版本({})", version, current_version));
    }

    Ok(())
}

/// 获取存档列表
pub fn get_save_list(save_manager: &SaveManager) -> Result<Vec<SaveInfo>, String> {
    let save_dir = &save_manager.settings.save_location;

    // 读取存档目录
    let entries = std::fs::read_dir(save_dir)
        .map_err(|e| format!("读取存档目录失败: {}", e))?;

    let mut saves = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("读取存档条目失败: {}", e))?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            // 读取存档文件
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("读取存档文件失败: {}", e))?;

            // 解析存档信息
            let save_data: SaveData = serde_json::from_str(&content)
                .map_err(|e| format!("解析存档文件失败: {}", e))?;

            saves.push(save_data.info);
        }
    }

    // 按时间排序
    saves.sort_by(|a, b| b.real_time.partial_cmp(&a.real_time).unwrap());

    Ok(saves)
}

/// 删除存档
pub fn delete_save(save_manager: &SaveManager, save_id: &str) -> Result<(), String> {
    let save_path = save_manager.get_save_path(save_id);

    std::fs::remove_file(&save_path)
        .map_err(|e| format!("删除存档文件失败: {}", e))?;

    info!("存档删除成功: {}", save_id);
    Ok(())
}
