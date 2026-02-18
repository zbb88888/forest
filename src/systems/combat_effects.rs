use bevy::prelude::*;
use crate::components::combat::{
    CombatEffect, CombatEffectType, DamageEvent, HealEvent, DeathEvent
};
use crate::components::player::Player;

pub struct CombatEffectsPlugin;

impl Plugin for CombatEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_combat_effect_visuals,
        ))
        .add_observer(handle_damage_spawn)
        .add_observer(handle_heal_spawn)
        .add_observer(handle_death_spawn);
    }
}

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

fn handle_damage_spawn(
    event: On<DamageEvent>,
    mut commands: Commands,
) {
    let damage_event = event.event();

    let color = if damage_event.is_critical {
        Color::srgb(1.0, 0.0, 0.0)
    } else {
        Color::srgb(1.0, 1.0, 1.0)
    };

    let size = if damage_event.is_critical { 24.0 } else { 16.0 };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        GlobalTransform::default(),
        DamageEffect::new(damage_event.damage, damage_event.is_critical),
    ));

    spawn_damage_particles(&mut commands, damage_event);
}

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

#[derive(Component)]
pub struct ParticleEffect {
    pub direction: Vec3,
    pub speed: f32,
    pub lifetime: f32,
    pub timer: f32,
}

fn handle_heal_spawn(
    event: On<HealEvent>,
    mut commands: Commands,
) {
    let heal_event = event.event();

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        GlobalTransform::default(),
        HealEffect::new(heal_event.amount),
    ));

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

fn handle_death_spawn(
    event: On<DeathEvent>,
    mut commands: Commands,
) {
    let _death_event = event.event();

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

fn update_combat_effect_visuals(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut CombatEffectVisual, &mut Transform), Without<Player>>,
) {
    for (entity, mut effect, _transform) in effect_query.iter_mut() {
        effect.timer += time.delta_secs();

        match effect.effect_type {
            CombatEffectType::Burn => {
                let _alpha = (effect.timer / effect.duration).sin() * 0.5 + 0.5;
            }
            CombatEffectType::Freeze => {
            }
            CombatEffectType::Poison => {
                let _alpha = (effect.timer / effect.duration).sin() * 0.5 + 0.5;
            }
            CombatEffectType::Slow => {
            }
            CombatEffectType::Stun => {
            }
            CombatEffectType::Shield => {
            }
        }

        if effect.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}