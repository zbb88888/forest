use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum ResourceType {
    Metal,
    Soil,
    Energy,
}

#[derive(Component, Debug)]
pub struct ResourceItem {
    pub resource_type: ResourceType,
    pub amount: u32,
}

#[derive(Component, Default, Debug)]
pub struct Inventory {
    pub metal: u32,
    pub soil: u32,
    pub energy: u32,
}

#[derive(Component, Debug)]
pub struct MetalShield {
    pub health: u32,
    pub max_health: u32,
    pub energy_upkeep: u32, // Energy consumed per second
}
