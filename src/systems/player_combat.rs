use bevy::prelude::*;
use crate::components::combat::{
    Combat, DamageEvent, CombatEffectType
};

use crate::components::enemy::Enemy;

/// 玩家战斗系统插件
pub struct PlayerCombatPlugin;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_player_attack.run_if(in_state(crate::states::GameState::InGame)),
            update_player_combat.run_if(in_state(crate::states::GameState::InGame)),
        ));
    }
}

/// 玩家战斗组件
#[derive(Component, Clone, Debug)]
pub struct PlayerCombat {
    pub attack_type: PlayerAttackType,
    pub combo_count: u32,
    pub combo_timer: f32,
    pub special_attack_cooldown: f32,
    pub ultimate_cooldown: f32,
}

/// 玩家攻击类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAttackType {
    Melee,      // 近战攻击
    Ranged,     // 远程攻击
    Magic,      // 魔法攻击
}

impl Default for PlayerCombat {
    fn default() -> Self {
        Self {
            attack_type: PlayerAttackType::Melee,
            combo_count: 0,
            combo_timer: 0.0,
            special_attack_cooldown: 0.0,
            ultimate_cooldown: 0.0,
        }
    }
}

/// 处理玩家攻击
fn handle_player_attack(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(Entity, &mut Combat, &mut PlayerCombat, &Transform)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let Ok((player_entity, mut combat, mut player_combat, player_transform)) = player_query.single_mut() else { return; };
    // 普通攻击
    if mouse_input.just_pressed(MouseButton::Left) {
        perform_player_attack(
            &mut commands,
            player_entity,
            &mut combat,
            &mut player_combat,
            player_transform,
            &enemy_query,
        );
    }

    // 特殊攻击
    if keyboard_input.just_pressed(KeyCode::Space) {
        perform_special_attack(
            &mut commands,
            player_entity,
            &mut combat,
            &mut player_combat,
            player_transform,
            &enemy_query,
        );
    }

    // 终极技能
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        perform_ultimate_attack(
            &mut commands,
            player_entity,
            &mut combat,
            &mut player_combat,
            player_transform,
            &enemy_query,
        );
    }
}

/// 执行玩家普通攻击
fn perform_player_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_combat: &mut PlayerCombat,
    player_transform: &Transform,
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
) {
    if !combat.can_attack() {
        return;
    }

    // 查找最近的敌人
    let mut nearest_enemy = None;
    let mut nearest_distance = f32::MAX;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let distance = player_transform.translation.distance(enemy_transform.translation);
        if distance < nearest_distance && distance <= combat.attack.attack_range {
            nearest_distance = distance;
            nearest_enemy = Some(enemy_entity);
        }
    }

    // 如果找到敌人，执行攻击
    if let Some(target) = nearest_enemy {
        // 计算连击
        player_combat.combo_count += 1;
        player_combat.combo_timer = 2.0; // 2秒内可以连击

        // 连击加成
        let combo_multiplier = 1.0 + (player_combat.combo_count as f32 * 0.1);

        // 计算伤害
        let base_damage = combat.attack.damage;
        let is_critical = combat.is_critical();
        let final_damage = if is_critical {
            combat.get_critical_damage() * combo_multiplier
        } else {
            base_damage * combo_multiplier
        };

        // 发送伤害事件
        commands.trigger(
            DamageEvent {
                source: player_entity,
                target,
                damage: final_damage,
                damage_type: combat.attack.damage_type,
                is_critical,
            },
        );

        // 更新攻击冷却
        combat.attack_cooldown = 1.0 / combat.attack.attack_speed;

        info!("玩家攻击: 目标={:?}, 伤害={}, 连击={}, 暴击={}", 
            target, final_damage, player_combat.combo_count, is_critical);
    }
}

/// 执行玩家特殊攻击
fn perform_special_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_combat: &mut PlayerCombat,
    player_transform: &Transform,
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
) {
    if player_combat.special_attack_cooldown > 0.0 {
        return;
    }

    // 特殊攻击：范围攻击
    let attack_range = combat.attack.attack_range * 1.5;
    let special_damage = combat.attack.damage * 2.0;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let distance = player_transform.translation.distance(enemy_transform.translation);
        if distance <= attack_range {
            commands.trigger(
                DamageEvent {
                    source: player_entity,
                    target: enemy_entity,
                    damage: special_damage,
                    damage_type: combat.attack.damage_type,
                    is_critical: false,
                },
            );

            // 添加减速效果
            commands.entity(enemy_entity).insert(
                crate::components::combat::CombatEffect::new(
                    CombatEffectType::Slow,
                    3.0,
                    0.5,
                )
            );
        }
    }

    // 设置冷却
    player_combat.special_attack_cooldown = 10.0;

    info!("玩家特殊攻击: 范围={}, 伤害={}", attack_range, special_damage);
}

/// 执行玩家终极技能
fn perform_ultimate_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_combat: &mut PlayerCombat,
    player_transform: &Transform,
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
) {
    if player_combat.ultimate_cooldown > 0.0 {
        return;
    }

    // 终极技能：大范围高伤害攻击
    let attack_range = combat.attack.attack_range * 2.0;
    let ultimate_damage = combat.attack.damage * 3.0;

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let distance = player_transform.translation.distance(enemy_transform.translation);
        if distance <= attack_range {
            commands.trigger(
                DamageEvent {
                    source: player_entity,
                    target: enemy_entity,
                    damage: ultimate_damage,
                    damage_type: combat.attack.damage_type,
                    is_critical: true,
                },
            );

            // 添加眩晕效果
            commands.entity(enemy_entity).insert(
                crate::components::combat::CombatEffect::new(
                    CombatEffectType::Stun,
                    2.0,
                    0.0,
                )
            );
        }
    }

    // 设置冷却
    player_combat.ultimate_cooldown = 30.0;

    info!("玩家终极技能: 范围={}, 伤害={}", attack_range, ultimate_damage);
}

/// 更新玩家战斗状态
fn update_player_combat(
    time: Res<Time>,
    mut player_query: Query<&mut PlayerCombat>,
) {
    let Ok(mut player_combat) = player_query.single_mut() else { return; };
    // 更新连击计时器
    if player_combat.combo_timer > 0.0 {
        player_combat.combo_timer -= time.delta_secs();
        if player_combat.combo_timer <= 0.0 {
            player_combat.combo_count = 0;
        }
    }

    // 更新特殊攻击冷却
    if player_combat.special_attack_cooldown > 0.0 {
        player_combat.special_attack_cooldown -= time.delta_secs();
    }

    // 更新终极技能冷却
    if player_combat.ultimate_cooldown > 0.0 {
        player_combat.ultimate_cooldown -= time.delta_secs();
    }
}
