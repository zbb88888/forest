use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Grass,
    Forest,
    Mountain,
    Water,
}

#[derive(Clone, Copy, Debug)]
pub struct MapTile {
    pub tile_type: TileType,
    #[allow(dead_code)]
    pub x: u32,
    #[allow(dead_code)]
    pub y: u32,
}

#[derive(Resource)]
pub struct WorldMap {
    #[allow(dead_code)]
    pub width: u32,
    #[allow(dead_code)]
    pub height: u32,
    pub tiles: Vec<Vec<MapTile>>,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            width: 20,
            height: 20,
            tiles: Vec::new(),
        }
    }
}
