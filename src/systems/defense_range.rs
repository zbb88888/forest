use bevy::prelude::*;
use crate::components::defense::{DefenseTower, DefenseRange};

/// 防御范围系统插件
pub struct DefenseRangePlugin;

impl Plugin for DefenseRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_defense_ranges.run_if(in_state(crate::states::GameState::InGame)),
            draw_defense_ranges.run_if(in_state(crate::states::GameState::InGame)),
        ));
    }
}

/// 更新防御范围
fn update_defense_ranges(
    mut commands: Commands,
    tower_query: Query<(Entity, &DefenseTower), Added<DefenseTower>>,
) {
    for (entity, tower) in tower_query.iter() {
        // 为每个新创建的防御塔添加范围组件
        commands.entity(entity).insert(
            DefenseRange::new(tower.stats.range)
        );
    }
}

/// 绘制防御范围
fn draw_defense_ranges(
    mut gizmos: Gizmos,
    tower_query: Query<(&Transform, &DefenseRange), With<DefenseTower>>,
) {
    for (transform, range) in tower_query.iter() {
        // 绘制圆形范围指示器
        gizmos.circle_2d(
            transform.translation.truncate(),
            range.range,
            Color::srgba(0.0, 1.0, 0.0, 0.2), // 半透明绿色
        );
    }
}

/// 更新防御塔范围
pub fn update_tower_range(
    entity: Entity,
    commands: &mut Commands,
    new_range: f32,
) {
    commands.entity(entity).insert(
        DefenseRange::new(new_range)
    );
}
