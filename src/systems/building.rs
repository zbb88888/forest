use bevy::prelude::*;
use crate::components::building::{
    Building, BuildingType, BuildingStatus, BuildingPosition, Inventory, ResourceType
};
use crate::resources::world::WorldMap;

/// 建筑建造系统插件
pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(Update, (
                update_construction.run_if(in_state(crate::states::GameState::InGame)),
                update_building_production.run_if(in_state(crate::states::GameState::InGame)),
                update_building_function.run_if(in_state(crate::states::GameState::InGame)),
            ));
    }
}

/// 更新建造进度
fn update_construction(
    time: Res<Time>,
    mut building_query: Query<(&mut BuildingStatus, &Building)>,
) {
    for (mut status, building) in building_query.iter_mut() {
        if status.is_constructing {
            status.construction_progress += time.delta_secs() / building.stats.build_time;

            if status.construction_progress >= 1.0 {
                status.is_constructing = false;
                status.construction_progress = 0.0;
                info!("建造完成: {:?}", building.building_type);
            }
        }
    }
}

/// 更新建筑生产
fn update_building_production(
    time: Res<Time>,
    mut building_query: Query<(&mut Building, &BuildingStatus)>,
    mut inventory: ResMut<Inventory>,
) {
    for (mut building, status) in building_query.iter_mut() {
        if !status.is_constructing && building.is_operational {
            if let Some(resource_type) = building.produce(time.delta_secs()) {
                match resource_type {
                    ResourceType::Energy => inventory.energy += 1,
                    ResourceType::Metal => inventory.metal += 1,
                    ResourceType::Crystal => inventory.crystal += 1,
                    ResourceType::Organic => inventory.organic += 1,
                }
            }
        }
    }
}

/// 更新建筑功能
fn update_building_function(
    building_query: Query<(&Building, &BuildingPosition)>,
    world_map: Option<Res<WorldMap>>,
) {
    let world_map = match world_map {
        Some(wm) => wm,
        None => return,
    };

    for (building, position) in building_query.iter() {
        // 根据建筑类型执行不同功能
        match building.building_type {
            BuildingType::Radar => {
                // 雷达：探索周围区域
                if let Some(_tile) = world_map.get_tile(position.tile_x, position.tile_y) {
                    // TODO: 实现雷达探索功能
                }
            }
            BuildingType::RepairStation => {
                // 维修站：修复周围建筑
                // TODO: 实现维修站功能
            }
            BuildingType::ShieldGenerator => {
                // 护盾发生器：为周围建筑提供护盾
                // TODO: 实现护盾发生器功能
            }
            _ => {}
        }
    }
}

/// 放置建筑
pub fn place_building(
    commands: &mut Commands,
    building_type: BuildingType,
    tile_x: u32,
    tile_y: u32,
    world_map: &WorldMap,
    inventory: &mut Inventory,
) -> Result<Entity, String> {
    // 检查位置是否有效
    if tile_x >= world_map.width || tile_y >= world_map.height {
        return Err("位置超出地图范围".to_string());
    }

    // 检查地形是否可建造
    if let Some(tile) = world_map.get_tile(tile_x, tile_y) {
        if !tile.tile_type.is_walkable() {
            return Err("地形不可建造".to_string());
        }
    } else {
        return Err("无效的位置".to_string());
    }

    // 创建建筑
    let mut building = Building::new(building_type);

    // 检查资源是否足够
    if !building.can_build(inventory) {
        return Err("资源不足".to_string());
    }

    // 消耗资源
    if !building.consume_build_resources(inventory) {
        return Err("消耗资源失败".to_string());
    }

    // 计算世界坐标
    let tile_size = 32.0;
    let offset_x = -(world_map.width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(world_map.height as f32 * tile_size) / 2.0 + tile_size / 2.0;

    // 生成建筑实体
    let entity = commands.spawn((
        Sprite {
            color: building_type.color(),
            custom_size: Some(Vec2::splat(tile_size * 0.8)),
            ..default()
        },
        Transform::from_xyz(
            offset_x + tile_x as f32 * tile_size,
            offset_y + tile_y as f32 * tile_size,
            2.0
        ),
        building,
        BuildingPosition {
            tile_x,
            tile_y,
        },
        BuildingStatus::default(),
    )).id();

    info!("建造 {:?} 在 ({}, {})", building_type, tile_x, tile_y);

    Ok(entity)
}

/// 升级建筑
pub fn upgrade_building(
    building_entity: Entity,
    inventory: &mut Inventory,
    building_query: &mut Query<&mut Building>,
) -> Result<bool, String> {
    let Ok(mut building) = building_query.get_mut(building_entity) else {
        return Err("建筑未找到".to_string());
    };

    // 计算升级成本
    let base_cost = 100u32;
    let upgrade_cost = (base_cost as f32 * (1.0 + building.level as f32 * 0.5)) as u32;

    // 检查资源是否足够
    if inventory.energy < upgrade_cost {
        return Err(format!("能源不足，需要 {}", upgrade_cost));
    }

    // 扣除资源
    inventory.energy -= upgrade_cost;

    // 升级建筑
    building.upgrade();

    info!("建筑升级到等级 {}", building.level);

    Ok(true)
}

/// 启动建筑
pub fn start_building(
    building_entity: Entity,
    building_query: &mut Query<&mut Building>,
) -> Result<bool, String> {
    let Ok(mut building) = building_query.get_mut(building_entity) else {
        return Err("建筑未找到".to_string());
    };

    building.is_operational = true;

    info!("建筑已启动: {:?}", building.building_type);

    Ok(true)
}

/// 停止建筑
pub fn stop_building(
    building_entity: Entity,
    building_query: &mut Query<&mut Building>,
) -> Result<bool, String> {
    let Ok(mut building) = building_query.get_mut(building_entity) else {
        return Err("建筑未找到".to_string());
    };

    building.is_operational = false;

    info!("建筑已停止: {:?}", building.building_type);

    Ok(true)
}
