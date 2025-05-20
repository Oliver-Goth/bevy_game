use bevy::prelude::*;
use std::fs;
use bevy_rapier2d::prelude::*;

use crate::states::GameState;
use crate::tilemap::map_transition::Door;

#[derive(Component)]
pub struct MainMap;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Outside), spawn_map);
        app.add_systems(OnExit(GameState::Outside), despawn_map);
    }
}

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 16.0;
    let map_width = 50;
    let map_height = 36;

    let file_contents = fs::read_to_string("assets/map_layout_road_demo.txt")
        .expect("❌ Failed to read map_layout_road_demo.txt");

    let tile_map: Vec<Vec<u32>> = file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect();

    let top_left_x = -(map_width as f32 / 2.0) * tile_size;
    let top_left_y = (map_height as f32 / 2.0 - 1.0) * tile_size;

    for (y, row) in tile_map.iter().enumerate() {
        for (x, &tile_num) in row.iter().enumerate() {
            let texture = asset_server.load(format!("Tiles/tile_{:04}.png", tile_num));
            let pos = Vec3::new(
                top_left_x + x as f32 * tile_size,
                top_left_y - y as f32 * tile_size,
                0.0,
            );

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                },
                MainMap,
            ));
        }
    }
  
    // House 1
    let house1_pos_x = -5 as f32 * tile_size;
    let house1_pos_y = 5 as f32 * tile_size;

    let house1_texture = asset_server.load("Sprites/Buildings/House1.png");
    let house1g_texture = asset_server.load("Sprites/Buildings/House1g.png");
    let house1d_texture = asset_server.load("Sprites/Buildings/House1d.png");
    
    commands.spawn((
        SpriteBundle {
            texture: house1_texture,
            transform: Transform::from_xyz(house1_pos_x, house1_pos_y, 2.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(37.0, 24.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        MainMap,
    ));
    commands.spawn((
        SpriteBundle {
            texture: house1g_texture,
            transform: Transform::from_xyz(house1_pos_x, house1_pos_y, 0.0),
            ..Default::default()
        },
        MainMap,
    ));
    commands.spawn((
        SpriteBundle {
            texture: house1d_texture,
            transform: Transform::from_xyz(house1_pos_x, house1_pos_y-32.0, 0.0),
            ..Default::default()
        },
        Door {
            target_state: GameState::InsideHouse1,
        },
        MainMap,
    ));
    
    // School
    let school_pos_x = 9 as f32 * tile_size;
    let school_pos_y = 9 as f32 * tile_size;

    let school_texture = asset_server.load("Sprites/Buildings/School.png");
    let schools_texture = asset_server.load("Sprites/Buildings/Schools.png");
    let schoolg_texture = asset_server.load("Sprites/Buildings/Schoolg.png");
    let schoold_texture = asset_server.load("Sprites/Buildings/Schoold.png");

    let empty_texture = asset_server.load("Sprites/Others/Empty.png");
    
    commands.spawn((
        SpriteBundle {
            texture: school_texture,
            transform: Transform::from_xyz(school_pos_x, school_pos_y, 2.0),
            ..Default::default()
        },
        MainMap,
    ));

    commands.spawn((
        SpriteBundle {
            texture: schools_texture,
            transform: Transform::from_xyz(school_pos_x, school_pos_y, 0.0),
            ..Default::default()
        },
        MainMap,
    ));

    commands.spawn((
        SpriteBundle {
            texture: schoolg_texture,
            transform: Transform::from_xyz(school_pos_x, school_pos_y, 0.0),
            ..Default::default()
        },
        MainMap,
    ));

    commands.spawn((
        SpriteBundle {
            texture: schoold_texture,
            transform: Transform::from_xyz(school_pos_x-48.0, school_pos_y+28.0, 0.0),
            ..Default::default()
        },
        Door {
            target_state: GameState::InsideSchool,
        },
        MainMap,
    ));

    // Borders
    commands.spawn((
        SpriteBundle { //MAIN
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(school_pos_x+48.0, school_pos_y-24.0, 2.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(54.0, 52.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        MainMap,
    ));
    commands.spawn((
        SpriteBundle { //TOP
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(school_pos_x, school_pos_y+65.0, 2.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(102.0, 37.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        MainMap,
    ));
    commands.spawn((
        SpriteBundle { //PILLAR
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(school_pos_x-97.0, school_pos_y-24.0, 2.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5.0, 52.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        MainMap,
    ));
}

pub fn despawn_map(
    mut commands: Commands,
    query: Query<Entity, With<MainMap>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
