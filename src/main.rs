use bevy::prelude::*;

mod animation;
mod player;
mod movement;
mod tilemap;
mod display;

fn main() {
    App::new()
        .add_plugins(display::WindowConfigPlugin)
        .add_plugins(tilemap::TilemapPlugin)
        .add_systems(Startup, player::spawn_player)
        .add_systems(Update, (movement::player_movement, animation::animate_sprite))
        .run();
}
