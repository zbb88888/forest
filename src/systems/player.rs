use bevy::prelude::*;
use crate::resources::world::WorldMap;
use crate::components::player::Player;
use crate::components::resource::Inventory;
use crate::components::equipment::EquipmentBar;
use rand::Rng;

// Spawn player at map center
pub fn spawn_player(
    mut commands: Commands,
    world_map: Res<WorldMap>,
) {
    let center_x = world_map.width / 2;
    let center_y = world_map.height / 2;
    
    let tile_size = 32.0;
    let offset_x = -(world_map.width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(world_map.height as f32 * tile_size) / 2.0 + tile_size / 2.0;

    let pos_x = offset_x + center_x as f32 * tile_size;
    let pos_y = offset_y + center_y as f32 * tile_size;

    // Bevy 0.18: Use Required Components tuple instead of Bundle
    commands.spawn((
        Player {
            id: 0,
            name: "Administrator".into(),
            level: 1,
        },
        Inventory {
            metal: 0,
            soil: 0,
            energy: 100, // Initial energy
        },
        EquipmentBar::default(),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0), // Red player
            custom_size: Some(Vec2::splat(tile_size * 0.8)),
            ..default()
        },
        Transform::from_xyz(pos_x, pos_y, 1.0), // Z=1
        GlobalTransform::default(),
    ));
    
    info!("Player spawned at ({}, {}) [World: {:.1}, {:.1}]", center_x, center_y, pos_x, pos_y);
}

// System to randomly move player every 1.0s (Simulating input/movement)
pub fn move_player_randomly(
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut query: Query<(&Player, &mut Transform, &Inventory)>,
    world_map: Res<WorldMap>,
) {
    // Initialize timer if first run
    if timer.duration() == std::time::Duration::ZERO {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }
    
    timer.tick(time.delta());
    
    if timer.just_finished() {
        let mut rng = rand::thread_rng();
        
        for (player, mut transform, inventory) in query.iter_mut() {
            // Random direction: 0=Up, 1=Down, 2=Left, 3=Right
            let direction = rng.gen_range(0..4);
            let mut new_x = transform.translation.x;
            let mut new_y = transform.translation.y;
            
            match direction {
                0 => new_y += 1.0,
                1 => new_y -= 1.0,
                2 => new_x -= 1.0,
                3 => new_x += 1.0,
                _ => {}
            }
            
            // Bounds check
            if new_x >= 0.0 && new_x < world_map.width as f32 &&
               new_y >= 0.0 && new_y < world_map.height as f32 {
                
                // Update Logical Position (Grid) - Not stored in component yet, but logged
                // Update Visual Position (Transform)
                let tile_size = 32.0;
                let offset_x = -(world_map.width as f32 * tile_size) / 2.0 + tile_size / 2.0;
                let offset_y = -(world_map.height as f32 * tile_size) / 2.0 + tile_size / 2.0;
                
                transform.translation.x = offset_x + new_x * tile_size;
                transform.translation.y = offset_y + new_y * tile_size;
                
                info!("Player {} moved to ({:.0}, {:.0}) | Energy: {}, Metal: {}, Soil: {}", 
                      player.name, new_x, new_y, inventory.energy, inventory.metal, inventory.soil);
            } else {
                info!("Player {} hit wall at ({:.0}, {:.0})", player.name, new_x, new_y);
            }
        }
    }
}
