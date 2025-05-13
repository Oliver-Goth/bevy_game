use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
mod camera;
mod display;
mod movement;
mod npc;
mod player;
mod stamina;
mod tilemap;

fn main() {
    App::new()
        .add_plugins(display::WindowConfigPlugin)
        .add_plugins(tilemap::TilemapPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default()) 

        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(stamina::StaminaPrintTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))


        .add_systems(Startup, (camera::spawn_camera, npc::spawn_npc, player::spawn_player))
        .add_systems(Update, (
            animation::animate_sprite,
            camera::camera_follow_player,
            camera::camera_zoom,
            movement::player_movement,
            npc::npc_interact,
            npc::npc_patrol,
            stamina::stamina_system,
        ))
        .run();
}
