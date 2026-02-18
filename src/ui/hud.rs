use bevy::prelude::*;
use crate::states::GameState;
use crate::components::resource::Inventory;
use crate::components::player::Player;
use crate::systems::time::{GameTime, DayPhase, MoonPhase};

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

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct DayPhaseText;

#[derive(Component)]
struct MoonPhaseText;

fn setup_hud(mut commands: Commands) {
    // Left HUD - Resources
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

    // Right HUD - Time and Moon Phase
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), // Semi-transparent black
        ))
        .with_children(|parent| {
            // Time
            parent.spawn((
                Text::new("Day 1 06:00"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TimeText,
            ));

            // Day Phase
            parent.spawn((
                Text::new("Day"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.7)), // Light yellow
                DayPhaseText,
            ));

            // Moon Phase
            parent.spawn((
                Text::new("New Moon"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.9)), // Light blue
                MoonPhaseText,
            ));
        });
}

fn update_hud(
    player_query: Query<&Inventory, With<Player>>,
    game_time: Res<GameTime>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<EnergyText>>,
        Query<&mut Text, With<MetalText>>,
        Query<&mut Text, With<SoilText>>,
        Query<&mut Text, With<TimeText>>,
        Query<&mut Text, With<DayPhaseText>>,
        Query<&mut Text, With<MoonPhaseText>>,
    )>,
) {
    if let Some(inventory) = player_query.iter().next() {
        for mut text in text_queries.p0() {
            text.0 = format!("Energy: {}", inventory.energy);
        }
        for mut text in text_queries.p1() {
            text.0 = format!("Metal: {}", inventory.metal);
        }
        for mut text in text_queries.p2() {
            text.0 = format!("Soil: {}", inventory.soil);
        }
    }

    for mut text in text_queries.p3() {
        text.0 = format!(
            "Day {} {:02.0}:{:02.0}",
            game_time.day,
            game_time.hour,
            game_time.minute
        );
    }

    for mut text in text_queries.p4() {
        let phase_name = match game_time.current_phase {
            DayPhase::Dawn => "Dawn",
            DayPhase::Day => "Day",
            DayPhase::Dusk => "Dusk",
            DayPhase::Night => "Night",
        };
        text.0 = format!("{}", phase_name);
    }

    for mut text in text_queries.p5() {
        let phase_name = match game_time.moon_phase {
            MoonPhase::NewMoon => "New Moon",
            MoonPhase::WaxingCrescent => "Waxing Crescent",
            MoonPhase::FirstQuarter => "First Quarter",
            MoonPhase::WaxingGibbous => "Waxing Gibbous",
            MoonPhase::FullMoon => "Full Moon",
            MoonPhase::WaningGibbous => "Waning Gibbous",
            MoonPhase::LastQuarter => "Last Quarter",
            MoonPhase::WaningCrescent => "Waning Crescent",
            MoonPhase::DarkMoon => "Dark Moon",
        };
        text.0 = format!("{}", phase_name);
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HUDRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
