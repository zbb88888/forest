use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct GameTime {
    pub day: u32,
    pub timer: Timer,
}

#[allow(dead_code)]
pub fn update_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.timer.tick(time.delta());
    // Time update logic
}
