use bevy::prelude::*;
use bevy::sprite_render::prelude::*;
use bevy::mesh::Mesh2d;
use bevy::mesh::Mesh;
use rand::Rng;
use crate::components::enemy::{
    Enemy, EnemyType, EnemyBase, EnemyPosition, EnemyStatus, EnemySpawnConfig
};
use crate::components::player::Player;
use crate::resources::world::WorldMap;

#[derive(Resource)]
pub struct EnemyRenderAssets {
    pub enemy_mesh: Handle<Mesh>,
    pub enemy_material: Handle<ColorMaterial>,
    pub base_mesh: Handle<Mesh>,
    pub fortress_material: Handle<ColorMaterial>,
    pub mother_base_material: Handle<ColorMaterial>,
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnConfig>();

        app.add_systems(Update, (
            update_enemy_spawns,
            update_base_spawns,
        ).chain());
    }
}

fn update_enemy_spawns(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut EnemyBase, &Transform), Without<Player>>,
    world_map: Res<WorldMap>,
    assets: Res<EnemyRenderAssets>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut base, transform) in enemy_query.iter_mut() {
        if !base.active {
            continue;
        }

        base.spawn_timer += time.delta().as_secs_f32();

        if base.spawn_timer >= base.spawn_interval {
            base.spawn_timer = 0.0;

            let spawn_pos = if base.spawn_range > 0.0 {
                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let distance = rng.gen_range(1.0..base.spawn_range);

                let offset_x = angle.cos() * distance * 32.0;
                let offset_y = angle.sin() * distance * 32.0;

                let tile_x = ((transform.translation.x + offset_x) / 32.0).round() as i32;
                let tile_y = ((transform.translation.y + offset_y) / 32.0).round() as i32;

                if tile_x >= 0 && tile_x < world_map.width as i32 &&
                   tile_y >= 0 && tile_y < world_map.height as i32 {
                    Some((tile_x as u32, tile_y as u32))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((tile_x, tile_y)) = spawn_pos {
                spawn_enemy(
                    &mut commands,
                    EnemyType::CombatBot,
                    tile_x,
                    tile_y,
                    Some(entity),
                    &assets.enemy_mesh,
                    &assets.enemy_material,
                );
            }
        }
    }
}

fn update_base_spawns(
    time: Res<Time>,
    mut commands: Commands,
    world_map: Res<WorldMap>,
    assets: Res<EnemyRenderAssets>,
) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.01) {
        let edge = rng.gen_range(0..4);
        let (tile_x, tile_y) = match edge {
            0 => (rng.gen_range(0..world_map.width), 0u32),
            1 => (rng.gen_range(0..world_map.width), world_map.height - 1),
            2 => (0, rng.gen_range(0..world_map.height)),
            3 => (world_map.width - 1, rng.gen_range(0..world_map.height)),
            _ => (0, 0),
        };

        spawn_base(
            &mut commands,
            EnemyType::RobotFortress,
            tile_x,
            tile_y,
            &assets.base_mesh,
            &assets.fortress_material,
        );
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    enemy_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
    _base_entity: Option<Entity>,
    mesh_handle: &Handle<Mesh>,
    material_handle: &Handle<ColorMaterial>,
) {
    let tile_size = 32.0;
    let pos_x = tile_x as f32 * tile_size;
    let pos_y = tile_y as f32 * tile_size;

    let enemy = Enemy::new(enemy_type, 1);

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_xyz(pos_x, pos_y, 1.0),
        GlobalTransform::default(),
        enemy,
        EnemyPosition { tile_x, tile_y },
        EnemyStatus::default(),
    ));

    info!("生成敌人: {:?} at ({}, {})", enemy_type, tile_x, tile_y);
}

fn spawn_base(
    commands: &mut Commands,
    base_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
    mesh_handle: &Handle<Mesh>,
    material_handle: &Handle<ColorMaterial>,
) {
    let tile_size = 32.0;
    let pos_x = tile_x as f32 * tile_size;
    let pos_y = tile_y as f32 * tile_size;

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_xyz(pos_x, pos_y, 0.5),
        GlobalTransform::default(),
        EnemyBase::new(base_type),
        EnemyPosition { tile_x, tile_y },
    ));

    info!("生成基地: {:?} at ({}, {})", base_type, tile_x, tile_y);
}

pub fn spawn_enemy_at(
    commands: &mut Commands,
    enemy_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
    mesh_handle: &Handle<Mesh>,
    material_handle: &Handle<ColorMaterial>,
) {
    spawn_enemy(commands, enemy_type, tile_x, tile_y, None, mesh_handle, material_handle);
}

pub fn spawn_base_at(
    commands: &mut Commands,
    base_type: EnemyType,
    tile_x: u32,
    tile_y: u32,
    mesh_handle: &Handle<Mesh>,
    material_handle: &Handle<ColorMaterial>,
) {
    spawn_base(commands, base_type, tile_x, tile_y, mesh_handle, material_handle);
}

pub fn init_enemy_assets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> EnemyRenderAssets {
    let enemy_mesh = meshes.add(Rectangle::new(25.0, 25.0));
    let enemy_material = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.3, 0.3)));

    let base_mesh = meshes.add(Rectangle::new(48.0, 48.0));
    let fortress_material = materials.add(ColorMaterial::from_color(Color::srgb(0.3, 0.3, 1.0)));
    let mother_base_material = materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.0, 0.5)));

    EnemyRenderAssets {
        enemy_mesh,
        enemy_material,
        base_mesh,
        fortress_material,
        mother_base_material,
    }
}