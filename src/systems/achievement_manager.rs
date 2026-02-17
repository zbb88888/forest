use bevy::prelude::*;
use crate::components::achievement::{
    Achievement, AchievementLog, AchievementStatus, AchievementCondition
};

/// 成就管理系统插件
pub struct AchievementManagerPlugin;

impl Plugin for AchievementManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_achievements,
            update_achievement_stats,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 检查成就解锁条件
fn check_achievements(
    mut commands: Commands,
    mut achievement_query: Query<&mut Achievement>,
    mut achievement_log_query: Query<&mut AchievementLog>,
    player_query: Query<&crate::components::player::Player>,
    time: Res<Time>,
) {
    // 获取玩家信息
    let player_level = if let Ok(player) = player_query.get_single() {
        player.level
    } else {
        1
    };

    let play_time = time.elapsed_secs();

    // 检查每个成就
    for mut achievement in achievement_query.iter_mut() {
        // 跳过已解锁的成就
        if achievement.is_unlocked() {
            continue;
        }

        // 检查成就条件
        let should_unlock = check_achievement_condition(
            &achievement.condition,
            player_level,
            play_time,
            &mut achievement_log_query,
        );

        if should_unlock {
            // 解锁成就
            achievement.unlock();

            // 更新成就日志
            if let Ok(mut log) = achievement_log_query.get_single_mut() {
                log.unlock_achievement(achievement.id.clone(), achievement.points);
            }

            info!("成就解锁: {} ({})", achievement.title, achievement.id);
        }
    }
}

/// 检查成就条件
fn check_achievement_condition(
    condition: &AchievementCondition,
    player_level: u32,
    play_time: f32,
    achievement_log_query: &mut Query<&mut AchievementLog>,
) -> bool {
    match condition {
        // 社交条件：达到指定等级
        AchievementCondition::ReachLevel(level) => {
            player_level >= *level
        }

        // 里程碑条件：游戏时间
        AchievementCondition::PlayTime(seconds) => {
            play_time >= *seconds
        }

        // 里程碑条件：登录天数（简化处理）
        AchievementCondition::LoginDays(days) => {
            // 实际实现需要记录登录历史
            play_time >= (*days as f32 * 24.0 * 60.0 * 60.0)
        }

        // 自定义条件
        AchievementCondition::Custom(_, value) => {
            // 实际实现需要根据具体条件检查
            false
        }

        // 其他条件需要额外数据支持
        _ => false,
    }
}

/// 更新成就统计
fn update_achievement_stats(
    achievement_query: Query<&Achievement>,
    achievement_log_query: Query<&AchievementLog>,
    mut stats_query: Query<&mut crate::components::achievement::AchievementStats>,
) {
    // 获取成就总数
    let total_achievements = achievement_query.iter().len() as u32;

    // 获取已解锁成就数
    let unlocked_count = if let Ok(log) = achievement_log_query.get_single() {
        log.get_unlocked_count() as u32
    } else {
        0
    };

    // 计算总成就点数
    let total_points: u32 = achievement_query.iter().map(|a| a.points).sum();

    // 获取已获得成就点数
    let earned_points = if let Ok(log) = achievement_log_query.get_single() {
        log.total_points
    } else {
        0
    };

    // 更新统计
    if let Ok(mut stats) = stats_query.get_single_mut() {
        stats.update(total_achievements, unlocked_count, total_points, earned_points);
    }
}

/// 解锁成就
pub fn unlock_achievement(
    commands: &mut Commands,
    achievement_id: String,
    achievement_log: &mut AchievementLog,
) -> Result<(), String> {
    // 检查成就是否已解锁
    if achievement_log.is_achievement_unlocked(&achievement_id) {
        return Err("成就已解锁".to_string());
    }

    // 获取成就点数
    let points = 10; // 默认点数

    // 更新成就日志
    achievement_log.unlock_achievement(achievement_id.clone(), points);

    info!("成就解锁: {}", achievement_id);
    Ok(())
}

/// 检查成就是否已解锁
pub fn is_achievement_unlocked(
    achievement_log: &AchievementLog,
    achievement_id: &str,
) -> bool {
    achievement_log.is_achievement_unlocked(achievement_id)
}

/// 获取成就进度
pub fn get_achievement_progress(
    achievement_id: &str,
    achievement_query: &Query<&Achievement>,
) -> Option<f32> {
    if let Ok(achievement) = achievement_query.get_single() {
        if achievement.id == achievement_id {
            if achievement.is_unlocked() {
                return Some(1.0);
            } else {
                return Some(0.0);
            }
        }
    }
    None
}

/// 重置连续解锁数
pub fn reset_achievement_streak(achievement_log: &mut AchievementLog) {
    achievement_log.reset_streak();
    info!("成就连续解锁数已重置");
}
