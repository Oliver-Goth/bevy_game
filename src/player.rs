use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("Tiles/tile_0105.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 9, 5, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas { layout: layout_handle, index: 0 },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
    ));
}
