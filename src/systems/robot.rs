use bevy::prelude::*;
use crate::components::robot::{Robot, RobotType, RobotTask, RobotAI, RobotInventory};
use crate::components::plant::{Plant, Plantable, Harvestable};
use crate::components::resource::{ResourceItem, ResourceType, Inventory};
use crate::components::player::Player;
use crate::resources::world::{WorldMap, TileType};
use crate::systems::time::{GameTime, DayPhase};

/// 生成机器人
pub fn spawn_robot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if let Ok(player_transform) = player_query.get_single() {
            let position = player_transform.translation;
            spawn_robot_entity(&mut commands, RobotType::Harvester, position);
            info!("生成了采集机器人");
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        if let Ok(player_transform) = player_query.get_single() {
            let position = player_transform.translation;
            spawn_robot_entity(&mut commands, RobotType::Builder, position);
            info!("生成了建造机器人");
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        if let Ok(player_transform) = player_query.get_single() {
            let position = player_transform.translation;
            spawn_robot_entity(&mut commands, RobotType::Defender, position);
            info!("生成了防御机器人");
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        if let Ok(player_transform) = player_query.get_single() {
            let position = player_transform.translation;
            spawn_robot_entity(&mut commands, RobotType::Scout, position);
            info!("生成了侦察机器人");
        }
    }
}

fn spawn_robot_entity(commands: &mut Commands, robot_type: RobotType, position: Vec3) {
    let robot = Robot::new(robot_type);
    let ai = RobotAI::default();
    let inventory_capacity = match robot_type {
        RobotType::Harvester => 50,
        RobotType::Builder => 30,
        RobotType::Defender => 20,
        RobotType::Scout => 10,
    };

    commands.spawn((
        Sprite {
            color: robot_type.color(),
            custom_size: Some(Vec2::splat(24.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 2.0),
        robot,
        ai,
        RobotInventory::new(inventory_capacity),
    ));
}

/// 机器人 AI 系统
pub fn robot_ai_system(
    time: Res<Time>,
    game_time: Res<GameTime>,
    world_map: Res<WorldMap>,
    mut query: Query<(Entity, &mut Robot, &RobotAI, &Transform, &mut RobotInventory)>,
    plant_query: Query<(Entity, &Plant, &Transform), (With<Plantable>, Without<Harvestable>)>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    // 夜晚机器人效率降低
    let night_multiplier = match game_time.current_phase {
        DayPhase::Night => 0.5,
        _ => 1.0,
    };

    for (entity, mut robot, ai, transform, mut inventory) in query.iter_mut() {
        // 消耗能量
        let energy_cost = robot.robot_type.energy_consumption() * time.delta_secs();
        robot.consume_energy(energy_cost);

        // 如果能量不足，返回基地
        if robot.energy <= 0.0 {
            robot.current_task = RobotTask::ReturnToBase;
        }

        match robot.current_task {
            RobotTask::Idle => {
                // 根据机器人类型分配任务
                match robot.robot_type {
                    RobotType::Harvester => {
                        // 寻找可采集的植物
                        if let Some((plant_entity, plant_pos)) = find_nearest_plant(&plant_query, transform.translation, ai.detection_radius) {
                            robot.target_position = Some(plant_pos);
                            robot.current_task = RobotTask::Harvest;
                        }
                    }
                    RobotType::Scout => {
                        // 随机巡逻
                        if robot.target_position.is_none() {
                            let random_pos = generate_random_position(transform.translation, ai.patrol_radius);
                            robot.target_position = Some(random_pos);
                            robot.current_task = RobotTask::Patrol;
                        }
                    }
                    _ => {}
                }
            }
            RobotTask::Harvest => {
                // 移动到目标位置
                if let Some(target) = robot.target_position {
                    move_towards_target(&mut transform, target, robot.robot_type.movement_speed() * night_multiplier * time.delta_seconds());

                    // 检查是否到达目标
                    if transform.translation.truncate().distance(target) < 32.0 {
                        // 采集植物
                        for (plant_entity, plant, plant_transform) in plant_query.iter() {
                            if plant_transform.translation.truncate().distance(target) < 32.0 {
                                if plant.is_harvestable() {
                                    let reward = plant.calculate_harvest_reward();
                                    if inventory.add(reward, ResourceType::Energy) {
                                        commands.entity(plant_entity).despawn();
                                        info!("机器人采集了 {:?}，获得 {} 能源", plant.plant_type, reward);
                                    }
                                }
                                break;
                            }
                        }

                        // 如果背包已满，返回基地
                        if inventory.is_full() {
                            robot.current_task = RobotTask::ReturnToBase;
                        } else {
                            robot.current_task = RobotTask::Idle;
                            robot.target_position = None;
                        }
                    }
                }
            }
            RobotTask::Patrol => {
                if let Some(target) = robot.target_position {
                    move_towards_target(&mut transform, target, robot.robot_type.movement_speed() * night_multiplier * time.delta_seconds());

                    if transform.translation.truncate().distance(target) < 10.0 {
                        robot.current_task = RobotTask::Idle;
                        robot.target_position = None;
                    }
                }
            }
            RobotTask::ReturnToBase => {
                // 返回玩家位置
                if let Ok(player_transform) = player_query.get_single() {
                    let player_pos = player_transform.translation.truncate();
                    move_towards_target(&mut transform, player_pos, robot.robot_type.movement_speed() * time.delta_seconds());

                    if transform.translation.truncate().distance(player_pos) < 50.0 {
                        // 卸载资源
                        let amount = inventory.clear();
                        if amount > 0 {
                            if let Ok(mut player_inventory) = player_query.query::<&mut Inventory>().get_single_mut() {
                                player_inventory.energy += amount;
                                info!("机器人返回基地，卸载了 {} 能源", amount);
                            }
                        }

                        // 充能
                        robot.recharge(robot.max_energy);
                        robot.current_task = RobotTask::Idle;
                        robot.target_position = None;
                    }
                }
            }
            _ => {}
        }

        // 更新任务计时器
        robot.task_timer.tick(time.delta());
    }
}

/// 寻找最近的植物
fn find_nearest_plant(
    plant_query: &Query<(Entity, &Plant, &Transform), (With<Plantable>, Without<Harvestable>)>,
    position: Vec3,
    radius: f32,
) -> Option<(Entity, Vec2)> {
    let mut nearest: Option<(Entity, Vec2, f32)> = None;

    for (entity, plant, transform) in plant_query.iter() {
        let distance = transform.translation.truncate().distance(position.truncate());
        if distance < radius {
            if nearest.is_none() || distance < nearest.unwrap().2 {
                nearest = Some((entity, transform.translation.truncate(), distance));
            }
        }
    }

    nearest.map(|(entity, pos, _)| (entity, pos))
}

/// 生成随机位置
fn generate_random_position(center: Vec3, radius: f32) -> Vec2 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(0.0..radius);

    Vec2::new(
        center.x + angle.cos() * distance,
        center.y + angle.sin() * distance,
    )
}

/// 向目标移动
fn move_towards_target(transform: &mut Transform, target: Vec2, speed: f32) {
    let current = transform.translation.truncate();
    let direction = (target - current).normalize();
    let new_pos = current + direction * speed;

    transform.translation.x = new_pos.x;
    transform.translation.y = new_pos.y;
}
