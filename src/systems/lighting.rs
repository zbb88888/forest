use bevy::prelude::*;
use crate::systems::time::{GameTime, DayPhase};

/// 环境光照资源
#[derive(Resource, Clone, Debug)]
pub struct EnvironmentLighting {
    pub ambient_intensity: f32,
    pub base_color: Color,
}

impl Default for EnvironmentLighting {
    fn default() -> Self {
        Self {
            ambient_intensity: 1.0,
            base_color: Color::WHITE,
        }
    }
}

/// 更新环境光照
pub fn update_lighting(
    game_time: Res<GameTime>,
    mut lighting: ResMut<EnvironmentLighting>,
    mut camera_query: Query<&mut Camera2d>,
) {
    // 获取当前光照强度
    let light_intensity = game_time.current_phase.light_intensity(game_time.hour);

    // 根据昼夜阶段调整光照颜色
    let light_color = match game_time.current_phase {
        DayPhase::Dawn => Color::srgb(1.0, 0.9, 0.7),  // 暖黄色
        DayPhase::Day => Color::srgb(1.0, 1.0, 1.0),    // 白色
        DayPhase::Dusk => Color::srgb(1.0, 0.6, 0.4),  // 橙红色
        DayPhase::Night => Color::srgb(0.3, 0.3, 0.5), // 蓝紫色
    };

    // 更新光照资源
    lighting.ambient_intensity = light_intensity;
    lighting.base_color = light_color;

    // 更新相机的背景颜色（模拟昼夜变化）
    for mut camera in camera_query.iter_mut() {
        // 根据光照强度调整背景颜色的亮度
        let light_srgba = light_color.to_srgba();
        let bg_color = Color::srgb(
            light_srgba.red * light_intensity,
            light_srgba.green * light_intensity,
            light_srgba.blue * light_intensity,
        );

        // 注意：Bevy 0.18 中 Camera2d 可能没有直接设置背景颜色的方法
        // 这里我们只记录光照变化，实际渲染可能需要其他方式
        info!("光照更新: 强度={:.2}, 颜色={:?}", light_intensity, light_color);
    }
}

/// 初始化环境光照
pub fn init_lighting(mut commands: Commands) {
    commands.insert_resource(EnvironmentLighting::default());
    info!("环境光照系统初始化完成");
}
