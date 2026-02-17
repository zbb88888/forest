use bevy::prelude::*;
use crate::components::combat::{
    Combat, DamageEvent, HealEvent, DeathEvent, CombatEffect, CombatEffectType,
    CombatStats, DamageType
};
use crate::components::player::Player;
use crate::components::enemy::Enemy;

/// 战斗系统插件
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_event::<DeathEvent>()
            .add_systems(Update, (
                update_combat_cooldowns,
                process_damage_events,
                process_heal_events,
                process_death_events,
                update_combat_effects,
                update_combat_stats,
            ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 更新战斗冷却
fn update_combat_cooldowns(
    time: Res<Time>,
    mut combat_query: Query<&mut Combat>,
) {
    for mut combat in combat_query.iter_mut() {
        if combat.attack_cooldown > 0.0 {
            combat.attack_cooldown -= time.delta_seconds();
        }
    }
}

/// 处理伤害事件
fn process_damage_events(
    mut commands: Commands,
    mut damage_events: Event<DamageEvent>,
    mut combat_query: Query<&mut Combat>,
    mut enemy_query: Query<&mut Enemy>,
    mut player_query: Query<&mut Player>,
) {
    for event in damage_events.iter() {
        // 计算实际伤害
        let actual_damage = event.damage;

        // 处理目标伤害
        if let Ok(mut combat) = combat_query.get_mut(event.target) {
            // 更新战斗统计
            if let Ok(mut stats) = combat_query.get_component::<CombatStats>(event.target) {
                stats.damage_taken += actual_damage;
            }

            // 应用伤害到敌人
            if let Ok(mut enemy) = enemy_query.get_mut(event.target) {
                enemy.take_damage(actual_damage);

                // 检查是否死亡
                if enemy.is_dead() {
                    commands.trigger_targets(DeathEvent { entity: event.target }, event.target);
                }
            }

            // 应用伤害到玩家
            if let Ok(player) = player_query.get_mut(event.target) {
                // 玩家伤害处理
                // 实际实现需要玩家组件支持
            }
        }

        info!("伤害事件: {:?} -> {:?}, 伤害: {}, 类型: {:?}, 暴击: {}", 
            event.source, event.target, event.damage, event.damage_type, event.is_critical);
    }
}

/// 处理治疗事件
fn process_heal_events(
    mut heal_events: Event<HealEvent>,
    mut enemy_query: Query<&mut Enemy>,
) {
    for event in heal_events.iter() {
        if let Ok(mut enemy) = enemy_query.get_mut(event.target) {
            enemy.heal(event.amount);
            info!("治疗事件: {:?}, 恢复: {}", event.target, event.amount);
        }
    }
}

/// 处理死亡事件
fn process_death_events(
    mut death_events: Event<DeathEvent>,
    mut commands: Commands,
    mut combat_query: Query<&mut CombatStats>,
) {
    for event in death_events.iter() {
        // 更新攻击者的战斗统计
        if let Ok(mut stats) = combat_query.get_mut(event.entity) {
            stats.enemies_defeated += 1;
        }

        // 移除死亡实体
        commands.entity(event.entity).despawn_recursive();

        info!("死亡事件: {:?}", event.entity);
    }
}

/// 更新战斗效果
fn update_combat_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut CombatEffect, &mut Combat)>,
) {
    for (entity, mut effect, mut combat) in effect_query.iter_mut() {
        effect.timer += time.delta_seconds();

        // 应用效果
        match effect.effect_type {
            CombatEffectType::Burn => {
                // 燃烧效果：持续伤害
                if effect.timer % 1.0 < time.delta_seconds() {
                    // 每秒造成伤害
                    commands.trigger_targets(
                        DamageEvent {
                            source: entity,
                            target: entity,
                            damage: effect.value,
                            damage_type: DamageType::Explosive,
                            is_critical: false,
                        },
                        entity,
                    );
                }
            }
            CombatEffectType::Poison => {
                // 中毒效果：持续伤害
                if effect.timer % 1.0 < time.delta_seconds() {
                    commands.trigger_targets(
                        DamageEvent {
                            source: entity,
                            target: entity,
                            damage: effect.value,
                            damage_type: DamageType::Corrosive,
                            is_critical: false,
                        },
                        entity,
                    );
                }
            }
            CombatEffectType::Slow => {
                // 减速效果：降低攻击速度
                combat.attack.attack_speed *= 0.5;
            }
            CombatEffectType::Freeze => {
                // 冰冻效果：无法攻击
                combat.attack_cooldown = effect.duration - effect.timer;
            }
            CombatEffectType::Stun => {
                // 眩晕效果：无法行动
                combat.attack_cooldown = effect.duration - effect.timer;
            }
            CombatEffectType::Shield => {
                // 护盾效果：增加防御
                combat.defense.physical_resistance += effect.value;
                combat.defense.energy_resistance += effect.value;
            }
        }

        // 移除完成的效果
        if effect.is_finished() {
            commands.entity(entity).remove::<CombatEffect>();
        }
    }
}

/// 更新战斗统计
fn update_combat_stats(
    mut combat_query: Query<&mut CombatStats>,
) {
    for mut stats in combat_query.iter_mut() {
        // 定期保存战斗统计
        // 实际实现需要持久化系统
    }
}

/// 执行攻击
pub fn perform_attack(
    commands: &mut Commands,
    source: Entity,
    target: Entity,
    combat: &Combat,
) {
    // 检查是否可以攻击
    if !combat.can_attack() {
        return;
    }

    // 计算伤害
    let damage = combat.attack.damage;
    let is_critical = combat.is_critical();
    let final_damage = if is_critical {
        combat.get_critical_damage()
    } else {
        damage
    };

    // 发送伤害事件
    commands.trigger_targets(
        DamageEvent {
            source,
            target,
            damage: final_damage,
            damage_type: combat.attack.damage_type,
            is_critical,
        },
        target,
    );

    // 更新攻击冷却
    // 实际实现需要修改combat组件
}

/// 执行治疗
pub fn perform_heal(
    commands: &mut Commands,
    target: Entity,
    amount: f32,
) {
    commands.trigger_targets(
        HealEvent {
            target,
            amount,
        },
        target,
    );
}

/// 添加战斗效果
pub fn add_combat_effect(
    commands: &mut Commands,
    target: Entity,
    effect_type: CombatEffectType,
    duration: f32,
    value: f32,
) {
    commands.entity(target).insert(CombatEffect::new(effect_type, duration, value));
}
