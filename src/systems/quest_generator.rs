use bevy::prelude::*;
use crate::components::quest::{Quest, QuestType, QuestObjectiveType, QuestReward};

/// 任务生成系统插件
pub struct QuestGeneratorPlugin;

impl Plugin for QuestGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_main_quests)
            .add_systems(Update, (
                generate_daily_quests,
                generate_event_quests,
            ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 初始化主线任务
fn initialize_main_quests(mut commands: Commands) {
    // 主线任务1：建立基地
    let quest1 = Quest::new(
        "main_1".to_string(),
        QuestType::Main,
        "建立基地".to_string(),
        "建造你的第一个基地，开始你的生存之旅".to_string(),
        QuestReward::new(100, 50).with_resource("wood".to_string(), 100),
    )
    .with_objective(
        crate::components::quest::QuestObjective::new(
            QuestObjectiveType::Build,
            Some("base".to_string()),
            1,
            "建造1个基地".to_string(),
        )
    )
    .with_level_requirement(1);

    commands.spawn(quest1);

    // 主线任务2：采集资源
    let quest2 = Quest::new(
        "main_2".to_string(),
        QuestType::Main,
        "采集资源".to_string(),
        "采集足够的资源来支持你的发展".to_string(),
        QuestReward::new(150, 75).with_resource("stone".to_string(), 50),
    )
    .with_prerequisite("main_1".to_string())
    .with_objective(
        crate::components::quest::QuestObjective::new(
            QuestObjectiveType::Collect,
            Some("wood".to_string()),
            50,
            "采集50个木材".to_string(),
        )
    )
    .with_objective(
        crate::components::quest::QuestObjective::new(
            QuestObjectiveType::Collect,
            Some("stone".to_string()),
            30,
            "采集30个石头".to_string(),
        )
    )
    .with_level_requirement(1);

    commands.spawn(quest2);

    // 主线任务3：建造防御
    let quest3 = Quest::new(
        "main_3".to_string(),
        QuestType::Main,
        "建造防御".to_string(),
        "建造防御塔来保护你的基地".to_string(),
        QuestReward::new(200, 100).with_resource("iron".to_string(), 30),
    )
    .with_prerequisite("main_2".to_string())
    .with_objective(
        crate::components::quest::QuestObjective::new(
            QuestObjectiveType::Build,
            Some("defense_tower".to_string()),
            2,
            "建造2个防御塔".to_string(),
        )
    )
    .with_level_requirement(2);

    commands.spawn(quest3);

    // 主线任务4：消灭敌人
    let quest4 = Quest::new(
        "main_4".to_string(),
        QuestType::Main,
        "消灭敌人".to_string(),
        "消灭入侵的敌人，保护你的基地".to_string(),
        QuestReward::new(300, 150).with_item("weapon_rare".to_string()),
    )
    .with_prerequisite("main_3".to_string())
    .with_objective(
        crate::components::quest::QuestObjective::new(
            QuestObjectiveType::Kill,
            Some("enemy_robot".to_string()),
            10,
            "消灭10个机器人敌人".to_string(),
        )
    )
    .with_level_requirement(3);

    commands.spawn(quest4);

    info!("主线任务初始化完成");
}

/// 生成日常任务
fn generate_daily_quests(
    mut commands: Commands,
    time: Res<Time>,
    mut last_generation: Local<Option<f32>>,
) {
    // 每24小时生成一次日常任务
    let current_time = time.elapsed_seconds();
    let generation_interval = 24.0 * 60.0 * 60.0; // 24小时

    let should_generate = match *last_generation {
        Some(last) => current_time - last >= generation_interval,
        None => true,
    };

    if should_generate {
        // 生成日常任务1：日常采集
        let daily1 = Quest::new(
            format!("daily_collect_{}", current_time as u32),
            QuestType::Daily,
            "日常采集".to_string(),
            "采集指定数量的资源".to_string(),
            QuestReward::new(50, 25),
        )
        .with_objective(
            crate::components::quest::QuestObjective::new(
                QuestObjectiveType::Collect,
                Some("wood".to_string()),
                20,
                "采集20个木材".to_string(),
            )
        )
        .with_time_limit(24.0 * 60.0 * 60.0);

        commands.spawn(daily1);

        // 生成日常任务2：日常战斗
        let daily2 = Quest::new(
            format!("daily_combat_{}", current_time as u32),
            QuestType::Daily,
            "日常战斗".to_string(),
            "消灭指定数量的敌人".to_string(),
            QuestReward::new(75, 35),
        )
        .with_objective(
            crate::components::quest::QuestObjective::new(
                QuestObjectiveType::Kill,
                Some("enemy".to_string()),
                5,
                "消灭5个敌人".to_string(),
            )
        )
        .with_time_limit(24.0 * 60.0 * 60.0);

        commands.spawn(daily2);

        *last_generation = Some(current_time);
        info!("日常任务生成完成");
    }
}

/// 生成活动任务
fn generate_event_quests(
    mut commands: Commands,
    time: Res<Time>,
    mut last_generation: Local<Option<f32>>,
) {
    // 每小时检查一次是否生成活动任务
    let current_time = time.elapsed_seconds();
    let check_interval = 60.0 * 60.0; // 1小时

    let should_check = match *last_generation {
        Some(last) => current_time - last >= check_interval,
        None => true,
    };

    if should_check {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // 30%的概率生成活动任务
        if rng.gen_bool(0.3) {
            // 随机选择活动类型
            let event_type = rng.gen_range(0..3);

            let event_quest = match event_type {
                0 => {
                    // 限时击杀活动
                    Quest::new(
                        format!("event_kill_{}", current_time as u32),
                        QuestType::Event,
                        "限时击杀".to_string(),
                        "在限定时间内消灭大量敌人".to_string(),
                        QuestReward::new(200, 100).with_item("weapon_epic".to_string()),
                    )
                    .with_objective(
                        crate::components::quest::QuestObjective::new(
                            QuestObjectiveType::Kill,
                            Some("enemy".to_string()),
                            20,
                            "消灭20个敌人".to_string(),
                        )
                    )
                    .with_time_limit(30.0 * 60.0) // 30分钟
                }
                1 => {
                    // 限时采集活动
                    Quest::new(
                        format!("event_collect_{}", current_time as u32),
                        QuestType::Event,
                        "限时采集".to_string(),
                        "在限定时间内采集大量资源".to_string(),
                        QuestReward::new(150, 75),
                    )
                    .with_objective(
                        crate::components::quest::QuestObjective::new(
                            QuestObjectiveType::Collect,
                            Some("rare_resource".to_string()),
                            10,
                            "采集10个稀有资源".to_string(),
                        )
                    )
                    .with_time_limit(20.0 * 60.0) // 20分钟
                }
                _ => {
                    // 限时建造活动
                    Quest::new(
                        format!("event_build_{}", current_time as u32),
                        QuestType::Event,
                        "限时建造".to_string(),
                        "在限定时间内建造指定建筑".to_string(),
                        QuestReward::new(180, 90),
                    )
                    .with_objective(
                        crate::components::quest::QuestObjective::new(
                            QuestObjectiveType::Build,
                            Some("special_building".to_string()),
                            3,
                            "建造3个特殊建筑".to_string(),
                        )
                    )
                    .with_time_limit(25.0 * 60.0) // 25分钟
                }
            };

            commands.spawn(event_quest);
            info!("活动任务生成完成");
        }

        *last_generation = Some(current_time);
    }
}

/// 生成随机支线任务
pub fn generate_random_side_quest(
    commands: &mut Commands,
    player_level: u32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 根据玩家等级选择任务类型
    let quest_type = match player_level {
        1..=3 => 0,
        4..=6 => 1,
        _ => 2,
    };

    let side_quest = match quest_type {
        0 => {
            // 低等级：采集任务
            Quest::new(
                format!("side_collect_{}", rng.gen::<u32>()),
                QuestType::Side,
                "收集资源".to_string(),
                "收集指定数量的资源".to_string(),
                QuestReward::new(30, 15),
            )
            .with_objective(
                crate::components::quest::QuestObjective::new(
                    QuestObjectiveType::Collect,
                    Some("resource".to_string()),
                    rng.gen_range(10..20),
                    format!("收集{}个资源", rng.gen_range(10..20)),
                )
            )
        }
        1 => {
            // 中等级：战斗任务
            Quest::new(
                format!("side_combat_{}", rng.gen::<u32>()),
                QuestType::Side,
                "消灭敌人".to_string(),
                "消灭指定数量的敌人".to_string(),
                QuestReward::new(50, 25),
            )
            .with_objective(
                crate::components::quest::QuestObjective::new(
                    QuestObjectiveType::Kill,
                    Some("enemy".to_string()),
                    rng.gen_range(5..10),
                    format!("消灭{}个敌人", rng.gen_range(5..10)),
                )
            )
        }
        _ => {
            // 高等级：综合任务
            Quest::new(
                format!("side_mixed_{}", rng.gen::<u32>()),
                QuestType::Side,
                "综合任务".to_string(),
                "完成多个目标".to_string(),
                QuestReward::new(80, 40),
            )
            .with_objective(
                crate::components::quest::QuestObjective::new(
                    QuestObjectiveType::Kill,
                    Some("enemy".to_string()),
                    rng.gen_range(10..15),
                    format!("消灭{}个敌人", rng.gen_range(10..15)),
                )
            )
            .with_objective(
                crate::components::quest::QuestObjective::new(
                    QuestObjectiveType::Collect,
                    Some("resource".to_string()),
                    rng.gen_range(15..25),
                    format!("收集{}个资源", rng.gen_range(15..25)),
                )
            )
        }
    };

    commands.spawn(side_quest);
    info!("随机支线任务生成完成");
}
