use bevy::prelude::*;
use crate::components::enemy::{
    Enemy, EnemyType, EnemyPosition, EnemyStatus, AIState, AIBehavior, AttackType
};
use crate::components::player::Player;
use crate::resources::world::WorldMap;

/// 敌人AI系统插件
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_enemy_ai.run_if(in_state(crate::states::GameState::InGame)),
            update_enemy_movement.run_if(in_state(crate::states::GameState::InGame)),
            update_enemy_attack.run_if(in_state(crate::states::GameState::InGame)),
        ));
    }
}

/// 更新敌人AI状态
fn update_enemy_ai(
    time: Res<Time>,
    mut enemy_query: Query<(
        Entity,
        &mut Enemy,
        &mut EnemyStatus,
        &Transform,
        &EnemyPosition
    )>,
    player_query: Query<&Transform, With<Player>>,
    world_map: Res<WorldMap>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation;

    for (entity, mut enemy, mut status, transform, position) in enemy_query.iter_mut() {
        if enemy.is_dead() {
            enemy.ai_state = AIState::Dead;
            continue;
        }

        let enemy_pos = transform.translation;
        let distance_to_player = enemy_pos.distance(player_pos);

        // 根据敌人类型和当前状态更新AI
        let behavior = enemy.enemy_type.ai_behavior();

        match behavior {
            AIBehavior::Passive => {
                // 被动AI：不主动攻击，只在受到攻击时反击
                if status.is_attacking {
                    enemy.ai_state = AIState::Attack;
                } else {
                    enemy.ai_state = AIState::Idle;
                }
            }

            AIBehavior::Patrol => {
                // 巡逻AI：在区域内巡逻，发现玩家后追踪
                if distance_to_player <= enemy.stats.detection_range {
                    enemy.ai_state = AIState::Chase;
                    enemy.target = Some(player_entity);
                } else {
                    enemy.ai_state = AIState::Patrol;
                    // 巡逻逻辑
                    patrol_ai(&mut enemy, &time, &world_map, position);
                }
            }

            AIBehavior::Aggressive => {
                // 主动AI：主动攻击玩家
                if distance_to_player <= enemy.stats.detection_range {
                    if distance_to_player <= enemy.stats.attack_range {
                        enemy.ai_state = AIState::Attack;
                        enemy.target = Some(player_entity);
                    } else {
                        enemy.ai_state = AIState::Chase;
                        enemy.target = Some(player_entity);
                    }
                } else {
                    enemy.ai_state = AIState::Idle;
                }
            }

            AIBehavior::Guard => {
                // 守卫AI：保护特定区域
                if distance_to_player <= enemy.stats.detection_range {
                    enemy.ai_state = AIState::Chase;
                    enemy.target = Some(player_entity);
                } else {
                    enemy.ai_state = AIState::Idle;
                    // 守卫逻辑：返回守卫位置
                    guard_ai(&mut enemy, position);
                }
            }

            AIBehavior::Ranged => {
                // 远程AI：保持距离攻击
                if distance_to_player <= enemy.stats.detection_range {
                    if distance_to_player <= enemy.stats.attack_range {
                        enemy.ai_state = AIState::Attack;
                        enemy.target = Some(player_entity);
                    } else if distance_to_player > enemy.stats.attack_range * 1.5 {
                        enemy.ai_state = AIState::Chase;
                        enemy.target = Some(player_entity);
                    } else {
                        // 保持最佳攻击距离
                        enemy.ai_state = AIState::Chase;
                        enemy.target = Some(player_entity);
                    }
                } else {
                    enemy.ai_state = AIState::Idle;
                }
            }

            AIBehavior::Boss => {
                // Boss AI：特殊行为
                boss_ai(&mut enemy, &mut status, distance_to_player, &player_query);
            }

            AIBehavior::Spawn => {
                // 生成AI：不移动，只生成其他敌人
                enemy.ai_state = AIState::Idle;
            }
        }
    }
}

/// 巡逻AI逻辑
fn patrol_ai(enemy: &mut Enemy, time: &Time, world_map: &WorldMap, position: &EnemyPosition) {
    // 简化的巡逻逻辑：随机选择相邻瓦片
    let tile_size = 32.0;
    let patrol_radius = 5.0;

    // 使用时间作为随机种子
    let seed = (time.elapsed_secs() * 100.0) as u32;
    let random_direction = seed % 4;

    let (new_x, new_y) = match random_direction {
        0 => (position.tile_x as i32 + 1, position.tile_y as i32),
        1 => (position.tile_x as i32 - 1, position.tile_y as i32),
        2 => (position.tile_x as i32, position.tile_y as i32 + 1),
        3 => (position.tile_x as i32, position.tile_y as i32 - 1),
        _ => (position.tile_x as i32, position.tile_y as i32),
    };

    // 检查新位置是否有效
    if new_x >= 0 && new_y >= 0 {
        let new_x = new_x as u32;
        let new_y = new_y as u32;

        if let Some(tile) = world_map.get_tile(new_x, new_y) {
            if tile.tile_type.is_walkable() {
                // 计算距离，确保不超出巡逻范围
                let center_x = position.tile_x as f32 * tile_size;
                let center_y = position.tile_y as f32 * tile_size;
                let target_x = new_x as f32 * tile_size;
                let target_y = new_y as f32 * tile_size;

                let distance = ((target_x - center_x).powi(2) + (target_y - center_y).powi(2)).sqrt();
                if distance <= patrol_radius * tile_size {
                    // 更新目标位置（实际移动在movement系统中处理）
                }
            }
        }
    }
}

/// 守卫AI逻辑
fn guard_ai(enemy: &mut Enemy, position: &EnemyPosition) {
    // 守卫逻辑：返回守卫位置
    // 实际移动在movement系统中处理
    // 这里只是设置AI状态
    enemy.ai_state = AIState::Idle;
}

/// Boss AI逻辑
fn boss_ai(
    enemy: &mut Enemy,
    status: &mut EnemyStatus,
    distance_to_player: f32,
    player_query: &Query<&Transform, With<Player>>,
) {
    let health_percent = enemy.health_percentage();

    // Boss根据生命值和距离改变行为
    if health_percent < 0.3 {
        // 生命值低时撤退
        enemy.ai_state = AIState::Retreat;
    } else if distance_to_player <= enemy.stats.detection_range {
        if distance_to_player <= enemy.stats.attack_range {
            enemy.ai_state = AIState::Attack;
            // Target will be set by the AI system
        } else {
            enemy.ai_state = AIState::Chase;
            // Target will be set by the AI system
        }
    } else {
        enemy.ai_state = AIState::Idle;
    }

    // Boss特殊技能
    if enemy.enemy_type == EnemyType::QueenBug {
        // 虫后可以召唤其他敌人
        if health_percent < 0.5 && !status.is_attacking {
            // 召唤逻辑（在生成系统中实现）
        }
    }
}

/// 更新敌人移动
fn update_enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Enemy, &EnemyPosition)>,
) {
    for (mut transform, enemy, position) in enemy_query.iter_mut() {
        if enemy.ai_state == AIState::Dead {
            continue;
        }

        let tile_size = 32.0;
        let speed = enemy.stats.movement_speed * tile_size * time.delta_secs();

        match enemy.ai_state {
            AIState::Chase => {
                // 追逐玩家
                if let Some(target) = enemy.target {
                    // 这里需要获取目标的位置
                    // 简化处理：向目标方向移动
                    // 实际实现需要查询目标实体的Transform
                }
            }
            AIState::Retreat => {
                // 撤退
                let retreat_direction = transform.translation.normalize();
                transform.translation += retreat_direction * speed;
            }
            AIState::Patrol => {
                // 巡逻移动
                // 移动逻辑在patrol_ai中处理
            }
            _ => {
                // 其他状态不移动
            }
        }
    }
}

/// 更新敌人攻击
fn update_enemy_attack(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut EnemyStatus)>,
) {
    for (mut enemy, mut status) in enemy_query.iter_mut() {
        if enemy.ai_state == AIState::Dead {
            continue;
        }

        // 更新攻击冷却
        if enemy.attack_cooldown > 0.0 {
            enemy.attack_cooldown -= time.delta_secs();
        }

        match enemy.ai_state {
            AIState::Attack => {
                // 攻击逻辑
                if enemy.attack_cooldown <= 0.0 {
                    status.is_attacking = true;
                    status.attack_timer = 1.0 / enemy.stats.attack_speed;
                    enemy.attack_cooldown = 1.0 / enemy.stats.attack_speed;

                    // 根据攻击类型执行不同的攻击
                    match enemy.enemy_type.attack_type() {
                        AttackType::Melee => {
                            // 近战攻击
                            perform_melee_attack(&enemy);
                        }
                        AttackType::Laser => {
                            // 激光攻击
                            perform_laser_attack(&enemy);
                        }
                        AttackType::Spit => {
                            // 喷吐攻击
                            perform_spit_attack(&enemy);
                        }
                        AttackType::Summon => {
                            // 召唤攻击
                            perform_summon_attack(&enemy);
                        }
                        AttackType::None => {
                            // 无攻击
                        }
                    }
                }
            }
            _ => {
                status.is_attacking = false;
            }
        }
    }
}

/// 执行近战攻击
fn perform_melee_attack(enemy: &Enemy) {
    info!("近战攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);
    // 实际实现需要：
    // 1. 检测攻击范围内的目标
    // 2. 对目标造成伤害
    // 3. 播放攻击动画
    // 4. 创建攻击效果
}

/// 执行激光攻击
fn perform_laser_attack(enemy: &Enemy) {
    info!("激光攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);
    // 实际实现需要：
    // 1. 创建激光束实体
    // 2. 对路径上的目标造成伤害
    // 3. 播放激光效果
    // 4. 播放攻击动画
}

/// 执行喷吐攻击
fn perform_spit_attack(enemy: &Enemy) {
    info!("喷吐攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);
    // 实际实现需要：
    // 1. 创建喷吐物实体
    // 2. 设置喷吐物运动轨迹
    // 3. 碰撞时对目标造成伤害
    // 4. 播放喷吐效果
}

/// 执行召唤攻击
fn perform_summon_attack(enemy: &Enemy) {
    info!("召唤攻击: {:?}", enemy.enemy_type);
    // 实际实现需要：
    // 1. 确定召唤的敌人类型
    // 2. 在附近位置生成敌人
    // 3. 播放召唤效果
    // 4. 更新召唤计数
}
