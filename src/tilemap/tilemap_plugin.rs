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
    // === CONFIGURATION ===
    let tile_size = 16.0;
    let map_width = 50;
    let map_height = 36;
    let background_tile_id = 28; // tile_0028.png

    // === BACKGROUND LAYER ===
    let background_path = format!("Tiles/tile_{:04}.png", background_tile_id);
    let background_texture: Handle<Image> = asset_server.load(background_path);

    for y in 0..map_height {
        for x in 0..map_width {
            let pos_x = (x as f32 - map_width as f32 / 2.0) * tile_size;
            let pos_y = (y as f32 - map_height as f32 / 2.0) * tile_size;

            commands.spawn(SpriteBundle {
                texture: background_texture.clone(),
                transform: Transform::from_xyz(pos_x, pos_y, -1.0), // z = -1 → behind map layer
                ..Default::default()
            });
        }
    }

    // === FOREGROUND TILEMAP LAYER ===
    let map_file = "assets/map_layout_road_demo.txt";
    let file_contents = fs::read_to_string(map_file)
        .expect("❌ Failed to read map_layout_road_demo.txt");

    let tile_map: Vec<Vec<u32>> = file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect();

    let _map_tile_height = tile_map.len();
    let _map_tile_width = tile_map.get(0).map_or(0, |row| row.len());

    for (y, row) in tile_map.iter().enumerate() {
        for (x, &tile_num) in row.iter().enumerate() {
            let tile_path = format!("Tiles/tile_{:04}.png", tile_num);
            let texture = asset_server.load(tile_path);

            // Get top-left corner of background
            let background_top_left_x = -(map_width as f32 / 2.0) * tile_size;
            let background_top_left_y = (map_height as f32 / 2.0 - 1.0) * tile_size;

            // Place foreground tiles starting from top-left going right/down
            let pos_x = background_top_left_x + x as f32 * tile_size;
            let pos_y = background_top_left_y - y as f32 * tile_size;

            commands.spawn(SpriteBundle {
                texture,
                transform: Transform::from_xyz(pos_x, pos_y, 0.0), // z = 0 → on top of background
                ..Default::default()
            });
        }
    }
}
