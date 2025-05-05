use bevy::prelude::*;
use crate::player::{AnimationTimer, PlayerAnimation, Direction};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut AnimationTimer, &mut PlayerAnimation)>,
) {
    for (mut atlas, mut timer, mut anim) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if anim.moving {
                anim.frame = (anim.frame + 1) % 2;

                atlas.index = match anim.direction {
                    Direction::Left => if anim.frame == 0 { 50 } else { 77 },
                    Direction::Down => if anim.frame == 0 { 51 } else { 78 },
                    Direction::Up => if anim.frame == 0 { 52 } else { 79 },
                    Direction::Right => if anim.frame == 0 { 53 } else { 80 },
                };                
            } else {
                atlas.index = match anim.direction {
                    Direction::Left => 23,
                    Direction::Down => 24,
                    Direction::Up => 25,
                    Direction::Right => 26,
                };                
            }
        }
    }
}