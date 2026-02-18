use bevy::prelude::*;
use bevy::sprite_render::prelude::*;
use bevy::mesh::Mesh2d;
use bevy::mesh::Mesh;
use crate::resources::world::{WorldMap, MapTile, TileType};
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
    commands.insert_resource(world_map);

    info!("WorldMap resource initialized with Native Sprite Rendering");
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

    MapRenderAssets {
        tile_mesh,
        grass_material,
        forest_material,
        desert_material,
        mountain_material,
        water_material,
        dark_forest_material,
    }
}