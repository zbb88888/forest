use bevy::prelude::*;
use crate::components::quest::{Quest, QuestObjectiveType, QuestLog};

/// 任务事件系统插件
pub struct QuestEventsPlugin;

impl Plugin for QuestEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuestProgressEvent>()
            .add_systems(Update, (
                handle_quest_progress_events,
            ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 任务进度事件
#[derive(Event, Debug, Clone)]
pub struct QuestProgressEvent {
    pub objective_type: QuestObjectiveType,
    pub target_id: Option<String>,
    pub amount: u32,
}

/// 处理任务进度事件
fn handle_quest_progress_events(
    mut events: Event<QuestProgressEvent>,
    mut quest_query: Query<&mut Quest>,
    quest_log_query: Query<&QuestLog>,
) {
    // 获取进行中的任务ID列表
    let active_quest_ids = if let Ok(quest_log) = quest_log_query.get_single() {
        quest_log.active_quests.clone()
    } else {
        return;
    };

    // 处理每个进度事件
    for event in events.read() {
        // 更新所有进行中的任务
        for mut quest in quest_query.iter_mut() {
            // 只更新进行中的任务
            if quest.status != crate::components::quest::QuestStatus::InProgress {
                continue;
            }

            // 只更新在任务日志中的任务
            if !active_quest_ids.contains(&quest.id) {
                continue;
            }

            // 更新任务进度
            quest.update_progress(
                event.objective_type,
                event.target_id.as_deref(),
                event.amount,
            );

            // 记录进度更新
            if let Some(objective) = quest.objectives.iter().find(|obj| {
                obj.objective_type == event.objective_type
            }) {
                info!("任务进度更新: 任务={}, 目标={}, 进度={}/{}",
                    quest.title,
                    objective.description,
                    objective.current_count,
                    objective.target_count
                );
            }
        }
    }
}

/// 发送击杀敌人事件
pub fn send_kill_event(
    enemy_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Kill,
        target_id: Some(enemy_id),
        amount,
    });
}

/// 发送收集物品事件
pub fn send_collect_event(
    item_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Collect,
        target_id: Some(item_id),
        amount,
    });
}

/// 发送建造建筑事件
pub fn send_build_event(
    building_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Build,
        target_id: Some(building_id),
        amount,
    });
}

/// 发送防御位置事件
pub fn send_defend_event(
    location_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Defend,
        target_id: Some(location_id),
        amount,
    });
}

/// 发送探索区域事件
pub fn send_explore_event(
    area_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Explore,
        target_id: Some(area_id),
        amount,
    });
}

/// 发送生存时间事件
pub fn send_survive_event(
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Survive,
        target_id: None,
        amount,
    });
}

/// 发送制作物品事件
pub fn send_craft_event(
    item_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Craft,
        target_id: Some(item_id),
        amount,
    });
}

/// 发送收获植物事件
pub fn send_harvest_event(
    plant_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Harvest,
        target_id: Some(plant_id),
        amount,
    });
}

/// 发送升级建筑事件
pub fn send_upgrade_event(
    building_id: String,
    amount: u32,
    events: &mut Event<QuestProgressEvent>,
) {
    events.send(QuestProgressEvent {
        objective_type: QuestObjectiveType::Upgrade,
        target_id: Some(building_id),
        amount,
    });
}
