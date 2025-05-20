use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

use crate::states::GameState;
use crate::player::Player;

pub struct MapTransitionPlugin;

impl Plugin for MapTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, map_transition_system);
    }
}

#[derive(Component)]
pub struct Door {
    pub target_state: GameState,
}

pub fn map_transition_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<(&Transform, &Door), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (door_transform, door) in door_query.iter() {
            let distance = player_transform
                .translation
                .distance(door_transform.translation);

            if distance < 20.0 && keyboard_input.just_pressed(KeyCode::KeyE) {
                next_state.set(door.target_state.clone());
            }
        }
    }
}
