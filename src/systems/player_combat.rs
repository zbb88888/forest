use bevy::prelude::*;
use crate::components::combat::{Combat, DamageEvent, CombatEffect, CombatEffectType};
use crate::components::enemy::Enemy;
use crate::components::player::Player;

pub struct PlayerCombatPlugin;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyPositions>()
            .add_systems(Update, (
                collect_enemy_positions,
                handle_player_attack.run_if(in_state(crate::states::GameState::InGame)),
                update_player_combat.run_if(in_state(crate::states::GameState::InGame)),
            ));
    }
}

#[derive(Component, Clone, Debug)]
pub struct PlayerCombat {
    pub attack_type: PlayerAttackType,
    pub combo_count: u32,
    pub combo_timer: f32,
    pub special_attack_cooldown: f32,
    pub ultimate_cooldown: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAttackType {
    Melee,
    Ranged,
    Magic,
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

#[derive(Resource, Default)]
pub struct EnemyPositions {
    pub positions: Vec<(Entity, Vec3)>,
}

pub fn collect_enemy_positions(
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    mut enemy_positions: ResMut<EnemyPositions>,
) {
    enemy_positions.positions = enemy_query.iter()
        .map(|(e, t)| (e, t.translation))
        .collect();
}

fn handle_player_attack(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(Entity, &mut Combat, &mut PlayerCombat, &Transform), Without<Enemy>>,
    enemy_positions: Res<EnemyPositions>,
) {
    let Ok((player_entity, mut combat, mut player_combat, player_transform)) = player_query.single_mut() else { return; };

    let player_pos = player_transform.translation;
    let enemy_data = &enemy_positions.positions;

    if mouse_input.just_pressed(MouseButton::Left) {
        perform_player_attack(
            &mut commands,
            player_entity,
            &mut combat,
            player_pos,
            enemy_data,
        );
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        perform_special_attack(
            &mut commands,
            player_entity,
            &mut combat,
            &mut player_combat,
            player_pos,
            enemy_data,
        );
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        perform_ultimate_attack(
            &mut commands,
            player_entity,
            &mut combat,
            &mut player_combat,
            player_pos,
            enemy_data,
        );
    }
}

fn perform_player_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_pos: Vec3,
    enemy_data: &Vec<(Entity, Vec3)>,
) {
    if !combat.can_attack() {
        return;
    }

    let mut nearest_enemy = None;
    let mut nearest_distance = f32::MAX;

    for (enemy_entity, enemy_pos) in enemy_data.iter() {
        let distance = player_pos.distance(*enemy_pos);
        if distance < nearest_distance && distance <= combat.attack.attack_range {
            nearest_distance = distance;
            nearest_enemy = Some(*enemy_entity);
        }
    }

    if let Some(target) = nearest_enemy {
        combat.attack_cooldown = combat.attack.attack_speed;

        commands.trigger(
            DamageEvent {
                source: player_entity,
                target,
                damage: combat.attack.damage,
                damage_type: combat.attack.damage_type,
                is_critical: false,
            },
        );
    }
}

fn perform_special_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_combat: &mut PlayerCombat,
    player_pos: Vec3,
    enemy_data: &Vec<(Entity, Vec3)>,
) {
    if player_combat.special_attack_cooldown > 0.0 {
        return;
    }

    let attack_range = combat.attack.attack_range * 1.5;
    let special_damage = combat.attack.damage * 2.0;

    for (enemy_entity, enemy_pos) in enemy_data.iter() {
        let distance = player_pos.distance(*enemy_pos);
        if distance <= attack_range {
            commands.trigger(
                DamageEvent {
                    source: player_entity,
                    target: *enemy_entity,
                    damage: special_damage,
                    damage_type: combat.attack.damage_type,
                    is_critical: false,
                },
            );

            commands.entity(*enemy_entity).insert(
                CombatEffect::new(CombatEffectType::Slow, 2.0, 0.5)
            );
        }
    }

    player_combat.special_attack_cooldown = 10.0;
}

fn perform_ultimate_attack(
    commands: &mut Commands,
    player_entity: Entity,
    combat: &mut Combat,
    player_combat: &mut PlayerCombat,
    player_pos: Vec3,
    enemy_data: &Vec<(Entity, Vec3)>,
) {
    if player_combat.ultimate_cooldown > 0.0 {
        return;
    }

    let attack_range = combat.attack.attack_range * 2.0;
    let ultimate_damage = combat.attack.damage * 3.0;

    for (enemy_entity, enemy_pos) in enemy_data.iter() {
        let distance = player_pos.distance(*enemy_pos);
        if distance <= attack_range {
            commands.trigger(
                DamageEvent {
                    source: player_entity,
                    target: *enemy_entity,
                    damage: ultimate_damage,
                    damage_type: combat.attack.damage_type,
                    is_critical: true,
                },
            );

            commands.entity(*enemy_entity).insert(
                CombatEffect::new(CombatEffectType::Stun, 1.5, 0.0)
            );
        }
    }

    player_combat.ultimate_cooldown = 30.0;
}

fn update_player_combat(
    time: Res<Time>,
    mut player_query: Query<&mut PlayerCombat>,
) {
    for mut player_combat in player_query.iter_mut() {
        if player_combat.special_attack_cooldown > 0.0 {
            player_combat.special_attack_cooldown -= time.delta_secs();
        }
        if player_combat.ultimate_cooldown > 0.0 {
            player_combat.ultimate_cooldown -= time.delta_secs();
        }
        if player_combat.combo_timer > 0.0 {
            player_combat.combo_timer -= time.delta_secs();
            if player_combat.combo_timer <= 0.0 {
                player_combat.combo_count = 0;
            }
        }
    }
}