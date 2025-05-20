use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::GameState;
use crate::tilemap::map_transition::Door;

#[derive(Component)]
pub struct SchoolInterior;

pub struct SchoolInteriorPlugin;

impl Plugin for SchoolInteriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InsideSchool), school_map);
        app.add_systems(OnExit(GameState::InsideSchool), despawn_school_map);
    }
}

pub fn school_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 16.0;
    let map_width = 10;
    let map_height = 5;

    let tile_x = 10;
    let tile_y = 13;

    let office_interior_pos_x = tile_x as f32 * tile_size;
    let office_interior_pos_y = tile_y as f32 * tile_size;

    let officef_texture = asset_server.load("Sprites/Interior/office_floor.png");
    let officedm_texture = asset_server.load("Sprites/Interior/office_mat.png");
    let empty_texture = asset_server.load("Sprites/Others/Empty.png");

    for y in 0..map_height {
        for x in 0..map_width {
            commands.spawn((
                SpriteBundle {
                    texture: officef_texture.clone(),
                    transform: Transform::from_xyz(office_interior_pos_x, office_interior_pos_y-2.0, -1.0),
                    ..Default::default()
                },
                SchoolInterior,
            ));
        }
    }

    commands.spawn((
        SpriteBundle {
            texture: officedm_texture.clone(),
            transform: Transform::from_xyz(office_interior_pos_x-64.0, office_interior_pos_y-34.0, 0.0),
            ..Default::default()
        },
        Door {
            target_state: GameState::Outside,
        },
        SchoolInterior,
    ));

    //World bounds
    commands.spawn((
        SpriteBundle { //TOP
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(office_interior_pos_x, office_interior_pos_y+49.0, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 0.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        SchoolInterior,
    ));
    commands.spawn((
        SpriteBundle { //RIGHT
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(office_interior_pos_x+106.0, office_interior_pos_y, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        SchoolInterior,
    ));
    commands.spawn((
        SpriteBundle { //BOTTOM
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(office_interior_pos_x, office_interior_pos_y-50.0, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(100.0, 0.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        SchoolInterior,
    ));
    commands.spawn((
        SpriteBundle { //LEFT
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(office_interior_pos_x-106.0, office_interior_pos_y, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        SchoolInterior,
    ));
}

fn despawn_school_map(
    mut commands: Commands,
    query: Query<Entity, With<SchoolInterior>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
