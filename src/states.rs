use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Outside,
    InsideHouse1,
    InsideSchool,
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    GameOver,
}
