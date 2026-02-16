use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Loading,
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
}
