use bevy::prelude::*;
use bevy::reflect::Reflect;
use std::ops::{Index, IndexMut};

pub const TILE_SIZE: f32 = 32.0;
pub const CHUNK_SIZE: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum TileType {
    #[default]
    Grass,
    Forest,
    Mountain,
    Water,
    Desert,
    DarkForest,
}

impl TileType {
    pub fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.2, 0.8, 0.2),
            TileType::Forest => Color::srgb(0.0, 0.4, 0.0),
            TileType::Mountain => Color::srgb(0.5, 0.5, 0.5),
            TileType::Water => Color::srgb(0.2, 0.2, 0.8),
            TileType::Desert => Color::srgb(0.9, 0.8, 0.6),
            TileType::DarkForest => Color::srgb(0.1, 0.2, 0.1),
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(
            self,
            TileType::Grass | TileType::Forest | TileType::Desert | TileType::DarkForest
        )
    }

    pub fn energy_multiplier(&self) -> f32 {
        match self {
            TileType::Grass => 1.0,
            TileType::Forest => 1.5,
            TileType::Mountain => 0.5,
            TileType::Water => 0.8,
            TileType::Desert => 0.7,
            TileType::DarkForest => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct TileData {
    pub tile_type: TileType,
    pub explored: bool,
    pub visible: bool,
}

impl TileData {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            explored: false,
            visible: false,
        }
    }
}

pub type MapTile = TileData;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct MapGrid {
    pub size: UVec2,
    pub tiles: Vec<TileData>,
    pub fog_of_war: bool,
    pub seed: u64,
}

impl MapGrid {
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        Self {
            size: UVec2::new(width, height),
            tiles: vec![TileData::default(); (width * height) as usize],
            fog_of_war: true,
            seed,
        }
    }

    pub fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.size.x && y < self.size.y {
            Some((y * self.size.x + x) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&TileData> {
        self.index(x, y).map(|i| &self.tiles[i])
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        self.index(x, y).map(|i| &mut self.tiles[i])
    }

    pub fn set(&mut self, x: u32, y: u32, tile: TileData) -> bool {
        if let Some(idx) = self.index(x, y) {
            self.tiles[idx] = tile;
            true
        } else {
            false
        }
    }

    pub fn world_to_grid(world_pos: Vec2) -> UVec2 {
        UVec2::new(
            (world_pos.x / TILE_SIZE).floor() as u32,
            (world_pos.y / TILE_SIZE).floor() as u32,
        )
    }

    pub fn grid_to_world(grid_pos: UVec2) -> Vec2 {
        Vec2::new(
            grid_pos.x as f32 * TILE_SIZE,
            grid_pos.y as f32 * TILE_SIZE,
        )
    }

    pub fn explore_area(&mut self, center_x: u32, center_y: u32, radius: u32) {
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let nx = center_x as i32 + dx;
                let ny = center_y as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();
                    if distance <= radius as f32 {
                        if let Some(tile) = self.get_mut(nx as u32, ny as u32) {
                            tile.explored = true;
                            tile.visible = true;
                        }
                    }
                }
            }
        }
    }

    pub fn chunk_count(&self) -> UVec2 {
        UVec2::new(
            (self.size.x + CHUNK_SIZE - 1) / CHUNK_SIZE,
            (self.size.y + CHUNK_SIZE - 1) / CHUNK_SIZE,
        )
    }
}

impl Index<UVec2> for MapGrid {
    type Output = TileData;

    fn index(&self, index: UVec2) -> &Self::Output {
        self.get(index.x, index.y).unwrap_or_else(|| {
            panic!("MapGrid index out of bounds: {:?}", index)
        })
    }
}

impl IndexMut<UVec2> for MapGrid {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        self.get_mut(index.x, index.y).unwrap_or_else(|| {
            panic!("MapGrid index out of bounds: {:?}", index)
        })
    }
}

#[derive(Event)]
pub struct TileChangedEvent {
    pub x: u32,
    pub y: u32,
    pub old_type: TileType,
    pub new_type: TileType,
}

#[derive(Event)]
pub struct MapReadyEvent {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
}

#[derive(Resource, Default)]
pub struct MapGenerationConfig {
    pub noise_scale: f32,
    pub water_threshold: f32,
    pub mountain_threshold: f32,
    pub forest_threshold: f32,
}

impl MapGenerationConfig {
    pub fn default_config() -> Self {
        Self {
            noise_scale: 0.1,
            water_threshold: 0.3,
            mountain_threshold: 0.7,
            forest_threshold: 0.5,
        }
    }
}

#[derive(Resource, Clone)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<MapTile>>,
    pub fog_of_war: bool,
    pub seed: u64,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            width: 20,
            height: 20,
            tiles: Vec::new(),
            fog_of_war: true,
            seed: 0,
        }
    }
}

impl WorldMap {
    pub fn new(width: u32, height: u32, tiles: Vec<Vec<MapTile>>, fog_of_war: bool, seed: u64) -> Self {
        Self {
            width,
            height,
            tiles,
            fog_of_war,
            seed,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&MapTile> {
        if x < self.width && y < self.height {
            self.tiles.get(y as usize).and_then(|row| row.get(x as usize))
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut MapTile> {
        if x < self.width && y < self.height {
            self.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize))
        } else {
            None
        }
    }

    pub fn explore_area(&mut self, center_x: u32, center_y: u32, radius: u32) {
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let nx = center_x as i32 + dx;
                let ny = center_y as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    let tile_x = nx as u32;
                    let tile_y = ny as u32;

                    if let Some(tile) = self.get_tile_mut(tile_x, tile_y) {
                        let distance = ((dx * dx + dy * dy) as f32).sqrt();
                        if distance <= radius as f32 {
                            tile.explored = true;
                            tile.visible = true;
                        }
                    }
                }
            }
        }
    }
}