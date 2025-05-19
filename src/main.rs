use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
mod day_night_cycle;
mod dialogue;
mod display;
use display::{spawn_camera, spawn_ui_camera, camera_follow_player, camera_zoom};
mod game_state;
use game_state::AppState;
mod movement;
mod npc;
mod player;
mod stamina;
mod tilemap;
mod ui;
use ui::game_over_ui::{
    spawn_game_over_ui,
    handle_restart_button_click,
    despawn_game_over_ui,
};
use ui::stamina_bar::{spawn_stamina_bar_ui, update_stamina_bar};


fn main() {
    App::new()
        .add_plugins((
            display::WindowConfigPlugin,
            tilemap::TilemapPlugin,
            dialogue::DialoguePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            // day_night_cycle::DayNightCyclePlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(stamina::StaminaPrintTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(Startup, (
            player::spawn_player,
            spawn_camera,
            spawn_ui_camera, // UI camera with layer
            npc::spawn_npc,
            |mut commands: Commands, asset_server: Res<AssetServer>, mut layouts: ResMut<Assets<TextureAtlasLayout>>| {
                spawn_stamina_bar_ui(&mut commands, &asset_server, &mut layouts);
            },
        ))
        .add_systems(Update, (
            animation::animate_sprite,
            camera_follow_player,
            camera_zoom,
            movement::player_movement,
            npc::npc_interact,
            npc::npc_patrol,
            stamina::stamina_system,
            update_stamina_bar,     
        ))
        .init_state::<AppState>()
        // Spawn UI when entering Game Over
        .add_systems(OnEnter(AppState::GameOver), spawn_game_over_ui)

        // Run button logic while in Game Over
        .add_systems(Update, handle_restart_button_click.run_if(in_state(AppState::GameOver)))

        // Despawn UI when leaving Game Over
        .add_systems(OnExit(AppState::GameOver), despawn_game_over_ui)
        .run();
}
