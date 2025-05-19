use bevy::prelude::*;

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    GameOver,
}
