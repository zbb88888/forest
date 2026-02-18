use bevy::prelude::*;
use crate::components::quest::{Quest, QuestObjectiveType, QuestLog};

pub struct QuestEventsPlugin;

impl Plugin for QuestEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<QuestProgressEvent>()
            .add_observer(handle_quest_progress);
    }
}

#[derive(Event, Message, Debug, Clone)]
pub struct QuestProgressEvent {
    pub objective_type: QuestObjectiveType,
    pub target_id: Option<String>,
    pub amount: u32,
}

fn handle_quest_progress(
    event: On<QuestProgressEvent>,
    mut quest_query: Query<&mut Quest>,
    quest_log_query: Query<&QuestLog>,
) {
    let quest_event = event.event();

    let active_quest_ids = if let Ok(quest_log) = quest_log_query.single() {
        quest_log.active_quests.clone()
    } else {
        return;
    };

    for mut quest in quest_query.iter_mut() {
        if quest.status != crate::components::quest::QuestStatus::InProgress {
            continue;
        }

        if !active_quest_ids.contains(&quest.id) {
            continue;
        }

        quest.update_progress(
            quest_event.objective_type,
            quest_event.target_id.as_deref(),
            quest_event.amount,
        );

        if let Some(objective) = quest.objectives.iter().find(|obj| {
            obj.objective_type == quest_event.objective_type
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

pub fn send_kill_event(
    mut commands: Commands,
    enemy_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Kill,
        target_id: Some(enemy_id),
        amount,
    });
}

pub fn send_collect_event(
    mut commands: Commands,
    item_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Collect,
        target_id: Some(item_id),
        amount,
    });
}

pub fn send_build_event(
    mut commands: Commands,
    building_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Build,
        target_id: Some(building_id),
        amount,
    });
}

pub fn send_defend_event(
    mut commands: Commands,
    location_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Defend,
        target_id: Some(location_id),
        amount,
    });
}

pub fn send_explore_event(
    mut commands: Commands,
    area_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Explore,
        target_id: Some(area_id),
        amount,
    });
}

pub fn send_survive_event(
    mut commands: Commands,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Survive,
        target_id: None,
        amount,
    });
}

pub fn send_craft_event(
    mut commands: Commands,
    item_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Craft,
        target_id: Some(item_id),
        amount,
    });
}

pub fn send_harvest_event(
    mut commands: Commands,
    plant_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Harvest,
        target_id: Some(plant_id),
        amount,
    });
}

pub fn send_upgrade_event(
    mut commands: Commands,
    building_id: String,
    amount: u32,
) {
    commands.trigger(QuestProgressEvent {
        objective_type: QuestObjectiveType::Upgrade,
        target_id: Some(building_id),
        amount,
    });
}