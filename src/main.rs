use bevy::prelude::*;

mod animation;
mod display;
use display::{spawn_camera, camera_follow_player, camera_zoom};
mod movement;
mod npc;
mod player;
mod tilemap;

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
