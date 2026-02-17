use bevy::prelude::*;
use crate::components::defense::{DefenseWall, DefenseStats};

/// 防御墙系统插件
pub struct DefenseWallPlugin;

impl Plugin for DefenseWallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_defense_walls,
            update_wall_health,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 更新防御墙
fn update_defense_walls(
    mut commands: Commands,
    mut wall_query: Query<(Entity, &DefenseWall, &Transform)>,
    time: Res<Time>,
) {
    for (entity, wall, transform) in wall_query.iter() {
        // 检查是否被摧毁
        if wall.is_destroyed() {
            // 播放摧毁效果
            spawn_wall_destruction(&mut commands, transform);

            // 移除墙体
            commands.entity(entity).despawn();

            info!("防御墙被摧毁: 位置={:?}", transform.translation);
        }
    }
}

/// 更新墙生命
fn update_wall_health(
    mut wall_query: Query<&mut DefenseWall>,
) {
    for mut wall in wall_query.iter_mut() {
        // 可以添加自动修复逻辑
        // 例如：每10秒恢复5%的生命值
    }
}

/// 生成墙摧毁效果
fn spawn_wall_destruction(
    commands: &mut Commands,
    transform: &Transform,
) {
    // 创建粒子效果
    for _ in 0..20 {
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let speed = rand::random::<f32>() * 100.0 + 50.0;
        let lifetime = rand::random::<f32>() * 0.5 + 0.3;

        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(4.0, 4.0)),
                ..default()
            },
            Transform::from_translation(transform.translation),
            WallDebris {
                direction: Vec3::new(angle.cos(), angle.sin(), 0.0),
                speed,
                lifetime,
                timer: 0.0,
            },
        ));
    }
}

/// 墙碎片组件
#[derive(Component)]
pub struct WallDebris {
    pub direction: Vec3,
    pub speed: f32,
    pub lifetime: f32,
    pub timer: f32,
}

/// 创建防御墙
pub fn create_defense_wall(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    let wall = DefenseWall::new();

    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.5),
        wall,
        DefenseStats::default(),
        GlobalTransform::default(),
    )).id()
}

/// 修理防御墙
pub fn repair_defense_wall(
    wall: &mut DefenseWall,
    amount: f32,
) {
    wall.repair(amount);
    info!("防御墙修理: 恢复={}, 当前生命={}/{}", 
        amount, wall.health, wall.max_health);
}

/// 升级防御墙
pub fn upgrade_defense_wall(
    wall: &mut DefenseWall,
) {
    wall.upgrade();
    info!("防御墙升级: 等级={}, 生命={}, 防御={}", 
        wall.level, wall.max_health, wall.defense);
}

/// 接受伤害
pub fn take_wall_damage(
    wall: &mut DefenseWall,
    damage: f32,
) -> f32 {
    let actual_damage = wall.take_damage(damage);
    info!("防御墙受到伤害: 原始={}, 实际={}, 剩余生命={}/{}", 
        damage, actual_damage, wall.health, wall.max_health);
    actual_damage
}
