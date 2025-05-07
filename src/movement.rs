use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use crate::player::{Player, PlayerAnimation, Direction};

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerAnimation), With<Player>>,
    time: Res<Time>,
) {
    let mut direction_vec = Vec3::ZERO;
    let mut new_direction = None;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction_vec.y += 1.0;
        new_direction = Some(Direction::Up);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction_vec.y -= 1.0;
        new_direction = Some(Direction::Down);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction_vec.x -= 1.0;
        new_direction = Some(Direction::Left);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction_vec.x += 1.0;
        new_direction = Some(Direction::Right);
    }

    for (mut transform, mut anim) in query.iter_mut() {
        if direction_vec != Vec3::ZERO {
            transform.translation += direction_vec.normalize() * 100.0 * time.delta_seconds();
            if let Some(dir) = new_direction {
                anim.direction = dir;
            }
            anim.moving = true;
        } else {
            anim.moving = false;
        }
    }
}
