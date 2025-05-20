use bevy::prelude::*;
use crate::player::{Player, spawn_player};
use crate::stamina::Stamina;
use bevy_rapier2d::prelude::*;

pub fn reset_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<Entity, With<Player>>,
) {
    // Despawn the old player
    for entity in &player_query {
        commands.entity(entity).despawn_recursive();
    }

    // Respawn the player
    spawn_player(commands, asset_server, texture_atlas_layouts);

    println!("🔁 Player respawned and game reset.");
}

// Called on Game Over to stop the player from sliding
pub fn freeze_player_velocity(mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.linvel = Vec2::ZERO;
        velocity.angvel = 0.0;
    }
}
