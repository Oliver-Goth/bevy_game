use bevy::prelude::*;
use crate::player::{AnimationTimer, CharacterAnimation, Direction};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut AnimationTimer, &mut CharacterAnimation)>,
) {
    for (mut atlas, mut timer, mut anim) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if anim.moving {
                anim.frame = (anim.frame + 1) % 4;

                atlas.index = match anim.direction {
                    Direction::Left => match anim.frame {
                        0 => 4,
                        1 => 0,
                        2 => 8,
                        _ => 0,
                    },
                    Direction::Down => match anim.frame {
                        0 => 5,
                        1 => 1,
                        2 => 9,
                        _ => 1,
                    },
                    Direction::Up => match anim.frame {
                        0 => 6,
                        1 => 2,
                        2 => 10,
                        _ => 2,
                    },
                    Direction::Right => match anim.frame {
                        0 => 7,
                        1 => 3,
                        2 => 11,
                        _ => 3,
                    },
                };               
            } else {
                atlas.index = match anim.direction {
                    Direction::Left => 0,
                    Direction::Down => 1,
                    Direction::Up => 2,
                    Direction::Right => 3,
                };                
            }
        }
    }
}
