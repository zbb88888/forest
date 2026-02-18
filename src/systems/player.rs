use bevy::prelude::*;
use bevy::sprite_render::prelude::*;
use bevy::mesh::Mesh2d;
use bevy::mesh::Mesh;
use crate::resources::world::WorldMap;
use crate::components::player::Player;
use crate::components::resource::Inventory;
use crate::components::equipment::EquipmentBar;
use crate::components::enemy::Enemy;
use crate::systems::map::MapRenderAssets;
use rand::Rng;

#[derive(Resource)]
pub struct PlayerRenderAssets {
    pub player_mesh: Handle<Mesh>,
    pub player_material: Handle<ColorMaterial>,
}

pub fn spawn_player(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    assets: Res<PlayerRenderAssets>,
) {
    let center_x = world_map.width / 2;
    let center_y = world_map.height / 2;

    let tile_size = 32.0;
    let offset_x = -(world_map.width as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = -(world_map.height as f32 * tile_size) / 2.0 + tile_size / 2.0;

    let pos_x = offset_x + center_x as f32 * tile_size;
    let pos_y = offset_y + center_y as f32 * tile_size;

    commands.spawn((
        Player {
            id: 0,
            name: "Administrator".into(),
            level: 1,
        },
        Inventory {
            metal: 0,
            soil: 0,
            energy: 100,
        },
        EquipmentBar::default(),
        Mesh2d(assets.player_mesh.clone()),
        MeshMaterial2d(assets.player_material.clone()),
        Transform::from_xyz(pos_x, pos_y, 1.0),
        GlobalTransform::default(),
    ));

    info!("Player spawned at ({}, {}) [World: {:.1}, {:.1}]", center_x, center_y, pos_x, pos_y);
}

pub fn move_player_randomly(
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut query: Query<(&Player, &mut Transform, &Inventory), Without<Enemy>>,
    world_map: Res<WorldMap>,
) {
    if timer.duration() == std::time::Duration::ZERO {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        let mut rng = rand::thread_rng();

        for (player, mut transform, inventory) in query.iter_mut() {
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

            if new_x >= 0.0 && new_x < world_map.width as f32 &&
               new_y >= 0.0 && new_y < world_map.height as f32 {

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

pub fn init_player_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> PlayerRenderAssets {
    let player_mesh = meshes.add(Rectangle::new(25.0, 25.0));
    let player_material = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.0, 0.0)));

    PlayerRenderAssets {
        player_mesh,
        player_material,
    }
}