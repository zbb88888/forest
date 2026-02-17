use bevy::prelude::*;
use crate::components::enemy::{EnemyBase, EnemyType, EnemyPosition};
use crate::systems::enemy_spawn::{spawn_enemy_at, spawn_base_at};

/// 敌人大本营系统插件
pub struct EnemyBasePlugin;

impl Plugin for EnemyBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_base_health,
            update_base_spawning,
            handle_base_destruction,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 更新基地生命
fn update_base_health(
    mut commands: Commands,
    mut base_query: Query<(Entity, &mut EnemyBase, &mut EnemyPosition)>,
    time: Res<Time>,
) {
    for (entity, mut base, position) in base_query.iter_mut() {
        // 检查基地是否需要恢复
        // 可以添加逻辑让基地在一定条件下恢复生命
        // 例如：每30秒恢复10%的生命值

        // 检查基地是否被摧毁
        if base.current_spawn_count >= base.max_spawn_count {
            // 基地耗尽生成能力，可以添加特殊效果
        }
    }
}

/// 更新基地生成
fn update_base_spawning(
    time: Res<Time>,
    mut base_query: Query<(Entity, &mut EnemyBase, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Color>>,
) {
    for (entity, mut base, transform) in base_query.iter_mut() {
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

            // 根据基地类型选择生成策略
            match base.base_type {
                EnemyType::RobotFortress => {
                    // 机器人堡垒生成策略
                    spawn_from_fortress(&mut commands, &mut meshes, &mut materials, &base, transform);
                }
                EnemyType::AIMotherBase => {
                    // AI母巢生成策略
                    spawn_from_mother_base(&mut commands, &mut meshes, &mut materials, &base, transform);
                }
                _ => {}
            }

            // 更新生成计数
            base.current_spawn_count += 1;
        }
    }
}

/// 从机器人堡垒生成敌人
fn spawn_from_fortress(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    base: &EnemyBase,
    transform: &Transform,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 机器人堡垒生成机器人敌人
    let spawn_types = base.get_spawn_types();
    let enemy_type = spawn_types[rng.gen_range(0..spawn_types.len())];

    // 计算生成位置
    let tile_size = 32.0;
    let spawn_range = base.spawn_range;
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(2.0..spawn_range) * tile_size;

    let offset_x = angle.cos() * distance;
    let offset_y = angle.sin() * distance;

    let tile_x = ((transform.translation.x + offset_x) / tile_size).round() as u32;
    let tile_y = ((transform.translation.y + offset_y) / tile_size).round() as u32;

    spawn_enemy_at(commands, meshes, materials, enemy_type, tile_x, tile_y);

    info!("从机器人堡垒生成: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

/// 从AI母巢生成敌人
fn spawn_from_mother_base(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    base: &EnemyBase,
    transform: &Transform,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // AI母巢可以生成更多类型的敌人
    let spawn_types = base.get_spawn_types();
    let enemy_type = spawn_types[rng.gen_range(0..spawn_types.len())];

    // 计算生成位置
    let tile_size = 32.0;
    let spawn_range = base.spawn_range;
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(3.0..spawn_range) * tile_size;

    let offset_x = angle.cos() * distance;
    let offset_y = angle.sin() * distance;

    let tile_x = ((transform.translation.x + offset_x) / tile_size).round() as u32;
    let tile_y = ((transform.translation.y + offset_y) / tile_size).round() as u32;

    spawn_enemy_at(commands, meshes, materials, enemy_type, tile_x, tile_y);

    info!("从AI母巢生成: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

/// 处理基地摧毁
fn handle_base_destruction(
    mut commands: Commands,
    mut base_query: Query<(Entity, &EnemyBase, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Color>>,
) {
    for (entity, base, transform) in base_query.iter_mut() {
        // 检查基地是否被摧毁
        // 实际实现需要检查基地的生命值
        // 这里只是示例

        // 如果基地被摧毁：
        // 1. 播放摧毁效果
        // 2. 掉落奖励
        // 3. 可能生成新的基地
    }
}

/// 初始化敌人大本营
pub fn initialize_enemy_bases(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<Color>>,
    map_width: u32,
    map_height: u32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 在地图边缘生成机器人堡垒
    let fortress_count = 2;
    for _ in 0..fortress_count {
        // 随机选择边缘位置
        let edge = rng.gen_range(0..4);
        let (tile_x, tile_y) = match edge {
            0 => (rng.gen_range(0..map_width), 0),
            1 => (rng.gen_range(0..map_width), map_height - 1),
            2 => (0, rng.gen_range(0..map_height)),
            3 => (map_width - 1, rng.gen_range(0..map_height)),
            _ => (0, 0),
        };

        spawn_base_at(commands, meshes, materials, EnemyType::RobotFortress, tile_x, tile_y);
    }

    // 在地图中心附近生成AI母巢
    let center_x = map_width / 2;
    let center_y = map_height / 2;
    let offset = rng.gen_range(5..10);
    let mother_base_x = if rng.gen_bool(0.5) {
        center_x + offset
    } else {
        center_x - offset
    };
    let mother_base_y = if rng.gen_bool(0.5) {
        center_y + offset
    } else {
        center_y - offset
    };

    spawn_base_at(
        commands,
        meshes,
        materials,
        EnemyType::AIMotherBase,
        mother_base_x.min(map_width - 1),
        mother_base_y.min(map_height - 1),
    );
}
