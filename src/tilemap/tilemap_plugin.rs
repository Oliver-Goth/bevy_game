use bevy::prelude::*;
use std::fs;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 2. Read tile path from config file
    let config_path = "assets/tile_config.txt";
    let tile_path = fs::read_to_string(config_path)
        .expect("Failed to read tile_config.txt")
        .trim()
        .to_string();

    println!("Loading background tile from: {}", tile_path);

    // 3. Load the texture
    let tile_texture: Handle<Image> = asset_server.load(tile_path);

    // 4. Tilemap config
    let tile_size = 16.0;        // pixel size of the tile
    let map_width = 50;
    let map_height = 50;

    // 5. Spawn a grid of tiles
    for y in 0..map_height {
        for x in 0..map_width {
            // Calculate the position so it's centered on the screen
            let pos_x = (x as f32 - map_width as f32 / 2.0) * tile_size;
            let pos_y = (y as f32 - map_height as f32 / 2.0) * tile_size;

            commands.spawn(SpriteBundle {
                texture: tile_texture.clone(),
                transform: Transform::from_xyz(pos_x, pos_y, -1.0), // z = -1 → behind other things
                ..Default::default()
            });
        }
    }
}
