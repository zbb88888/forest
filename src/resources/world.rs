use bevy::prelude::*;

/// 地形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Grass,      // 草地
    Forest,     // 森林
    Mountain,   // 山脉
    Water,      // 水域
    Desert,     // 沙漠
    DarkForest, // 黑暗森林
}

impl TileType {
    /// 获取地形颜色
    pub fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.2, 0.8, 0.2),      // 绿色
            TileType::Forest => Color::srgb(0.0, 0.4, 0.0),     // 深绿色
            TileType::Mountain => Color::srgb(0.5, 0.5, 0.5),  // 灰色
            TileType::Water => Color::srgb(0.2, 0.2, 0.8),      // 蓝色
            TileType::Desert => Color::srgb(0.9, 0.8, 0.6),    // 沙黄色
            TileType::DarkForest => Color::srgb(0.1, 0.2, 0.1), // 深暗绿色
        }
    }

    /// 获取地形是否可通行
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Grass | TileType::Forest | TileType::Desert | TileType::DarkForest => true,
            TileType::Mountain | TileType::Water => false,
        }
    }

    /// 获取地形的能源产出倍率
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

/// 地图瓦片
#[derive(Clone, Copy, Debug)]
pub struct MapTile {
    pub tile_type: TileType,
    pub x: u32,
    pub y: u32,
    pub explored: bool,      // 是否已探索
    pub visible: bool,        // 是否当前可见
}

impl MapTile {
    pub fn new(tile_type: TileType, x: u32, y: u32) -> Self {
        Self {
            tile_type,
            x,
            y,
            explored: false,
            visible: false,
        }
    }
}

/// 世界地图
#[derive(Resource)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<MapTile>>,
    pub fog_of_war: bool,     // 是否启用战争迷雾
    pub seed: u64,            // 地图种子
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
    /// 获取指定位置的瓦片
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&MapTile> {
        if x < self.width && y < self.height {
            self.tiles.get(y as usize).and_then(|row| row.get(x as usize))
        } else {
            None
        }
    }

    /// 获取指定位置的瓦片（可变）
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut MapTile> {
        if x < self.width && y < self.height {
            self.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize))
        } else {
            None
        }
    }

    /// 更新瓦片的探索状态
    pub fn explore_area(&mut self, center_x: u32, center_y: u32, radius: u32) {
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let nx = center_x as i32 + dx;
                let ny = center_y as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    let tile_x = nx as u32;
                    let tile_y = ny as u32;

                    if let Some(tile) = self.get_tile_mut(tile_x, tile_y) {
                        // 计算距离
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
