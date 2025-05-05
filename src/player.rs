use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub direction: Direction,
    pub moving: bool,
    pub frame: usize,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("Tilemap/tilemap_packed.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 27, 18, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas { layout: layout_handle, index: 24 },
            //transform: Transform::from_xyz(0.0, 0.0, 1.0),
            
            //midlertidig zoom
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },


            ..default()
        },
        Player,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        PlayerAnimation {
            direction: Direction::Down,
            moving: false,
            frame: 0,
        },
    ));
}
