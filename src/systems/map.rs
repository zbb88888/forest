use bevy::prelude::*;
use crate::resources::world::{WorldMap, MapTile, TileType};
use rand::Rng;

pub fn setup_map(mut commands: Commands) {
    let width = 20;
    let height = 20;
    let mut rng = rand::thread_rng();
    let tile_size = 32.0; // Size in pixels
    
    let mut tiles = Vec::with_capacity(height as usize);
    
    // Map offset to center it on screen (0,0)
    let offset_x = -(width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(height as f32 * tile_size) / 2.0 + tile_size / 2.0;

    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let tile_type = match rng.gen_range(0..100) {
                0..=60 => TileType::Grass,
                61..=85 => TileType::Forest,
                86..=95 => TileType::Mountain,
                _ => TileType::Water,
            };
            
            // Visual representation
            // We use Sprite component directly as per Bevy 0.18 standards
            // Required Components (Transform, Visibility) are added automatically
            let color = match tile_type {
                TileType::Grass => Color::srgb(0.2, 0.8, 0.2),    // Green
                TileType::Forest => Color::srgb(0.0, 0.4, 0.0),   // Dark Green
                TileType::Mountain => Color::srgb(0.5, 0.5, 0.5), // Grey
                TileType::Water => Color::srgb(0.2, 0.2, 0.8),    // Blue
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(tile_size - 1.0)), // -1.0 for grid lines
                    ..default()
                },
                Transform::from_xyz(
                    offset_x + x as f32 * tile_size, 
                    offset_y + y as f32 * tile_size, 
                    0.0
                ),
            ));

            row.push(MapTile {
                tile_type,
                x,
                y,
            });
        }
        tiles.push(row);
    }
    
    let world_map = WorldMap {
        width,
        height,
        tiles,
    };
    
    // Print map preview in headless mode (kept for debugging)
    #[cfg(target_os = "linux")] // Or check for headless feature/env
    if std::env::var("HEADLESS").is_ok() {
        println!("--- World Map Preview ---");
        for row in world_map.tiles.iter().rev() {
            let line: String = row.iter().map(|tile| {
                match tile.tile_type {
                    TileType::Grass => ".",
                    TileType::Forest => "T",
                    TileType::Mountain => "^",
                    TileType::Water => "~",
                }
            }).collect();
            println!("{}", line);
        }
        println!("-------------------------");
    }
    
    commands.insert_resource(world_map);
    info!("WorldMap resource initialized with Native Sprite Rendering");
}
