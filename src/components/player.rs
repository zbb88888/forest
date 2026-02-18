use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub level: u32,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name, level: 1 }
    }
}
