use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy_rapier2d::prelude::*;
use crate::player::{Player, CharacterAnimation, Direction};

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut CharacterAnimation), With<Player>>,
) {
    let mut direction_vec = Vec2::ZERO;
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

    for (mut velocity, mut anim) in query.iter_mut() {
        if direction_vec != Vec2::ZERO {
            velocity.linvel = direction_vec.normalize() * 100.0;
            if let Some(dir) = new_direction {
                anim.direction = dir;
            }
            anim.moving = true;
        } else {
            velocity.linvel = Vec2::ZERO;
            anim.moving = false;
        }
    }
}