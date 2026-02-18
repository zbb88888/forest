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

#[derive(Component, Resource, Default, Debug)]
pub struct Inventory {
    pub metal: u32,
    pub soil: u32,
    pub energy: u32,
}

impl Inventory {
    pub fn get_material(&self, material_type: crate::components::crafting::MaterialType) -> u32 {
        match material_type {
            crate::components::crafting::MaterialType::Energy => self.energy,
            crate::components::crafting::MaterialType::Metal => self.metal,
            crate::components::crafting::MaterialType::Soil => self.soil,
            crate::components::crafting::MaterialType::Crystal => 0,
            crate::components::crafting::MaterialType::Organic => 0,
        }
    }

    pub fn remove_material(&mut self, material_type: crate::components::crafting::MaterialType, amount: u32) {
        match material_type {
            crate::components::crafting::MaterialType::Energy => self.energy = self.energy.saturating_sub(amount),
            crate::components::crafting::MaterialType::Metal => self.metal = self.metal.saturating_sub(amount),
            crate::components::crafting::MaterialType::Soil => self.soil = self.soil.saturating_sub(amount),
            crate::components::crafting::MaterialType::Crystal => {},
            crate::components::crafting::MaterialType::Organic => {},
        }
    }
}

#[derive(Component, Debug)]
pub struct MetalShield {
    pub health: u32,
    pub max_health: u32,
    pub energy_upkeep: u32, // Energy consumed per second
}
