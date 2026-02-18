use bevy::prelude::*;
use crate::components::achievement::{Achievement, AchievementCondition, AchievementLog};

pub struct AchievementEventsPlugin;

impl Plugin for AchievementEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AchievementProgressEvent>()
            .add_observer(handle_achievement_progress);
    }
}

#[derive(Event, Message, Debug, Clone)]
pub struct AchievementProgressEvent {
    pub condition_type: AchievementConditionType,
    pub target_id: Option<String>,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchievementConditionType {
    KillEnemy,
    DealDamage,
    SurviveTime,
    ExploreArea,
    BuildBuilding,
    UpgradeBuilding,
    CollectResource,
    CompleteQuest,
    CompleteDailyQuest,
    ReachLevel,
    PlayTime,
    Custom,
}

fn handle_achievement_progress(
    event: On<AchievementProgressEvent>,
    mut achievement_query: Query<&mut Achievement>,
    achievement_log_query: Query<&mut AchievementLog>,
) {
    let achievement_event = event.event();

    let _unlocked_achievements = if let Ok(log) = achievement_log_query.single() {
        log.unlocked_achievements.clone()
    } else {
        return;
    };

    for mut achievement in achievement_query.iter_mut() {
        if achievement.is_unlocked() {
            continue;
        }

        if matches_achievement_condition(&achievement.condition, achievement_event) {
            if achievement.status == crate::components::achievement::AchievementStatus::Hidden {
                achievement.reveal();
            }

            info!("成就进度更新: 成就={}, 类型={:?}, 数量={}",
                achievement.title, achievement_event.condition_type, achievement_event.amount);
        }
    }
}

fn matches_achievement_condition(
    condition: &AchievementCondition,
    event: &AchievementProgressEvent,
) -> bool {
    match condition {
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

        AchievementCondition::ExploreArea(area_id) => {
            event.condition_type == AchievementConditionType::ExploreArea
                && event.target_id.as_ref().map_or(true, |id| id == area_id)
        }

        AchievementCondition::DiscoverAllAreas => {
            event.condition_type == AchievementConditionType::ExploreArea
        }

        AchievementCondition::BuildBuilding(building_id, _) => {
            event.condition_type == AchievementConditionType::BuildBuilding
                && event.target_id.as_ref().map_or(true, |id| id == building_id)
        }

        AchievementCondition::UpgradeBuilding(building_id, _) => {
            event.condition_type == AchievementConditionType::UpgradeBuilding
                && event.target_id.as_ref().map_or(true, |id| id == building_id)
        }

        AchievementCondition::CollectResource(resource_id, _) => {
            event.condition_type == AchievementConditionType::CollectResource
                && event.target_id.as_ref().map_or(true, |id| id == resource_id)
        }

        AchievementCondition::ReachResourceAmount(resource_id, _) => {
            event.condition_type == AchievementConditionType::CollectResource
                && event.target_id.as_ref().map_or(true, |id| id == resource_id)
        }

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

        AchievementCondition::ReachLevel(_) => {
            event.condition_type == AchievementConditionType::ReachLevel
        }

        AchievementCondition::PlayTime(_) => {
            event.condition_type == AchievementConditionType::PlayTime
        }

        AchievementCondition::LoginDays(_) => {
            false
        }

        AchievementCondition::Custom(_, _) => {
            event.condition_type == AchievementConditionType::Custom
        }
    }
}

pub fn send_kill_enemy_event(
    mut commands: Commands,
    enemy_id: String,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::KillEnemy,
        target_id: Some(enemy_id),
        amount,
    });
}

pub fn send_deal_damage_event(
    mut commands: Commands,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::DealDamage,
        target_id: None,
        amount,
    });
}

pub fn send_explore_area_event(
    mut commands: Commands,
    area_id: String,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::ExploreArea,
        target_id: Some(area_id),
        amount: 1,
    });
}

pub fn send_build_building_event(
    mut commands: Commands,
    building_id: String,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::BuildBuilding,
        target_id: Some(building_id),
        amount,
    });
}

pub fn send_upgrade_building_event(
    mut commands: Commands,
    building_id: String,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::UpgradeBuilding,
        target_id: Some(building_id),
        amount,
    });
}

pub fn send_collect_resource_event(
    mut commands: Commands,
    resource_id: String,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::CollectResource,
        target_id: Some(resource_id),
        amount,
    });
}

pub fn send_complete_quest_event(
    mut commands: Commands,
    quest_id: String,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::CompleteQuest,
        target_id: Some(quest_id),
        amount: 1,
    });
}

pub fn send_complete_daily_quest_event(
    mut commands: Commands,
    amount: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::CompleteDailyQuest,
        target_id: None,
        amount,
    });
}

pub fn send_reach_level_event(
    mut commands: Commands,
    level: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::ReachLevel,
        target_id: None,
        amount: level,
    });
}

pub fn send_play_time_event(
    mut commands: Commands,
    seconds: u32,
) {
    commands.trigger(AchievementProgressEvent {
        condition_type: AchievementConditionType::PlayTime,
        target_id: None,
        amount: seconds,
    });
}