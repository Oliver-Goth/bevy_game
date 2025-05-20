use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::stamina::Stamina;

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
pub struct CharacterAnimation {
    pub direction: Direction,
    pub moving: bool,
    pub frame: usize,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("Sprites/Character/character1.png");

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 3, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas { layout: layout_handle, index: 2 },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),

            ..default()
        },
        Player,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        CharacterAnimation {
            direction: Direction::Down,
            moving: false,
            frame: 0,
        },
        
        RigidBody::Dynamic,
        Collider::cuboid(8.0, 8.0),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),

        Stamina::new(500.0),
    ));
}