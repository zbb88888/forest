use bevy::prelude::*;
use crate::components::achievement::{Achievement, AchievementType, AchievementCondition, AchievementReward};

/// 成就生成系统插件
pub struct AchievementGeneratorPlugin;

impl Plugin for AchievementGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_achievements);
    }
}

/// 初始化所有成就
fn initialize_achievements(mut commands: Commands) {
    // === 战斗成就 ===

    // 初次杀敌
    let combat1 = Achievement::new(
        "combat_first_kill".to_string(),
        AchievementType::Combat,
        "初次杀敌".to_string(),
        "消灭你的第一个敌人".to_string(),
        "icon_combat_1".to_string(),
        AchievementCondition::KillEnemy("enemy".to_string(), 1),
        AchievementReward::new("新手战士".to_string()).with_experience(50),
    ).with_points(10);

    commands.spawn(combat1);

    // 敌人杀手
    let combat2 = Achievement::new(
        "combat_enemy_slayer".to_string(),
        AchievementType::Combat,
        "敌人杀手".to_string(),
        "消灭100个敌人".to_string(),
        "icon_combat_2".to_string(),
        AchievementCondition::KillEnemy("enemy".to_string(), 100),
        AchievementReward::new("敌人杀手".to_string()).with_experience(200).with_gold(100),
    ).with_points(20);

    commands.spawn(combat2);

    // 伤害输出
    let combat3 = Achievement::new(
        "combat_damage_dealer".to_string(),
        AchievementType::Combat,
        "伤害输出".to_string(),
        "累计造成10000点伤害".to_string(),
        "icon_combat_3".to_string(),
        AchievementCondition::DealDamage(10000.0),
        AchievementReward::new("伤害输出者".to_string()).with_experience(150).with_gold(75),
    ).with_points(15);

    commands.spawn(combat3);

    // 幸存者
    let combat4 = Achievement::new(
        "combat_survivor".to_string(),
        AchievementType::Combat,
        "幸存者".to_string(),
        "在游戏中生存1小时".to_string(),
        "icon_combat_4".to_string(),
        AchievementCondition::SurviveTime(3600.0),
        AchievementReward::new("幸存者".to_string()).with_experience(300).with_gold(150),
    ).with_points(30);

    commands.spawn(combat4);

    // === 探索成就 ===

    // 初次探索
    let explore1 = Achievement::new(
        "explore_first_area".to_string(),
        AchievementType::Exploration,
        "初次探索".to_string(),
        "探索第一个区域".to_string(),
        "icon_explore_1".to_string(),
        AchievementCondition::ExploreArea("area_1".to_string()),
        AchievementReward::new("探险家".to_string()).with_experience(50),
    ).with_points(10);

    commands.spawn(explore1);

    // 世界探索者
    let explore2 = Achievement::new(
        "explore_world".to_string(),
        AchievementType::Exploration,
        "世界探索者".to_string(),
        "探索所有区域".to_string(),
        "icon_explore_2".to_string(),
        AchievementCondition::DiscoverAllAreas,
        AchievementReward::new("世界探索者".to_string()).with_experience(500).with_gold(250),
    ).with_points(50);

    commands.spawn(explore2);

    // === 建造成就 ===

    // 建筑师
    let build1 = Achievement::new(
        "build_first_building".to_string(),
        AchievementType::Building,
        "建筑师".to_string(),
        "建造你的第一个建筑".to_string(),
        "icon_build_1".to_string(),
        AchievementCondition::BuildBuilding("building".to_string(), 1),
        AchievementReward::new("建筑师".to_string()).with_experience(50),
    ).with_points(10);

    commands.spawn(build1);

    // 建造大师
    let build2 = Achievement::new(
        "build_master".to_string(),
        AchievementType::Building,
        "建造大师".to_string(),
        "建造50个建筑".to_string(),
        "icon_build_2".to_string(),
        AchievementCondition::BuildBuilding("building".to_string(), 50),
        AchievementReward::new("建造大师".to_string()).with_experience(300).with_gold(150),
    ).with_points(30);

    commands.spawn(build2);

    // 升级专家
    let build3 = Achievement::new(
        "build_upgrader".to_string(),
        AchievementType::Building,
        "升级专家".to_string(),
        "升级10个建筑".to_string(),
        "icon_build_3".to_string(),
        AchievementCondition::UpgradeBuilding("building".to_string(), 10),
        AchievementReward::new("升级专家".to_string()).with_experience(200).with_gold(100),
    ).with_points(20);

    commands.spawn(build3);

    // === 资源成就 ===

    // 资源收集者
    let resource1 = Achievement::new(
        "resource_collector".to_string(),
        AchievementType::Resource,
        "资源收集者".to_string(),
        "收集1000个资源".to_string(),
        "icon_resource_1".to_string(),
        AchievementCondition::CollectResource("resource".to_string(), 1000),
        AchievementReward::new("资源收集者".to_string()).with_experience(100).with_gold(50),
    ).with_points(15);

    commands.spawn(resource1);

    // 资源大亨
    let resource2 = Achievement::new(
        "resource_tycoon".to_string(),
        AchievementType::Resource,
        "资源大亨".to_string(),
        "拥有10000个资源".to_string(),
        "icon_resource_2".to_string(),
        AchievementCondition::ReachResourceAmount("resource".to_string(), 10000),
        AchievementReward::new("资源大亨".to_string()).with_experience(400).with_gold(200),
    ).with_points(40);

    commands.spawn(resource2);

    // === 任务成就 ===

    // 任务完成者
    let quest1 = Achievement::new(
        "quest_completer".to_string(),
        AchievementType::Quest,
        "任务完成者".to_string(),
        "完成10个任务".to_string(),
        "icon_quest_1".to_string(),
        AchievementCondition::CompleteQuests(10),
        AchievementReward::new("任务完成者".to_string()).with_experience(150).with_gold(75),
    ).with_points(20);

    commands.spawn(quest1);

    // 日常达人
    let quest2 = Achievement::new(
        "quest_daily_master".to_string(),
        AchievementType::Quest,
        "日常达人".to_string(),
        "完成30个日常任务".to_string(),
        "icon_quest_2".to_string(),
        AchievementCondition::CompleteDailyQuests(30),
        AchievementReward::new("日常达人".to_string()).with_experience(300).with_gold(150),
    ).with_points(30);

    commands.spawn(quest2);

    // === 社交成就 ===

    // 成长之路
    let social1 = Achievement::new(
        "social_level_10".to_string(),
        AchievementType::Social,
        "成长之路".to_string(),
        "达到10级".to_string(),
        "icon_social_1".to_string(),
        AchievementCondition::ReachLevel(10),
        AchievementReward::new("成长中".to_string()).with_experience(200).with_gold(100),
    ).with_points(20);

    commands.spawn(social1);

    // 成熟战士
    let social2 = Achievement::new(
        "social_level_20".to_string(),
        AchievementType::Social,
        "成熟战士".to_string(),
        "达到20级".to_string(),
        "icon_social_2".to_string(),
        AchievementCondition::ReachLevel(20),
        AchievementReward::new("成熟战士".to_string()).with_experience(400).with_gold(200),
    ).with_points(40);

    commands.spawn(social2);

    // === 里程碑成就 ===

    // 初次游戏
    let milestone1 = Achievement::new(
        "milestone_first_play".to_string(),
        AchievementType::Milestone,
        "初次游戏".to_string(),
        "游戏时间达到1小时".to_string(),
        "icon_milestone_1".to_string(),
        AchievementCondition::PlayTime(3600.0),
        AchievementReward::new("新手".to_string()).with_experience(100).with_gold(50),
    ).with_points(10);

    commands.spawn(milestone1);

    // 游戏达人
    let milestone2 = Achievement::new(
        "milestone_play_master".to_string(),
        AchievementType::Milestone,
        "游戏达人".to_string(),
        "游戏时间达到10小时".to_string(),
        "icon_milestone_2".to_string(),
        AchievementCondition::PlayTime(36000.0),
        AchievementReward::new("游戏达人".to_string()).with_experience(500).with_gold(250),
    ).with_points(50);

    commands.spawn(milestone2);

    // 游戏大师
    let milestone3 = Achievement::new(
        "milestone_play_expert".to_string(),
        AchievementType::Milestone,
        "游戏大师".to_string(),
        "游戏时间达到100小时".to_string(),
        "icon_milestone_3".to_string(),
        AchievementCondition::PlayTime(360000.0),
        AchievementReward::new("游戏大师".to_string()).with_experience(1000).with_gold(500),
    ).with_points(100);

    commands.spawn(milestone3);

    // === 特殊成就 ===

    // 幸运儿
    let special1 = Achievement::new(
        "special_lucky".to_string(),
        AchievementType::Special,
        "幸运儿".to_string(),
        "获得稀有物品".to_string(),
        "icon_special_1".to_string(),
        AchievementCondition::Custom("rare_item".to_string(), 1),
        AchievementReward::new("幸运儿".to_string()).with_experience(200).with_gold(100),
    ).with_points(25).with_hidden(true);

    commands.spawn(special1);

    info!("成就系统初始化完成，共加载20个成就");
}
