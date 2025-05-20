use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod states;

use crate::states::GameState;


mod animation;
mod dialogue;
mod display;
use display::{spawn_camera, camera_follow_player, camera_zoom};
mod movement;
mod npc;
mod player;
mod stamina;
mod tilemap;
mod day_night_cycle;

fn main() {
    App::new()
        .add_systems(Startup, (player::spawn_player, spawn_camera, npc::spawn_npc))
        .add_plugins((
            display::WindowConfigPlugin,
            tilemap::TilemapPlugin,
            tilemap::House1InteriorPlugin,
            tilemap::MapTransitionPlugin,
            tilemap::SchoolInteriorPlugin,
            dialogue::DialoguePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            day_night_cycle::DayNightCyclePlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(stamina::StaminaPrintTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(Update, (
            animation::animate_sprite,
            camera_follow_player,
            camera_zoom,
            movement::player_movement,
            npc::npc_interact,
            npc::npc_patrol,
            stamina::stamina_system,
        ))
        .insert_state(GameState::Outside)
        .run();
}
