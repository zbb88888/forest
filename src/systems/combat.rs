use bevy::prelude::*;
use crate::components::combat::{
    Combat, DamageEvent, HealEvent, DeathEvent, CombatEffect, CombatEffectType,
    CombatStats, DamageType
};
use crate::components::player::Player;
use crate::components::enemy::Enemy;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_message::<HealEvent>()
            .add_message::<DeathEvent>()
            .add_systems(Update, (
                update_combat_cooldowns,
                update_combat_effects,
                update_combat_stats,
            ))
            .add_observer(handle_damage_event)
            .add_observer(handle_heal_event)
            .add_observer(handle_death_event);
    }
}

fn update_combat_cooldowns(
    time: Res<Time>,
    mut combat_query: Query<&mut Combat>,
) {
    for mut combat in combat_query.iter_mut() {
        if combat.attack_cooldown > 0.0 {
            combat.attack_cooldown -= time.delta_secs();
        }
    }
}

fn handle_damage_event(
    event: On<DamageEvent>,
    mut commands: Commands,
    mut combat_query: Query<&mut Combat>,
    mut enemy_query: Query<&mut Enemy>,
    mut player_query: Query<&mut Player>,
) {
    let damage_event = event.event();
    let actual_damage = damage_event.damage;

    if let Ok(mut combat) = combat_query.get_mut(damage_event.target) {
        combat.damage_taken += actual_damage;

        if let Ok(mut enemy) = enemy_query.get_mut(damage_event.target) {
            enemy.take_damage(actual_damage);

            if enemy.is_dead() {
                commands.trigger(DeathEvent { entity: damage_event.target });
            }
        }

        if let Ok(_player) = player_query.get_mut(damage_event.target) {
        }
    }

    info!("伤害事件: {:?} -> {:?}, 伤害: {}, 类型: {:?}, 暴击: {}",
        damage_event.source, damage_event.target, damage_event.damage, damage_event.damage_type, damage_event.is_critical);
}

fn handle_heal_event(
    event: On<HealEvent>,
    mut enemy_query: Query<&mut Enemy>,
) {
    let heal_event = event.event();
    if let Ok(mut enemy) = enemy_query.get_mut(heal_event.target) {
        enemy.heal(heal_event.amount);
        info!("治疗事件: {:?}, 恢复: {}", heal_event.target, heal_event.amount);
    }
}

fn handle_death_event(
    event: On<DeathEvent>,
    mut commands: Commands,
    mut combat_query: Query<&mut CombatStats>,
) {
    let death_event = event.event();

    if let Ok(mut stats) = combat_query.get_mut(death_event.entity) {
        stats.enemies_defeated += 1;
    }

    commands.entity(death_event.entity).despawn();
    info!("死亡事件: {:?}", death_event.entity);
}

fn update_combat_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut CombatEffect, &mut Combat)>,
) {
    for (entity, mut effect, mut combat) in effect_query.iter_mut() {
        effect.timer += time.delta_secs();

        match effect.effect_type {
            CombatEffectType::Burn => {
                if effect.timer % 1.0 < time.delta_secs() {
                    commands.trigger(DamageEvent {
                        source: entity,
                        target: entity,
                        damage: effect.value,
                        damage_type: DamageType::Explosive,
                        is_critical: false,
                    });
                }
            }
            CombatEffectType::Poison => {
                if effect.timer % 1.0 < time.delta_secs() {
                    commands.trigger(DamageEvent {
                        source: entity,
                        target: entity,
                        damage: effect.value,
                        damage_type: DamageType::Corrosive,
                        is_critical: false,
                    });
                }
            }
            CombatEffectType::Slow => {
                combat.attack.attack_speed *= 0.5;
            }
            CombatEffectType::Freeze => {
                combat.attack_cooldown = effect.duration - effect.timer;
            }
            CombatEffectType::Stun => {
                combat.attack_cooldown = effect.duration - effect.timer;
            }
            CombatEffectType::Shield => {
            }
        }

        if effect.is_finished() {
            commands.entity(entity).remove::<CombatEffect>();
        }
    }
}

fn update_combat_stats(
    mut combat_query: Query<&mut CombatStats>,
) {
    for _stats in combat_query.iter_mut() {
        // CombatStats.damage_dealt is already a single f32 value
    }
}