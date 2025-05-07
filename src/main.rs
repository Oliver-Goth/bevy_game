use bevy::prelude::*;

mod animation;
mod npc;
mod movement;
mod player;
mod tilemap;
mod display;
use display::{spawn_camera, camera_follow_player, camera_zoom};

fn main() {
    App::new()
        .add_plugins(display::WindowConfigPlugin)
        .add_plugins(tilemap::TilemapPlugin)
        .add_systems(Startup, (player::spawn_player, spawn_camera, npc::spawn_npc))
        .add_systems(Update, (
            animation::animate_sprite,
            camera_follow_player,
            camera_zoom,
            movement::player_movement,
            npc::npc_interact,
            npc::npc_patrol,
        ))
        .run();
}
