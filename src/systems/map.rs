use bevy::prelude::*;
use bevy::sprite_render::prelude::*;
use bevy::mesh::Mesh2d;
use bevy::mesh::Mesh;
use crate::resources::world::{WorldMap, MapTile, TileType};
use crate::systems::time::{GameTime, DayPhase, MoonPhase};
use crate::components::player::Player;
use rand::Rng;

#[derive(Resource)]
pub struct MapRenderAssets {
    pub tile_mesh: Handle<Mesh>,
    pub grass_material: Handle<ColorMaterial>,
    pub forest_material: Handle<ColorMaterial>,
    pub desert_material: Handle<ColorMaterial>,
    pub mountain_material: Handle<ColorMaterial>,
    pub water_material: Handle<ColorMaterial>,
    pub dark_forest_material: Handle<ColorMaterial>,
    pub metal_tree_material: Handle<ColorMaterial>,
    pub cooling_tower_material: Handle<ColorMaterial>,
    pub circuit_cable_material: Handle<ColorMaterial>,
    pub ruins_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct MetalTree {
    pub jitter_timer: f32,
    pub pulse_phase: f32,
}

#[derive(Component)]
pub struct CoolingTower {
    pub heat_level: f32,
    pub pulse_timer: f32,
}

#[derive(Component)]
pub struct CircuitCable {
    pub flow_direction: Vec2,
    pub pulse_speed: f32,
}

#[derive(Component)]
pub struct EnvironmentDecoration {
    pub decoration_type: DecorationType,
}

#[derive(Clone, Copy)]
pub enum DecorationType {
    MetalTree,
    CoolingTower,
    CircuitCable,
    Ruins,
}

pub fn setup_map(
    mut commands: Commands,
    assets: Res<MapRenderAssets>,
) {
    let width = 20;
    let height = 20;
    let mut rng = rand::thread_rng();
    let tile_size = 32.0;

    let mut tiles = Vec::with_capacity(height as usize);

    let offset_x = -(width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(height as f32 * tile_size) / 2.0 + tile_size / 2.0;

    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let tile_type = match rng.gen_range(0..100) {
                0..=50 => TileType::Grass,
                51..=75 => TileType::Forest,
                76..=85 => TileType::Desert,
                86..=92 => TileType::Mountain,
                93..=97 => TileType::Water,
                _ => TileType::DarkForest,
            };

            let tile = MapTile::new(tile_type, x, y);

            let material = match tile_type {
                TileType::Grass => assets.grass_material.clone(),
                TileType::Forest => assets.forest_material.clone(),
                TileType::Desert => assets.desert_material.clone(),
                TileType::Mountain => assets.mountain_material.clone(),
                TileType::Water => assets.water_material.clone(),
                TileType::DarkForest => assets.dark_forest_material.clone(),
            };

            commands.spawn((
                Mesh2d(assets.tile_mesh.clone()),
                MeshMaterial2d(material),
                Transform::from_xyz(
                    offset_x + x as f32 * tile_size,
                    offset_y + y as f32 * tile_size,
                    0.0
                ),
            ));

            row.push(tile);
        }
        tiles.push(row);
    }

    let world_map = WorldMap::new(width, height, tiles, true, 42);
    let world_map_clone = world_map.clone();
    commands.insert_resource(world_map);

    spawn_environment_decorations(&mut commands, &assets, &world_map_clone, width, height, tile_size, offset_x, offset_y);

    info!("WorldMap resource initialized with Native Sprite Rendering");
}

fn spawn_environment_decorations(
    commands: &mut Commands,
    assets: &MapRenderAssets,
    world_map: &WorldMap,
    width: u32,
    height: u32,
    tile_size: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let world_x = offset_x + x as f32 * tile_size;
            let world_y = offset_y + y as f32 * tile_size;

            let tile = world_map.get_tile(x, y).unwrap_or_else(|| {
                world_map.tiles[y as usize].get(x as usize).unwrap()
            });

            let can_spawn_tree = matches!(tile.tile_type, TileType::Forest | TileType::DarkForest | TileType::Grass);
            let can_spawn_tower = matches!(tile.tile_type, TileType::Desert | TileType::Mountain | TileType::Grass);
            let can_spawn_cable = matches!(tile.tile_type, TileType::Grass | TileType::Forest);
            let can_spawn_ruins = matches!(tile.tile_type, TileType::Desert | TileType::Mountain);

            if can_spawn_tree && rng.gen_range(0..100) < 12 {
                spawn_metal_tree(commands, assets, world_x, world_y);
            }

            if can_spawn_tower && rng.gen_range(0..100) < 6 {
                spawn_cooling_tower(commands, assets, world_x, world_y);
            }

            if can_spawn_cable && rng.gen_range(0..100) < 8 {
                spawn_circuit_cable(commands, assets, world_x, world_y);
            }

            if can_spawn_ruins && rng.gen_range(0..100) < 4 {
                spawn_ruins(commands, assets, world_x, world_y);
            }
        }
    }
}

fn spawn_metal_tree(commands: &mut Commands, assets: &MapRenderAssets, x: f32, y: f32) {
    let z = 10.0 - y * 0.01;
    commands.spawn((
        Mesh2d(assets.tile_mesh.clone()),
        MeshMaterial2d(assets.metal_tree_material.clone()),
        Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.8, 1.2, 1.0)),
        MetalTree {
            jitter_timer: 0.0,
            pulse_phase: rand::thread_rng().gen_range(0.0..std::f32::consts::TAU),
        },
        EnvironmentDecoration {
            decoration_type: DecorationType::MetalTree,
        },
    ));
}

fn spawn_cooling_tower(commands: &mut Commands, assets: &MapRenderAssets, x: f32, y: f32) {
    let z = 10.0 - y * 0.01;
    commands.spawn((
        Mesh2d(assets.tile_mesh.clone()),
        MeshMaterial2d(assets.cooling_tower_material.clone()),
        Transform::from_xyz(x, y, z).with_scale(Vec3::new(1.5, 2.0, 1.0)),
        CoolingTower {
            heat_level: 0.0,
            pulse_timer: 0.0,
        },
        EnvironmentDecoration {
            decoration_type: DecorationType::CoolingTower,
        },
    ));
}

fn spawn_circuit_cable(commands: &mut Commands, assets: &MapRenderAssets, x: f32, y: f32) {
    let z = 5.0 - y * 0.01;
    let direction = Vec2::new(
        rand::thread_rng().gen_range(-1.0..1.0),
        rand::thread_rng().gen_range(-1.0..1.0),
    ).normalize_or_zero();

    commands.spawn((
        Mesh2d(assets.tile_mesh.clone()),
        MeshMaterial2d(assets.circuit_cable_material.clone()),
        Transform::from_xyz(x, y, z).with_scale(Vec3::new(2.0, 0.2, 1.0)),
        CircuitCable {
            flow_direction: direction,
            pulse_speed: 2.0,
        },
        EnvironmentDecoration {
            decoration_type: DecorationType::CircuitCable,
        },
    ));
}

fn spawn_ruins(commands: &mut Commands, assets: &MapRenderAssets, x: f32, y: f32) {
    let z = 10.0 - y * 0.01;
    commands.spawn((
        Mesh2d(assets.tile_mesh.clone()),
        MeshMaterial2d(assets.ruins_material.clone()),
        Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.6, 0.6, 1.0)),
        EnvironmentDecoration {
            decoration_type: DecorationType::Ruins,
        },
    ));
}

pub fn update_environment_animations(
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut metal_trees: Query<(&mut MetalTree, &mut Transform), Without<Player>>,
    mut cooling_towers: Query<(&mut CoolingTower, &mut Transform), Without<Player>>,
    mut circuit_cables: Query<(&mut CircuitCable, &mut Transform), Without<Player>>,
) {
    let delta = time.delta_secs();
    let is_night = game_time.current_phase == DayPhase::Night;
    let is_blood_moon = game_time.day == 15 && game_time.moon_phase == MoonPhase::DarkMoon;

    for (mut tree, mut transform) in metal_trees.iter_mut() {
        tree.jitter_timer += delta;

        let jitter_freq = if is_blood_moon { 24.0 } else { 2.0 };
        if tree.jitter_timer * jitter_freq > std::f32::consts::TAU {
            tree.jitter_timer = 0.0;
        }

        let jitter_amount = if is_blood_moon { 2.0 } else { 0.5 };
        let jitter = (tree.jitter_timer * jitter_freq).sin() * jitter_amount;

        transform.translation.x += jitter * delta * 10.0;

        tree.pulse_phase += delta * (if is_night { 8.0 } else { 2.0 });
    }

    for (mut tower, mut transform) in cooling_towers.iter_mut() {
        tower.pulse_timer += delta;

        let heat_level = if is_night { 1.0 } else { 0.3 };
        tower.heat_level = heat_level;

        let pulse_freq = if is_night { 8.0 } else { 2.0 };
        let pulse = (tower.pulse_timer * pulse_freq).sin() * 0.1;

        transform.scale.y = 2.0 + pulse;
    }

    for (cable, mut transform) in circuit_cables.iter_mut() {
        let flow_speed = if is_night { 3.0 } else { 0.5 };
        let offset = (time.elapsed_secs() * flow_speed) % 2.0;

        transform.translation.x += cable.flow_direction.x * offset * delta;
        transform.translation.y += cable.flow_direction.y * offset * delta;
    }
}

pub fn init_map_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> MapRenderAssets {
    let tile_mesh = meshes.add(Rectangle::new(31.0, 31.0));

    let grass_material = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.8, 0.2)));
    let forest_material = materials.add(ColorMaterial::from_color(Color::srgb(0.1, 0.5, 0.1)));
    let desert_material = materials.add(ColorMaterial::from_color(Color::srgb(0.9, 0.8, 0.5)));
    let mountain_material = materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.5, 0.5)));
    let water_material = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.4, 0.8)));
    let dark_forest_material = materials.add(ColorMaterial::from_color(Color::srgb(0.1, 0.1, 0.2)));

    let metal_tree_material = materials.add(ColorMaterial::from_color(Color::srgb(0.24, 0.15, 0.14)));
    let cooling_tower_material = materials.add(ColorMaterial::from_color(Color::srgb(0.36, 0.25, 0.22)));
    let circuit_cable_material = materials.add(ColorMaterial::from_color(Color::srgb(0.0, 0.9, 1.0)));
    let ruins_material = materials.add(ColorMaterial::from_color(Color::srgb(0.0, 0.9, 1.0)));

    MapRenderAssets {
        tile_mesh,
        grass_material,
        forest_material,
        desert_material,
        mountain_material,
        water_material,
        dark_forest_material,
        metal_tree_material,
        cooling_tower_material,
        circuit_cable_material,
        ruins_material,
    }
}