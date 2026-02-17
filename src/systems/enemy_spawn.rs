use bevy::prelude::*;
use rand::Rng;
use crate::components::enemy::{
    Enemy, EnemyType, EnemyBase, EnemyPosition, EnemyStatus, EnemySpawnConfig
};
use crate::resources::world::WorldMap;

/// 敌人生成系统插件
pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnConfig>()
            .add_systems(Update, (
                update_enemy_spawns,
                update_base_spawns,
            ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 更新敌人生成
fn update_enemy_spawns(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut EnemyBase, &Transform)>,
    world_map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Color>>,
) {
    for (entity, mut base, transform) in enemy_query.iter_mut() {
        if !base.active {
            continue;
        }

        // 更新生成计时器
        base.spawn_timer += time.delta_secs();

        // 检查是否可以生成
        if base.spawn_timer >= base.spawn_interval && base.can_spawn() {
            base.spawn_timer = 0.0;

            // 获取可以生成的敌人类型
            let spawn_types = base.get_spawn_types();
            if spawn_types.is_empty() {
                continue;
            }

            // 随机选择一种敌人类型
            let mut rng = rand::thread_rng();
            let enemy_type = spawn_types[rng.gen_range(0..spawn_types.len())];

            // 计算生成位置
            let spawn_pos = calculate_spawn_position(
                transform.translation,
                base.spawn_range,
                &world_map,
                &mut rng,
            );

            if let Some((tile_x, tile_y)) = spawn_pos {
                // 创建敌人实体
                spawn_enemy(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    enemy_type,
                    tile_x,
                    tile_y,
                    Some(entity),
                );

                // 更新生成计数
                base.current_spawn_count += 1;

                info!("从 {:?} 生成敌人: {:?} at ({}, {})", 
                    base.base_type, enemy_type, tile_x, tile_y);
            }
        }
    }
}

/// 计算生成位置
fn calculate_spawn_position(
    base_pos: Vec3,
    spawn_range: f32,
    world_map: &WorldMap,
    rng: &mut impl rand::Rng,
) -> Option<(u32, u32)> {
    let tile_size = 32.0;
    let max_attempts = 10;

    for _ in 0..max_attempts {
        // 随机生成偏移
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rng.gen_range(2.0..spawn_range) * tile_size;

        let offset_x = angle.cos() * distance;
        let offset_y = angle.sin() * distance;

        // 计算目标瓦片坐标
        let target_x = ((base_pos.x + offset_x) / tile_size).round() as i32;
        let target_y = ((base_pos.y + offset_y) / tile_size).round() as i32;

        // 检查坐标是否有效
        if target_x < 0 || target_y < 0 {
            continue;
        }

        let tile_x = target_x as u32;
        let tile_y = target_y as u32;

        // 检查瓦片是否存在且可通行
        if let Some(tile) = world_map.get_tile(tile_x, tile_y) {
            if tile.tile_type.is_walkable() {
                return Some((tile_x, tile_y));
            }
        }
    }

    None
}

/// 生成敌人实体
fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    enemy_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
    base_entity: Option<Entity>,
) {
    let tile_size = 32.0;
    let pos_x = tile_x as f32 * tile_size;
    let pos_y = tile_y as f32 * tile_size;

    // 创建敌人
    let enemy = Enemy::new(enemy_type, 1);
    let color = enemy_type.color();

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(tile_size * 0.8, tile_size * 0.8)),
            ..default()
        },
        Transform::from_xyz(pos_x, pos_y, 1.0),
        GlobalTransform::default(),
        enemy,
        EnemyPosition { tile_x, tile_y },
        EnemyStatus::default(),
    ));

    info!("生成敌人: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

/// 更新基地生成
fn update_base_spawns(
    time: Res<Time>,
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Color>>,
) {
    // 这里可以添加定期生成新基地的逻辑
    // 例如：每X分钟生成一个新的机器人堡垒或AI母巢

    // 示例：在地图边缘随机生成敌人
    let spawn_interval = 60.0; // 每60秒尝试生成一次
    let map_width = world_map.width;
    let map_height = world_map.height;

    // 使用时间作为随机种子
    let seed = (time.elapsed_seconds() * 100.0) as u32;
    let mut rng = rand::thread_rng();

    // 随机选择边缘位置
    let edge = rng.gen_range(0..4);
    let (tile_x, tile_y) = match edge {
        0 => (rng.gen_range(0..map_width), 0), // 上边缘
        1 => (rng.gen_range(0..map_width), map_height - 1), // 下边缘
        2 => (0, rng.gen_range(0..map_height)), // 左边缘
        3 => (map_width - 1, rng.gen_range(0..map_height)), // 右边缘
        _ => (0, 0),
    };

    // 检查位置是否有效
    if let Some(tile) = world_map.get_tile(tile_x, tile_y) {
        if tile.tile_type.is_walkable() {
            // 随机决定是否生成基地
            if rng.gen_bool(0.1) { // 10%的概率生成
                // 随机选择基地类型
                let base_type = if rng.gen_bool(0.3) {
                    EnemyType::AIMotherBase
                } else {
                    EnemyType::RobotFortress
                };

                spawn_base(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    base_type,
                    tile_x,
                    tile_y,
                );
            }
        }
    }
}

/// 生成基地实体
fn spawn_base(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    base_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
) {
    let tile_size = 32.0;
    let pos_x = tile_x as f32 * tile_size;
    let pos_y = tile_y as f32 * tile_size;
    let color = base_type.color();

    // 基地比普通敌人大
    let size = match base_type {
        EnemyType::RobotFortress => tile_size * 1.5,
        EnemyType::AIMotherBase => tile_size * 2.0,
        _ => tile_size,
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        Transform::from_xyz(pos_x, pos_y, 0.5),
        GlobalTransform::default(),
        EnemyBase::new(base_type),
        EnemyPosition { tile_x, tile_y },
    ));

    info!("生成基地: {:?} at ({}, {})", base_type, tile_x, tile_y);
}

/// 手动生成敌人（用于测试或事件触发）
pub fn spawn_enemy_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    enemy_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
) {
    spawn_enemy(commands, meshes, materials, enemy_type, tile_x, tile_y, None);
}

/// 手动生成基地（用于测试或事件触发）
pub fn spawn_base_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    base_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
) {
    spawn_base(commands, meshes, materials, base_type, tile_x, tile_y);
}
