use bevy::prelude::*;
use crate::components::resource::Inventory;

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    pub id: u64,
    pub name: String,
}

use crate::components::equipment::EquipmentBar;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub inventory: Inventory,
    pub equipment_bar: EquipmentBar,
    pub sprite: Sprite,     // Added Sprite component
    pub transform: Transform,
    // Add other bundles like SpriteBundle/GlobalTransform if needed
}
