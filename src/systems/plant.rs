use bevy::prelude::*;
use crate::components::plant::{Plant, PlantType, Growable, Plantable, Harvestable};
use crate::components::resource::{ResourceItem, ResourceType};
use crate::resources::world::{WorldMap, TileType};
use crate::systems::time::{GameTime, DayPhase};

/// 种植植物
pub fn plant_seed(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tile_size = 32.0;
        let offset_x = -(world_map.width as f32 * tile_size) / 2.0 + tile_size / 2.0;
        let offset_y = -(world_map.height as f32 * tile_size) / 2.0 + tile_size / 2.0;

        let tile_x = ((world_position.x - offset_x) / tile_size).round() as i32;
        let tile_y = ((world_position.y - offset_y) / tile_size).round() as i32;

        if tile_x >= 0 && tile_y >= 0 {
            let tile_x = tile_x as u32;
            let tile_y = tile_y as u32;

            if let Some(tile) = world_map.get_tile(tile_x, tile_y) {
                // 只能在草地、森林或黑暗森林上种植
                if matches!(tile.tile_type, TileType::Grass | TileType::Forest | TileType::DarkForest) {
                    let plant_type = match tile.tile_type {
                        TileType::Grass => PlantType::Grass,
                        TileType::Forest => PlantType::Bush,
                        TileType::DarkForest => PlantType::EnergyFlower,
                        _ => PlantType::Grass,
                    };

                    spawn_plant(&mut commands, plant_type, tile_x, tile_y, tile_size, offset_x, offset_y);
                    info!("种植了 {:?} 在 ({}, {})", plant_type, tile_x, tile_y);
                }
            }
        }
    }
}

/// 生成植物实体
fn spawn_plant(
    commands: &mut Commands,
    plant_type: PlantType,
    tile_x: u32,
    tile_y: u32,
    tile_size: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let plant = Plant::new(plant_type);
    let growable = Growable::new(plant_type.base_growth_rate(), 5);

    commands.spawn((
        Sprite {
            color: plant_type.color(),
            custom_size: Some(Vec2::splat(tile_size * 0.6)),
            ..default()
        },
        Transform::from_xyz(
            offset_x + tile_x as f32 * tile_size,
            offset_y + tile_y as f32 * tile_size,
            1.0
        ),
        plant,
        growable,
        Plantable,
    ));
}

/// 植物生长系统
pub fn grow_plants(
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Plant, &mut Growable, &mut Transform)>,
) {
    // 白天生长更快
    let day_multiplier = match game_time.current_phase {
        DayPhase::Day => 1.5,
        DayPhase::Dawn | DayPhase::Dusk => 1.0,
        DayPhase::Night => 0.5,
    };

    for (entity, mut plant, mut growable, mut transform) in query.iter_mut() {
        // 检查植物健康度和资源水平
        if plant.health <= 0.0 || plant.water_level <= 0.0 || plant.nutrient_level <= 0.0 {
            continue;
        }

        // 计算生长速率
        let growth_rate = growable.base_growth_rate * day_multiplier * plant.health;
        growable.growth_progress += growth_rate * time.delta_seconds();

        // 更新植物状态
        plant.maturity = growable.growth_progress / growable.max_stages as f32;

        // 阶段转换
        if growable.growth_progress >= 1.0 {
            growable.current_stage += 1;
            growable.growth_progress = 0.0;
            plant.growth_stage = growable.current_stage;

            // 更新视觉大小
            let scale = 0.6 + (growable.current_stage as f32 / growable.max_stages as f32) * 0.4;
            transform.scale = Vec3::splat(scale);

            // 如果达到最大阶段，添加可收获标记
            if growable.current_stage >= growable.max_stages {
                commands.entity(plant.entity()).insert(Harvestable);
            }
        }

        // 消耗水和营养
        plant.water_level -= 0.01 * time.delta_seconds();
        plant.nutrient_level -= 0.01 * time.delta_seconds();
    }
}

/// 收获植物
pub fn harvest_plants(
    mut commands: Commands,
    mut player_inventory: Query<&mut crate::components::resource::Inventory, With<crate::components::player::Player>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    plants_query: Query<(Entity, &Plant, &Transform)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let click_range = 32.0;

        for (entity, plant, transform) in plants_query.iter() {
            let distance = transform.translation.truncate().distance(world_position);

            if distance < click_range && plant.is_harvestable() {
                let reward = plant.calculate_harvest_reward();

                // 添加资源到玩家背包
                if let Ok(mut inventory) = player_inventory.get_single_mut() {
                    inventory.energy += reward;
                    info!("收获 {:?} 获得 {} 能源", plant.plant_type, reward);
                }

                // 生成资源掉落物
                commands.spawn((
                    ResourceItem {
                        resource_type: ResourceType::Energy,
                        amount: reward,
                    },
                    Transform::from_translation(transform.translation),
                ));

                // 移除植物
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}

/// 植物自然衰减系统
pub fn plant_decay(
    time: Res<Time>,
    mut query: Query<&mut Plant>,
) {
    for mut plant in query.iter_mut() {
        // 植物随时间自然衰减
        plant.health -= 0.001 * time.delta_seconds();
        plant.health = plant.health.max(0.0);
    }
}
