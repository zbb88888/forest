use bevy::prelude::*;
use crate::components::quest::{Quest, QuestLog, QuestStatus};
use crate::components::player::Player;

/// 任务管理系统插件
pub struct QuestManagerPlugin;

impl Plugin for QuestManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_quest_timers.run_if(in_state(crate::states::GameState::InGame)),
            check_quest_completion.run_if(in_state(crate::states::GameState::InGame)),
            update_quest_progress.run_if(in_state(crate::states::GameState::InGame)),
        ));
    }
}

/// 更新任务计时器
fn update_quest_timers(
    time: Res<Time>,
    mut quest_query: Query<&mut Quest>,
) {
    for mut quest in quest_query.iter_mut() {
        if quest.status == QuestStatus::InProgress {
            quest.update_time_limit(time.delta_secs());
        }
    }
}

/// 检查任务完成
fn check_quest_completion(
    mut commands: Commands,
    mut quest_query: Query<(Entity, &mut Quest)>,
    mut quest_log_query: Query<&mut QuestLog>,
) {
    for (entity, mut quest) in quest_query.iter_mut() {
        if quest.status == QuestStatus::InProgress && quest.is_completed() {
            if quest.auto_complete {
                // 自动完成任务
                quest.complete();

                // 更新任务日志
                let mut quest_log = quest_log_query.single_mut();
                quest_log.complete_quest(&quest.id);

                info!("任务自动完成: {}", quest.title);
            }
        }
    }
}

/// 更新任务进度
fn update_quest_progress(
    mut quest_query: Query<&mut Quest>,
    mut quest_log_query: Query<&mut QuestLog>,
    player_query: Query<&Player>,
) {
    // 获取玩家信息
    let player_level = if let Ok(player) = player_query.single() {
        player.level
    } else {
        1
    };

    // 获取已完成任务列表
    let completed_quests = if let Ok(quest_log) = quest_log_query.single() {
        quest_log.completed_quests.clone()
    } else {
        Vec::new()
    };

    // 更新所有进行中的任务
    for mut quest in quest_query.iter_mut() {
        if quest.status == QuestStatus::InProgress {
            // 这里可以根据游戏事件更新任务进度
            // 实际实现需要监听游戏事件
        }
    }
}

/// 接受任务
pub fn accept_quest(
    commands: &mut Commands,
    quest: Quest,
    quest_log: &mut QuestLog,
    player_level: u32,
) -> Result<(), String> {
    // 检查是否可以接受任务
    if !quest.can_accept(player_level, &quest_log.completed_quests) {
        return Err("无法接受该任务".to_string());
    }

    // 添加任务到游戏世界
    commands.spawn(quest.clone());

    // 更新任务日志
    quest_log.add_quest(quest.id.clone());

    info!("接受任务: {}", quest.title);
    Ok(())
}

/// 完成任务
pub fn complete_quest(
    commands: &mut Commands,
    quest_id: &str,
    quest_log: &mut QuestLog,
    quest_stats: &mut crate::components::quest::QuestStats,
) -> Result<(), String> {
    // 检查任务是否在进行中
    if !quest_log.is_quest_active(quest_id) {
        return Err("任务未在进行中".to_string());
    }

    // 更新任务日志
    quest_log.complete_quest(quest_id);

    // 更新任务统计
    quest_stats.total_quests_completed += 1;

    info!("完成任务: {}", quest_id);
    Ok(())
}

/// 放弃任务
pub fn abandon_quest(
    commands: &mut Commands,
    quest_id: &str,
    quest_log: &mut QuestLog,
) -> Result<(), String> {
    // 检查任务是否在进行中
    if !quest_log.is_quest_active(quest_id) {
        return Err("任务未在进行中".to_string());
    }

    // 从任务日志中移除
    if let Some(index) = quest_log.active_quests.iter().position(|id| id == quest_id) {
        quest_log.active_quests.remove(index);

        if quest_log.current_quest.as_deref() == Some(quest_id) {
            quest_log.current_quest = quest_log.active_quests.first().cloned();
        }
    }

    info!("放弃任务: {}", quest_id);
    Ok(())
}

/// 设置当前追踪的任务
pub fn set_current_quest(
    quest_log: &mut QuestLog,
    quest_id: Option<String>,
) {
    quest_log.set_current_quest(quest_id);
    info!("设置当前追踪任务: {:?}", quest_id);
}

/// 获取任务进度
pub fn get_quest_progress(
    quest_id: &str,
    quest_query: &Query<&Quest>,
) -> Option<Vec<(String, u32, u32)>> {
    if let Ok(quest) = quest_query.single() {
        if quest.id == quest_id {
            let progress = quest.objectives.iter().map(|obj| {
                (
                    obj.description.clone(),
                    obj.current_count,
                    obj.target_count,
                )
            }).collect();
            return Some(progress);
        }
    }
    None
}
