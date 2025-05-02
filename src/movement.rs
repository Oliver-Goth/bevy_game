use bevy::prelude::{Res, Query, Transform, With, Vec3, Time};
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::player::Player;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(1.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(-1.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, -1.0, 0.0);
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction * 100.0 * time.delta_seconds();
    }
}
