use bevy::prelude::*;
use crate::systems::time::{GameTime, DayPhase, MoonPhase};

#[derive(Resource, Clone, Debug)]
pub struct EnvironmentLighting {
    pub ambient_intensity: f32,
    pub base_color: Color,
    pub emissive_intensity: f32,
    pub blood_moon_active: bool,
    pub transition_timer: f32,
    pub flash_effect: f32,
}

impl Default for EnvironmentLighting {
    fn default() -> Self {
        Self {
            ambient_intensity: 1.0,
            base_color: Color::WHITE,
            emissive_intensity: 0.15,
            blood_moon_active: false,
            transition_timer: 0.0,
            flash_effect: 0.0,
        }
    }
}

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_lighting, apply_blood_moon_effects));
    }
}

pub fn update_lighting(
    game_time: Res<GameTime>,
    mut lighting: ResMut<EnvironmentLighting>,
) {
    let prev_phase = lighting.transition_timer;
    lighting.transition_timer = game_time.hour;

    let phase_changed = (prev_phase - lighting.transition_timer).abs() > 0.1;

    if phase_changed && matches!(game_time.current_phase, DayPhase::Dusk) {
        lighting.flash_effect = 1.0;
    }

    if lighting.flash_effect > 0.0 {
        lighting.flash_effect -= 0.33;
    }

    let (light_intensity, light_color, emissive) = match game_time.current_phase {
        DayPhase::Dawn => (0.7, Color::srgb(1.0, 0.9, 0.7), 0.3),
        DayPhase::Day => (1.0, Color::srgb(0.75, 0.7, 0.51), 0.15),
        DayPhase::Dusk => (0.5, Color::srgb(1.0, 0.6, 0.4), 0.5),
        DayPhase::Night => (0.3, Color::srgb(0.3, 0.3, 0.5), 1.2),
    };

    lighting.ambient_intensity = light_intensity;
    lighting.base_color = light_color;
    lighting.emissive_intensity = emissive;

    let saturation_boost = (game_time.day as f32 / 14.0).min(1.0);
    if saturation_boost > 0.0 {
        let current = lighting.base_color.to_srgba();
        let red_boost = 1.0 + saturation_boost * 0.3;
        lighting.base_color = Color::srgb(
            current.red * red_boost,
            current.green * (1.0 - saturation_boost * 0.2),
            current.blue * (1.0 - saturation_boost * 0.3),
        );
    }

    lighting.blood_moon_active = game_time.day == 15 && game_time.moon_phase == MoonPhase::DarkMoon;
    if lighting.blood_moon_active {
        lighting.base_color = Color::srgb(1.0, 0.0, 0.2);
        lighting.emissive_intensity = 2.5;
    }
}

pub fn apply_blood_moon_effects(
    lighting: Res<EnvironmentLighting>,
) {
    if lighting.blood_moon_active {
        info!("Blood moon effect active - sky turns red");
    }
}

pub fn init_lighting(mut commands: Commands) {
    commands.insert_resource(EnvironmentLighting::default());
    info!("环境光照系统初始化完成");
}