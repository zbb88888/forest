use bevy::prelude::*;
use crate::components::defense::{
    DefenseTower, DefenseTowerType, DefenseEffect, DefenseEffectType, DefenseStats
};
use crate::components::enemy::Enemy;
use crate::components::combat::{DamageEvent, DamageType};

/// 防御塔系统插件
pub struct DefenseTowerPlugin;

impl Plugin for DefenseTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_defense_towers,
            update_tower_attacks,
            update_tower_rotation,
            update_defense_stats,
        ).chain());
    }
}

/// 更新防御塔
fn update_defense_towers(
    time: Res<Time>,
    mut tower_query: Query<(&mut DefenseTower, &Transform)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (mut tower, tower_transform) in tower_query.iter_mut() {
        if !tower.is_active {
            continue;
        }

        // 更新攻击冷却
        if tower.attack_cooldown > 0.0 {
            tower.attack_cooldown -= time.delta_secs();
        }

        // 寻找最近的敌人
        let mut nearest_enemy = None;
        let mut nearest_distance = f32::MAX;

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = tower_transform.translation.distance(enemy_transform.translation);
            if distance < nearest_distance && distance <= tower.stats.range {
                nearest_distance = distance;
                nearest_enemy = Some(enemy_entity);
            }
        }

        // 更新目标
        tower.target = nearest_enemy;
    }
}

/// 更新塔攻击
fn update_tower_attacks(
    mut commands: Commands,
    mut tower_query: Query<(Entity, &mut DefenseTower, &Transform)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (tower_entity, mut tower, tower_transform) in tower_query.iter_mut() {
        if !tower.can_attack() {
            continue;
        }

        // 检查是否有目标
        if let Some(target) = tower.target {
            if let Ok((_, target_transform)) = enemy_query.get(target) {
                // 计算距离
                let distance = tower_transform.translation.distance(target_transform.translation);
                if distance <= tower.stats.range {
                    // 执行攻击
                    perform_tower_attack(
                        &mut commands,
                        tower_entity,
                        target,
                        &tower,
                        tower_transform,
                        target_transform,
                    );

                    // 更新攻击冷却
                    tower.attack_cooldown = tower.get_attack_interval();
                }
            }
        }
    }
}

/// 执行塔攻击
fn perform_tower_attack(
    commands: &mut Commands,
    tower_entity: Entity,
    target_entity: Entity,
    tower: &DefenseTower,
    tower_transform: &Transform,
    target_transform: &Transform,
) {
    match tower.tower_type {
        DefenseTowerType::ArrowTower => {
            // 箭塔：物理伤害
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Physical,
                    is_critical: false,
                },
            );
            info!("箭塔攻击: 目标={:?}, 伤害={}", target_entity, tower.stats.damage);
        }

        DefenseTowerType::CannonTower => {
            // 炮塔：爆炸伤害
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Explosive,
                    is_critical: false,
                },
            );
            info!("炮塔攻击: 目标={:?}, 伤害={}", target_entity, tower.stats.damage);
        }

        DefenseTowerType::LaserTower => {
            // 激光塔：激光伤害
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Laser,
                    is_critical: false,
                },
            );
            info!("激光塔攻击: 目标={:?}, 伤害={}", target_entity, tower.stats.damage);
        }

        DefenseTowerType::IceTower => {
            // 冰塔：冰冻效果
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Energy,
                    is_critical: false,
                },
            );

            // 添加冰冻效果
            commands.entity(target_entity).insert(
                DefenseEffect::new(DefenseEffectType::Freeze, 2.0, 0.5),
            );
            info!("冰塔攻击: 目标={:?}, 伤害={}, 冰冻", target_entity, tower.stats.damage);
        }

        DefenseTowerType::PoisonTower => {
            // 毒塔：中毒效果
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Corrosive,
                    is_critical: false,
                },
            );

            // 添加中毒效果
            commands.entity(target_entity).insert(
                DefenseEffect::new(DefenseEffectType::Poison, 5.0, 2.0),
            );
            info!("毒塔攻击: 目标={:?}, 伤害={}, 中毒", target_entity, tower.stats.damage);
        }

        DefenseTowerType::ElectricTower => {
            // 电塔：眩晕效果
            commands.trigger(
                DamageEvent {
                    source: tower_entity,
                    target: target_entity,
                    damage: tower.stats.damage,
                    damage_type: DamageType::Energy,
                    is_critical: false,
                },
            );

            // 添加眩晕效果
            commands.entity(target_entity).insert(
                DefenseEffect::new(DefenseEffectType::Stun, 1.0, 0.0),
            );
            info!("电塔攻击: 目标={:?}, 伤害={}, 眩晕", target_entity, tower.stats.damage);
        }
    }
}

/// 更新塔旋转
fn update_tower_rotation(
    time: Res<Time>,
    mut tower_query: Query<(&DefenseTower, &mut Transform)>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for (tower, mut transform) in tower_query.iter_mut() {
        if let Some(target) = tower.target {
            if let Ok(target_transform) = enemy_query.get(target) {
                // 计算目标方向
                let direction = target_transform.translation - transform.translation;
                let target_angle = direction.y.atan2(direction.x);

                // 当前角度
                let current_angle = transform.rotation.to_euler(EulerRot::ZYX).2;

                // 计算角度差
                let mut angle_diff = target_angle - current_angle;

                // 规范化角度到 [-π, π]
                while angle_diff > std::f32::consts::PI {
                    angle_diff -= 2.0 * std::f32::consts::PI;
                }
                while angle_diff < -std::f32::consts::PI {
                    angle_diff += 2.0 * std::f32::consts::PI;
                }

                // 限制旋转速度
                let max_rotation = tower.stats.rotation_speed * time.delta_secs();
                let rotation = angle_diff.clamp(-max_rotation, max_rotation);

                // 应用旋转
                transform.rotate_z(rotation);
            }
        }
    }
}

/// 更新防御统计
fn update_defense_stats(
    mut damage_events: EventReader<DamageEvent>,
    mut tower_query: Query<&mut DefenseStats>,
) {
    for event in damage_events.iter() {
        // 检查伤害来源是否是防御塔
        if let Ok(mut stats) = tower_query.get_mut(event.source) {
            stats.total_damage += event.damage;
            stats.shots_fired += 1;
        }
    }
}

/// 创建防御塔
pub fn create_defense_tower(
    commands: &mut Commands,
    tower_type: DefenseTowerType,
    position: Vec3,
) -> Entity {
    let tower = DefenseTower::new(tower_type);
    let color = match tower_type {
        DefenseTowerType::ArrowTower => Color::srgb(0.6, 0.4, 0.2),
        DefenseTowerType::CannonTower => Color::srgb(0.4, 0.4, 0.4),
        DefenseTowerType::LaserTower => Color::srgb(0.0, 0.8, 1.0),
        DefenseTowerType::IceTower => Color::srgb(0.5, 0.8, 1.0),
        DefenseTowerType::PoisonTower => Color::srgb(0.5, 1.0, 0.5),
        DefenseTowerType::ElectricTower => Color::srgb(1.0, 0.8, 0.0),
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.0),
        GlobalTransform::default(),
        tower,
        DefenseStats::default(),
    )).id()
}

/// 升级防御塔
pub fn upgrade_defense_tower(
    tower: &mut DefenseTower,
) {
    tower.upgrade();
    info!("防御塔升级: 类型={:?}, 等级={}", tower.tower_type, tower.stats.level);
}
