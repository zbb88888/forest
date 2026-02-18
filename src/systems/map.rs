use bevy::prelude::*;
use bevy::mesh::Mesh2d;
use bevy::mesh::Mesh;
use crate::resources::world::{MapGrid, TileType, TileData, TILE_SIZE, CHUNK_SIZE, MapReadyEvent};
use crate::systems::time::{GameTime, DayPhase, MoonPhase};
use crate::components::player::Player;
use crate::states::GameState;
use rand::{Rng, SeedableRng};

#[derive(Resource)]
pub struct MapRenderAssets {
    pub tile_mesh: Handle<Mesh>,
    pub materials: Vec<Handle<ColorMaterial>>,
    pub metal_tree_material: Handle<ColorMaterial>,
    pub cooling_tower_material: Handle<ColorMaterial>,
    pub circuit_cable_material: Handle<ColorMaterial>,
    pub ruins_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Chunk {
    pub chunk_x: u32,
    pub chunk_y: u32,
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

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_map)
            .add_systems(OnEnter(GameState::InGame), spawn_environment_decorations.after(setup_map));
    }
}

pub fn setup_map(
    mut commands: Commands,
    assets: Res<MapRenderAssets>,
) {
    let width = 20u32;
    let height = 20u32;
    let seed = 42u64;

    let mut map_grid = MapGrid::new(width, height, seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for y in 0..height {
        for x in 0..width {
            let noise_val = rng.gen::<f32>();
            let tile_type = determine_tile_type(noise_val, x, y, width, height);
            map_grid.set(x, y, TileData::new(tile_type));
        }
    }

    commands.insert_resource(map_grid);

    spawn_chunk_mesh(&mut commands, &assets, width, height);

    commands.trigger(MapReadyEvent {
        width,
        height,
        seed,
    });

    info!("MapGrid initialized with procedural generation");
}

fn determine_tile_type(noise: f32, x: u32, y: u32, width: u32, height: u32) -> TileType {
    let dist_from_center = ((x as f32 - width as f32 / 2.0).powi(2)
        + (y as f32 - height as f32 / 2.0).powi(2)).sqrt()
        / ((width.pow(2) + height.pow(2)) as f32).sqrt();

    let adjusted_noise = noise + dist_from_center * 0.3;

    if adjusted_noise < 0.25 {
        TileType::Water
    } else if adjusted_noise < 0.4 {
        TileType::Grass
    } else if adjusted_noise < 0.6 {
        TileType::Forest
    } else if adjusted_noise < 0.75 {
        TileType::DarkForest
    } else if adjusted_noise < 0.85 {
        TileType::Desert
    } else {
        TileType::Mountain
    }
}

fn spawn_chunk_mesh(
    commands: &mut Commands,
    assets: &MapRenderAssets,
    width: u32,
    height: u32,
) {
    let offset_x = -(width as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
    let offset_y = -(height as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;

    for y in 0..height {
        for x in 0..width {
            let world_x = offset_x + x as f32 * TILE_SIZE;
            let world_y = offset_y + y as f32 * TILE_SIZE;
            let z = 0.0;

            commands.spawn((
                Mesh2d(assets.tile_mesh.clone()),
                MeshMaterial2d(assets.materials[0].clone()),
                Transform::from_xyz(world_x, world_y, z),
            ));
        }
    }
}

pub fn spawn_environment_decorations(
    mut commands: Commands,
    assets: Res<MapRenderAssets>,
    map_grid: Res<MapGrid>,
) {
    let width = map_grid.size.x;
    let height = map_grid.size.y;

    let offset_x = -(width as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
    let offset_y = -(height as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;

    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let world_x = offset_x + x as f32 * TILE_SIZE;
            let world_y = offset_y + y as f32 * TILE_SIZE;

            if let Some(tile) = map_grid.get(x, y) {
                spawn_decoration_for_tile(&mut commands, &assets, world_x, world_y, tile.tile_type, &mut rng);
            }
        }
    }
}

fn spawn_decoration_for_tile(
    commands: &mut Commands,
    assets: &MapRenderAssets,
    x: f32,
    y: f32,
    tile_type: TileType,
    rng: &mut impl Rng,
) {
    let z = 10.0 - y * 0.01;

    match tile_type {
        TileType::Forest | TileType::DarkForest | TileType::Grass => {
            if rng.gen_range(0..100) < 12 {
                commands.spawn((
                    Mesh2d(assets.tile_mesh.clone()),
                    MeshMaterial2d(assets.metal_tree_material.clone()),
                    Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.8, 1.2, 1.0)),
                    MetalTree {
                        jitter_timer: 0.0,
                        pulse_phase: rng.gen_range(0.0..std::f32::consts::TAU),
                    },
                    EnvironmentDecoration {
                        decoration_type: DecorationType::MetalTree,
                    },
                ));
            }

            if rng.gen_range(0..100) < 8 {
                let direction = Vec2::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                ).normalize_or_zero();

                commands.spawn((
                    Mesh2d(assets.tile_mesh.clone()),
                    MeshMaterial2d(assets.circuit_cable_material.clone()),
                    Transform::from_xyz(x, y, z - 0.01).with_scale(Vec3::new(2.0, 0.2, 1.0)),
                    CircuitCable {
                        flow_direction: direction,
                        pulse_speed: 2.0,
                    },
                    EnvironmentDecoration {
                        decoration_type: DecorationType::CircuitCable,
                    },
                ));
            }
        }

        TileType::Desert | TileType::Mountain => {
            if rng.gen_range(0..100) < 6 {
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

            if rng.gen_range(0..100) < 4 {
                commands.spawn((
                    Mesh2d(assets.tile_mesh.clone()),
                    MeshMaterial2d(assets.ruins_material.clone()),
                    Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.6, 0.6, 1.0)),
                    EnvironmentDecoration {
                        decoration_type: DecorationType::Ruins,
                    },
                ));
            }
        }

        TileType::Water => {}
    }
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
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0));

    let tile_colors = vec![
        Color::srgb(0.2, 0.8, 0.2),
        Color::srgb(0.0, 0.4, 0.0),
        Color::srgb(0.5, 0.5, 0.5),
        Color::srgb(0.2, 0.2, 0.8),
        Color::srgb(0.9, 0.8, 0.6),
        Color::srgb(0.1, 0.2, 0.1),
    ];

    let mat_handles: Vec<Handle<ColorMaterial>> = tile_colors
        .iter()
        .map(|c| materials.add(ColorMaterial::from_color(*c)))
        .collect();

    let metal_tree_material = materials.add(ColorMaterial::from_color(Color::srgb(0.24, 0.15, 0.14)));
    let cooling_tower_material = materials.add(ColorMaterial::from_color(Color::srgb(0.36, 0.25, 0.22)));
    let circuit_cable_material = materials.add(ColorMaterial::from_color(Color::srgb(0.0, 0.9, 1.0)));
    let ruins_material = materials.add(ColorMaterial::from_color(Color::srgb(0.0, 0.9, 1.0)));

    MapRenderAssets {
        tile_mesh,
        materials: mat_handles,
        metal_tree_material,
        cooling_tower_material,
        circuit_cable_material,
        ruins_material,
    }
}