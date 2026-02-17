use bevy::prelude::*;
use bevy::ecs::event::EventReader;
use crate::components::combat::{
    DamageEvent, HealEvent, DeathEvent, CombatEffect, CombatEffectType
};

/// 战斗效果系统插件
pub struct CombatEffectsPlugin;

impl Plugin for CombatEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_damage_effects,
            spawn_heal_effects,
            spawn_death_effects,
            update_combat_effect_visuals,
        ).run_if(in_state(crate::states::GameState::InGame)));
    }
}

/// 伤害效果组件
#[derive(Component)]
pub struct DamageEffect {
    pub timer: f32,
    pub duration: f32,
    pub damage: f32,
    pub is_critical: bool,
}

impl DamageEffect {
    pub fn new(damage: f32, is_critical: bool) -> Self {
        Self {
            timer: 0.0,
            duration: 0.5,
            damage,
            is_critical,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 治疗效果组件
#[derive(Component)]
pub struct HealEffect {
    pub timer: f32,
    pub duration: f32,
    pub amount: f32,
}

impl HealEffect {
    pub fn new(amount: f32) -> Self {
        Self {
            timer: 0.0,
            duration: 0.5,
            amount,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 死亡效果组件
#[derive(Component)]
pub struct DeathEffect {
    pub timer: f32,
    pub duration: f32,
}

impl DeathEffect {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            duration: 1.0,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 战斗效果视觉组件
#[derive(Component)]
pub struct CombatEffectVisual {
    pub effect_type: CombatEffectType,
    pub timer: f32,
    pub duration: f32,
}

impl CombatEffectVisual {
    pub fn new(effect_type: CombatEffectType, duration: f32) -> Self {
        Self {
            effect_type,
            timer: 0.0,
            duration,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration
    }
}

/// 生成伤害效果
fn spawn_damage_effects(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
) {
    for event in damage_events.iter() {
        // 获取目标位置
        // 实际实现需要查询目标实体的Transform

        // 创建伤害数字效果
        let color = if event.is_critical {
            Color::srgb(1.0, 0.0, 0.0) // 暴击：红色
        } else {
            Color::srgb(1.0, 1.0, 1.0) // 普通伤害：白色
        };

        let size = if event.is_critical {
            24.0 // 暴击字体更大
        } else {
            16.0
        };

        // 创建伤害效果实体
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 10.0),
            GlobalTransform::default(),
            DamageEffect::new(event.damage, event.is_critical),
        ));

        // 创建伤害粒子效果
        spawn_damage_particles(&mut commands, event);
    }
}

/// 生成伤害粒子效果
fn spawn_damage_particles(
    commands: &mut Commands,
    event: &DamageEvent,
) {
    let particle_count = if event.is_critical { 20 } else { 10 };
    let particle_size = 4.0;

    for _ in 0..particle_count {
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let speed = rand::random::<f32>() * 100.0 + 50.0;
        let lifetime = rand::random::<f32>() * 0.3 + 0.2;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(particle_size, particle_size)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 9.0),
            GlobalTransform::default(),
            ParticleEffect {
                direction: Vec3::new(angle.cos(), angle.sin(), 0.0),
                speed,
                lifetime,
                timer: 0.0,
            },
        ));
    }
}

/// 粒子效果组件
#[derive(Component)]
pub struct ParticleEffect {
    pub direction: Vec3,
    pub speed: f32,
    pub lifetime: f32,
    pub timer: f32,
}

/// 生成治疗效果
fn spawn_heal_effects(
    mut commands: Commands,
    mut heal_events: EventReader<HealEvent>,
) {
    for event in heal_events.iter() {
        // 创建治疗数字效果
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 10.0),
            GlobalTransform::default(),
            HealEffect::new(event.amount),
        ));

        // 创建治疗粒子效果
        for _ in 0..10 {
            let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let speed = rand::random::<f32>() * 50.0 + 30.0;
            let lifetime = rand::random::<f32>() * 0.3 + 0.2;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(4.0, 4.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 9.0),
                GlobalTransform::default(),
                ParticleEffect {
                    direction: Vec3::new(angle.cos(), angle.sin(), 0.0),
                    speed,
                    lifetime,
                    timer: 0.0,
                },
            ));
        }
    }
}

/// 生成死亡效果
fn spawn_death_effects(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
) {
    for event in death_events.iter() {
        // 创建死亡粒子效果
        for _ in 0..30 {
            let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let speed = rand::random::<f32>() * 150.0 + 100.0;
            let lifetime = rand::random::<f32>() * 0.5 + 0.3;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 9.0),
                GlobalTransform::default(),
                ParticleEffect {
                    direction: Vec3::new(angle.cos(), angle.sin(), 0.0),
                    speed,
                    lifetime,
                    timer: 0.0,
                },
            ));
        }

        // 创建死亡效果实体
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 8.0),
            GlobalTransform::default(),
            DeathEffect::new(),
        ));
    }
}

/// 更新战斗效果视觉
fn update_combat_effect_visuals(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut CombatEffectVisual, &mut Transform)>,
) {
    for (entity, mut effect, mut transform) in effect_query.iter_mut() {
        effect.timer += time.delta_secs();

        // 根据效果类型更新视觉效果
        match effect.effect_type {
            CombatEffectType::Burn => {
                // 燃烧效果：红色闪烁
                let alpha = (effect.timer / effect.duration).sin() * 0.5 + 0.5;
                // 实际实现需要修改Sprite的color
            }
            CombatEffectType::Freeze => {
                // 冰冻效果：蓝色覆盖
                // 实际实现需要修改Sprite的color
            }
            CombatEffectType::Poison => {
                // 中毒效果：绿色闪烁
                let alpha = (effect.timer / effect.duration).sin() * 0.5 + 0.5;
                // 实际实现需要修改Sprite的color
            }
            CombatEffectType::Slow => {
                // 减速效果：黄色覆盖
                // 实际实现需要修改Sprite的color
            }
            CombatEffectType::Stun => {
                // 眩晕效果：紫色覆盖
                // 实际实现需要修改Sprite的color
            }
            CombatEffectType::Shield => {
                // 护盾效果：蓝色光环
                // 实际实现需要修改Sprite的color
            }
        }

        // 移除完成的效果
        if effect.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
