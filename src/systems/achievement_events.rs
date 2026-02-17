use bevy::prelude::*;
use crate::components::achievement::{Achievement, AchievementCondition, AchievementLog};

/// 成就事件系统插件
pub struct AchievementEventsPlugin;

impl Plugin for AchievementEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AchievementProgressEvent>()
            .add_systems(Update, (
                handle_achievement_progress_events,
            ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 成就进度事件
#[derive(Event, Debug, Clone)]
pub struct AchievementProgressEvent {
    pub condition_type: AchievementConditionType,
    pub target_id: Option<String>,
    pub amount: u32,
}

/// 成就条件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementConditionType {
    KillEnemy,          // 击杀敌人
    DealDamage,         // 造成伤害
    SurviveTime,        // 生存时间
    ExploreArea,        // 探索区域
    BuildBuilding,      // 建造建筑
    UpgradeBuilding,    // 升级建筑
    CollectResource,    // 收集资源
    CompleteQuest,      // 完成任务
    CompleteDailyQuest, // 完成日常任务
    ReachLevel,         // 达到等级
    PlayTime,           // 游戏时间
    Custom,             // 自定义
}

/// 处理成就进度事件
fn handle_achievement_progress_events(
    mut events: Event<AchievementProgressEvent>,
    mut achievement_query: Query<&mut Achievement>,
    mut achievement_log_query: Query<&mut AchievementLog>,
) {
    // 获取已解锁的成就列表
    let unlocked_achievements = if let Ok(log) = achievement_log_query.get_single() {
        log.unlocked_achievements.clone()
    } else {
        return;
    };

    // 处理每个进度事件
    for event in events.read() {
        // 更新所有相关成就
        for mut achievement in achievement_query.iter_mut() {
            // 跳过已解锁的成就
            if achievement.is_unlocked() {
                continue;
            }

            // 检查成就是否与事件匹配
            if matches_achievement_condition(&achievement.condition, event) {
                // 揭示隐藏成就
                if achievement.status == crate::components::achievement::AchievementStatus::Hidden {
                    achievement.reveal();
                }

                // 记录进度更新
                info!("成就进度更新: 成就={}, 类型={:?}, 数量={}",
                    achievement.title, event.condition_type, event.amount);
            }
        }
    }
}

/// 检查成就条件是否匹配事件
fn matches_achievement_condition(
    condition: &AchievementCondition,
    event: &AchievementProgressEvent,
) -> bool {
    match condition {
        // 战斗条件
        AchievementCondition::KillEnemy(enemy_id, _) => {
            event.condition_type == AchievementConditionType::KillEnemy
                && event.target_id.as_ref().map_or(true, |id| id == enemy_id)
        }

        AchievementCondition::DealDamage(_) => {
            event.condition_type == AchievementConditionType::DealDamage
        }

        AchievementCondition::SurviveTime(_) => {
            event.condition_type == AchievementConditionType::SurviveTime
        }

        // 探索条件
        AchievementCondition::ExploreArea(area_id) => {
            event.condition_type == AchievementConditionType::ExploreArea
                && event.target_id.as_ref().map_or(true, |id| id == area_id)
        }

        AchievementCondition::DiscoverAllAreas => {
            event.condition_type == AchievementConditionType::ExploreArea
        }

        // 建造条件
        AchievementCondition::BuildBuilding(building_id, _) => {
            event.condition_type == AchievementConditionType::BuildBuilding
                && event.target_id.as_ref().map_or(true, |id| id == building_id)
        }

        AchievementCondition::UpgradeBuilding(building_id, _) => {
            event.condition_type == AchievementConditionType::UpgradeBuilding
                && event.target_id.as_ref().map_or(true, |id| id == building_id)
        }

        // 资源条件
        AchievementCondition::CollectResource(resource_id, _) => {
            event.condition_type == AchievementConditionType::CollectResource
                && event.target_id.as_ref().map_or(true, |id| id == resource_id)
        }

        AchievementCondition::ReachResourceAmount(resource_id, _) => {
            event.condition_type == AchievementConditionType::CollectResource
                && event.target_id.as_ref().map_or(true, |id| id == resource_id)
        }

        // 任务条件
        AchievementCondition::CompleteQuest(quest_id) => {
            event.condition_type == AchievementConditionType::CompleteQuest
                && event.target_id.as_ref().map_or(true, |id| id == quest_id)
        }

        AchievementCondition::CompleteQuests(_) => {
            event.condition_type == AchievementConditionType::CompleteQuest
        }

        AchievementCondition::CompleteDailyQuests(_) => {
            event.condition_type == AchievementConditionType::CompleteDailyQuest
        }

        // 社交条件
        AchievementCondition::ReachLevel(_) => {
            event.condition_type == AchievementConditionType::ReachLevel
        }

        // 里程碑条件
        AchievementCondition::PlayTime(_) => {
            event.condition_type == AchievementConditionType::PlayTime
        }

        AchievementCondition::LoginDays(_) => {
            // 登录天数不通过事件更新
            false
        }

        // 自定义条件
        AchievementCondition::Custom(_, _) => {
            event.condition_type == AchievementConditionType::Custom
        }
    }
}

/// 发送击杀敌人事件
pub fn send_kill_enemy_event(
    enemy_id: String,
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::KillEnemy,
        target_id: Some(enemy_id),
        amount,
    });
}

/// 发送造成伤害事件
pub fn send_deal_damage_event(
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::DealDamage,
        target_id: None,
        amount,
    });
}

/// 发送探索区域事件
pub fn send_explore_area_event(
    area_id: String,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::ExploreArea,
        target_id: Some(area_id),
        amount: 1,
    });
}

/// 发送建造建筑事件
pub fn send_build_building_event(
    building_id: String,
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::BuildBuilding,
        target_id: Some(building_id),
        amount,
    });
}

/// 发送升级建筑事件
pub fn send_upgrade_building_event(
    building_id: String,
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::UpgradeBuilding,
        target_id: Some(building_id),
        amount,
    });
}

/// 发送收集资源事件
pub fn send_collect_resource_event(
    resource_id: String,
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::CollectResource,
        target_id: Some(resource_id),
        amount,
    });
}

/// 发送完成任务事件
pub fn send_complete_quest_event(
    quest_id: String,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::CompleteQuest,
        target_id: Some(quest_id),
        amount: 1,
    });
}

/// 发送完成日常任务事件
pub fn send_complete_daily_quest_event(
    amount: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::CompleteDailyQuest,
        target_id: None,
        amount,
    });
}

/// 发送达到等级事件
pub fn send_reach_level_event(
    level: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::ReachLevel,
        target_id: None,
        amount: level,
    });
}

/// 发送游戏时间事件
pub fn send_play_time_event(
    seconds: u32,
    events: &mut Event<AchievementProgressEvent>,
) {
    events.send(AchievementProgressEvent {
        condition_type: AchievementConditionType::PlayTime,
        target_id: None,
        amount: seconds,
    });
}
