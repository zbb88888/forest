use bevy::prelude::*;
use crate::components::resource::Inventory;
use crate::components::equipment::EquipmentBar;

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    pub id: u64,
    pub name: String,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}
