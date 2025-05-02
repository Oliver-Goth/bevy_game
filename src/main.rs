use bevy::prelude::*;

mod map;
mod movement;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (map::spawn_map, player::spawn_player))
        .add_systems(Update, movement::player_movement)
        .run();
}
