use bevy::prelude::*;

mod animation;
mod player;
mod movement;
mod tilemap;
mod display;
use display::{spawn_camera, camera_follow_player, camera_zoom};

fn main() {
    App::new()
        .add_plugins(display::WindowConfigPlugin)
        .add_plugins(tilemap::TilemapPlugin)
        .add_systems(Startup, (player::spawn_player, spawn_camera))
        .add_systems(Update, (
            movement::player_movement,
            animation::animate_sprite,
            camera_follow_player,
            camera_zoom,
        ))
        .run();
}
