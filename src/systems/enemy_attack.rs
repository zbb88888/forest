use bevy::prelude::*;
use crate::components::enemy::{Enemy, EnemyStatus, AttackType};

/// 敌人攻击系统插件
pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_enemy_attacks,
            update_attack_effects,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 攻击效果组件
#[derive(Component)]
pub struct AttackEffect {
    pub damage: f32,
    pub duration: f32,
    pub timer: f32,
    pub attack_type: AttackType,
}

impl AttackEffect {
    pub fn new(damage: f32, duration: f32, attack_type: AttackType) -> Self {
        Self {
            damage,
            duration,
            timer: 0.0,
            attack_type,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 更新敌人攻击
fn update_enemy_attacks(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &mut EnemyStatus, &Transform)>,
    player_query: Query<(Entity, &Transform), With<crate::components::player::Player>>,
) {
    for (mut enemy, mut status, transform) in enemy_query.iter_mut() {
        if enemy.ai_state != crate::components::enemy::AIState::Attack {
            continue;
        }

        // 更新攻击计时器
        if status.attack_timer > 0.0 {
            status.attack_timer -= time.delta_seconds();
            if status.attack_timer <= 0.0 {
                status.is_attacking = false;
            }
        }

        // 检查攻击冷却
        if enemy.attack_cooldown > 0.0 {
            enemy.attack_cooldown -= time.delta_seconds();
            continue;
        }

        // 执行攻击
        if let Ok((player_entity, player_transform)) = player_query.get_single() {
            let distance = transform.translation.distance(player_transform.translation);

            if distance <= enemy.stats.attack_range {
                // 根据攻击类型执行攻击
                match enemy.enemy_type.attack_type() {
                    AttackType::Melee => {
                        perform_melee_attack(
                            &mut commands,
                            &enemy,
                            transform,
                            player_entity,
                            player_transform,
                        );
                    }
                    AttackType::Laser => {
                        perform_laser_attack(
                            &mut commands,
                            &enemy,
                            transform,
                            player_entity,
                            player_transform,
                        );
                    }
                    AttackType::Spit => {
                        perform_spit_attack(
                            &mut commands,
                            &enemy,
                            transform,
                            player_transform,
                        );
                    }
                    AttackType::Summon => {
                        perform_summon_attack(&mut commands, &enemy, transform);
                    }
                    AttackType::None => {
                        // 无攻击
                    }
                }

                // 设置攻击冷却
                enemy.attack_cooldown = 1.0 / enemy.stats.attack_speed;
                status.is_attacking = true;
                status.attack_timer = 1.0 / enemy.stats.attack_speed;
            }
        }
    }
}

/// 执行近战攻击
fn perform_melee_attack(
    commands: &mut Commands,
    enemy: &Enemy,
    enemy_transform: &Transform,
    target_entity: Entity,
    target_transform: &Transform,
) {
    info!("近战攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);

    // 创建攻击效果
    let direction = (target_transform.translation - enemy_transform.translation).normalize();
    let attack_range = enemy.stats.attack_range;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(attack_range, attack_range * 0.5)),
                ..default()
            },
            transform: Transform {
                translation: enemy_transform.translation + direction * (attack_range / 2.0),
                rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
                ..default()
            },
            ..default()
        },
        AttackEffect::new(enemy.stats.damage, 0.2, AttackType::Melee),
    ));

    // 对目标造成伤害
    // 实际实现需要通过事件系统或直接修改玩家生命值
}

/// 执行激光攻击
fn perform_laser_attack(
    commands: &mut Commands,
    enemy: &Enemy,
    enemy_transform: &Transform,
    target_entity: Entity,
    target_transform: &Transform,
) {
    info!("激光攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);

    // 计算激光方向
    let direction = (target_transform.translation - enemy_transform.translation).normalize();
    let laser_length = enemy.stats.attack_range;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(laser_length, 4.0)),
                ..default()
            },
            transform: Transform {
                translation: enemy_transform.translation + direction * (laser_length / 2.0),
                rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
                ..default()
            },
            ..default()
        },
        AttackEffect::new(enemy.stats.damage, 0.3, AttackType::Laser),
    ));

    // 激光攻击对路径上的所有目标造成伤害
    // 实际实现需要射线检测
}

/// 执行喷吐攻击
fn perform_spit_attack(
    commands: &mut Commands,
    enemy: &Enemy,
    enemy_transform: &Transform,
    target_transform: &Transform,
) {
    info!("喷吐攻击: {:?}, 伤害: {}", enemy.enemy_type, enemy.stats.damage);

    // 创建喷吐物
    let direction = (target_transform.translation - enemy_transform.translation).normalize();
    let speed = 200.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            transform: *enemy_transform,
            ..default()
        },
        AttackEffect::new(enemy.stats.damage, 2.0, AttackType::Spit),
        Projectile {
            direction,
            speed,
            lifetime: 2.0,
        },
    ));

    // 喷吐物会移动并在碰撞时造成伤害
}

/// 喷吐物组件
#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub speed: f32,
    pub lifetime: f32,
}

/// 执行召唤攻击
fn perform_summon_attack(
    commands: &mut Commands,
    enemy: &Enemy,
    enemy_transform: &Transform,
) {
    info!("召唤攻击: {:?}", enemy.enemy_type);

    // 创建召唤效果
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 1.0),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: *enemy_transform,
            ..default()
        },
        AttackEffect::new(0.0, 1.0, AttackType::Summon),
    ));

    // 实际召唤逻辑在生成系统中实现
    // 这里只创建视觉效果
}

/// 更新攻击效果
fn update_attack_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut AttackEffect, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in effect_query.iter_mut() {
        effect.timer += time.delta_seconds();

        // 更新喷吐物位置
        if effect.attack_type == AttackType::Spit {
            if let Some(projectile) = effect_query.get_component::<Projectile>(entity).ok() {
                transform.translation += projectile.direction * projectile.speed * time.delta_seconds();
            }
        }

        // 淡出效果
        if effect.timer > effect.duration * 0.7 {
            if let Some(mut sprite) = effect_query.get_component::<Sprite>(entity).ok() {
                sprite.color.set_alpha(1.0 - (effect.timer / effect.duration));
            }
        }

        // 移除完成的效果
        if effect.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
