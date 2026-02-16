use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct EnergySystem {
    pub current: u128,
    pub max: u128,
}

#[allow(dead_code)]
pub fn update_energy() {
    // Energy update logic
}
