use bevy::prelude::*;

mod animation;
mod camera;
mod player;
mod movement;
mod tilemap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(tilemap::TilemapPlugin)
        .add_systems(Startup, (player::spawn_player, camera::spawn_camera))
        .add_systems(Update, (
            movement::player_movement,
            animation::animate_sprite,
            camera::camera_follow_player,
            camera::camera_zoom,
        ))
        .run();
}
