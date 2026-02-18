use bevy::prelude::*;
use bevy::sprite_render::prelude::*;
use bevy::mesh::Mesh;
use crate::components::enemy::{EnemyBase, EnemyType, EnemyPosition, Enemy};
use crate::systems::enemy_spawn::EnemyRenderAssets;

pub struct EnemyBasePlugin;

impl Plugin for EnemyBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_base_spawning);
    }
}

fn update_base_spawning(
    time: Res<Time>,
    mut base_query: Query<(Entity, &mut EnemyBase, &Transform), Without<Enemy>>,
    mut commands: Commands,
    assets: Res<EnemyRenderAssets>,
) {
    for (_entity, mut base, transform) in base_query.iter_mut() {
        if !base.active {
            continue;
        }

        base.spawn_timer += time.delta().as_secs_f32();

        if base.spawn_timer >= base.spawn_interval {
            base.spawn_timer = 0.0;

            match base.base_type {
                EnemyType::RobotFortress => {
                    spawn_from_fortress(&mut commands, &base, transform, &assets);
                }
                EnemyType::AIMotherBase => {
                    spawn_from_mother_base(&mut commands, &base, transform, &assets);
                }
                _ => {}
            }
        }
    }
}

fn spawn_from_fortress(
    commands: &mut Commands,
    base: &EnemyBase,
    transform: &Transform,
    assets: &EnemyRenderAssets,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let spawn_types = base.get_spawn_types();
    let enemy_type = spawn_types[rng.gen_range(0..spawn_types.len())];

    let tile_size = 32.0;
    let spawn_range = base.spawn_range;
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(2.0..spawn_range) * tile_size;

    let offset_x = angle.cos() * distance;
    let offset_y = angle.sin() * distance;

    let tile_x = ((transform.translation.x + offset_x) / tile_size).round() as u32;
    let tile_y = ((transform.translation.y + offset_y) / tile_size).round() as u32;

    commands.spawn((
        bevy::mesh::Mesh2d(assets.enemy_mesh.clone()),
        MeshMaterial2d(assets.enemy_material.clone()),
        Transform::from_xyz(
            transform.translation.x + offset_x,
            transform.translation.y + offset_y,
            1.0
        ),
        GlobalTransform::default(),
        Enemy::new(enemy_type, 1),
        EnemyPosition { tile_x, tile_y },
    ));

    info!("从机器人堡垒生成: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

fn spawn_from_mother_base(
    commands: &mut Commands,
    base: &EnemyBase,
    transform: &Transform,
    assets: &EnemyRenderAssets,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let spawn_types = base.get_spawn_types();
    let enemy_type = spawn_types[rng.gen_range(0..spawn_types.len())];

    let tile_size = 32.0;
    let spawn_range = base.spawn_range;
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(3.0..spawn_range) * tile_size;

    let offset_x = angle.cos() * distance;
    let offset_y = angle.sin() * distance;

    let tile_x = ((transform.translation.x + offset_x) / tile_size).round() as u32;
    let tile_y = ((transform.translation.y + offset_y) / tile_size).round() as u32;

    commands.spawn((
        bevy::mesh::Mesh2d(assets.enemy_mesh.clone()),
        MeshMaterial2d(assets.enemy_material.clone()),
        Transform::from_xyz(
            transform.translation.x + offset_x,
            transform.translation.y + offset_y,
            1.0
        ),
        GlobalTransform::default(),
        Enemy::new(enemy_type, 1),
        EnemyPosition { tile_x, tile_y },
    ));

    info!("从AI母巢生成: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

pub fn initialize_enemy_bases(
    commands: &mut Commands,
    map_width: u32,
    map_height: u32,
    assets: &EnemyRenderAssets,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let fortress_count = 2;
    for _ in 0..fortress_count {
        let edge = rng.gen_range(0..4);
        let (tile_x, tile_y) = match edge {
            0 => (rng.gen_range(0..map_width), 0),
            1 => (rng.gen_range(0..map_width), map_height - 1),
            2 => (0, rng.gen_range(0..map_height)),
            3 => (map_width - 1, rng.gen_range(0..map_height)),
            _ => (0, 0),
        };

        let pos_x = tile_x as f32 * 32.0;
        let pos_y = tile_y as f32 * 32.0;

        commands.spawn((
            bevy::mesh::Mesh2d(assets.base_mesh.clone()),
            MeshMaterial2d(assets.fortress_material.clone()),
            Transform::from_xyz(pos_x, pos_y, 0.5),
            GlobalTransform::default(),
            EnemyBase::new(EnemyType::RobotFortress),
            EnemyPosition { tile_x, tile_y },
        ));
    }

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

    let pos_x = mother_base_x.min(map_width - 1) as f32 * 32.0;
    let pos_y = mother_base_y.min(map_height - 1) as f32 * 32.0;

    commands.spawn((
        bevy::mesh::Mesh2d(assets.base_mesh.clone()),
        MeshMaterial2d(assets.mother_base_material.clone()),
        Transform::from_xyz(pos_x, pos_y, 0.5),
        GlobalTransform::default(),
        EnemyBase::new(EnemyType::AIMotherBase),
        EnemyPosition { tile_x: mother_base_x.min(map_width - 1), tile_y: mother_base_y.min(map_height - 1) },
    ));
}