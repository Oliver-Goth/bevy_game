use bevy::prelude::*;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_texture = asset_server.load("Landscape/landscapeTiles_067.png");
    let tile_size = Vec2::new(64.0, 32.0);
    let width = 10;
    let height = 10;

    for y in 0..height {
        for x in 0..width {
            let world_x = (x as f32 - y as f32) * (tile_size.x / 2.0);
            let world_y = (x as f32 + y as f32) * (tile_size.y / 2.0);

            commands.spawn(SpriteBundle {
                texture: tile_texture.clone(),
                transform: Transform::from_xyz(world_x, world_y, 0.0),
                ..Default::default()
            });
        }
    }

    commands.spawn(Camera2dBundle::default());
}
