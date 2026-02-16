use bevy::prelude::*;
use bevy::hierarchy::DespawnRecursiveExt;
use crate::states::GameState;
use crate::components::resource::Inventory;
use crate::components::player::Player;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud)
           .add_systems(Update, update_hud.run_if(in_state(GameState::InGame)))
           .add_systems(OnExit(GameState::InGame), cleanup_hud);
    }
}

#[derive(Component)]
struct HUDRoot;

#[derive(Component)]
struct EnergyText;

#[derive(Component)]
struct MetalText;

#[derive(Component)]
struct SoilText;

fn setup_hud(mut commands: Commands) {
    // HUD Root Node
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), // Semi-transparent black
            HUDRoot,
        ))
        .with_children(|parent| {
            // Energy
            parent.spawn((
                Text::new("Energy: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)), // Yellow
                EnergyText,
            ));

            // Metal
            parent.spawn((
                Text::new("Metal: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)), // Gray
                MetalText,
            ));

            // Soil
            parent.spawn((
                Text::new("Soil: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.4, 0.2)), // Brown
                SoilText,
            ));
        });
}

fn update_hud(
    player_query: Query<&Inventory, With<Player>>,
    mut energy_query: Query<&mut Text, (With<EnergyText>, Without<MetalText>, Without<SoilText>)>,
    mut metal_query: Query<&mut Text, (With<MetalText>, Without<EnergyText>, Without<SoilText>)>,
    mut soil_query: Query<&mut Text, (With<SoilText>, Without<EnergyText>, Without<MetalText>)>,
) {
    if let Some(inventory) = player_query.iter().next() {
        for mut text in energy_query.iter_mut() {
            text.0 = format!("Energy: {}", inventory.energy);
        }
        for mut text in metal_query.iter_mut() {
            text.0 = format!("Metal: {}", inventory.metal);
        }
        for mut text in soil_query.iter_mut() {
            text.0 = format!("Soil: {}", inventory.soil);
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HUDRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
