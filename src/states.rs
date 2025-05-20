use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Outside,
    InsideHouse1,
}
