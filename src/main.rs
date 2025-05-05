use bevy::prelude::*;

mod animation;
mod player;
mod movement;
mod tilemap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(tilemap::TilemapPlugin)
        .add_systems(Startup, player::spawn_player)
        .add_systems(Update, (movement::player_movement, animation::animate_sprite))
        .run();
}
