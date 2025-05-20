use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::GameState;
use crate::tilemap::map_transition::Door;

#[derive(Component)]
pub struct House1Interior;

pub struct House1InteriorPlugin;

impl Plugin for House1InteriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InsideHouse1), house1_map);
        app.add_systems(OnExit(GameState::InsideHouse1), despawn_house1_map);
    }
}

pub fn house1_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 16.0;
    let map_width = 5;
    let map_height = 4;

    let tile_x = -5;
    let tile_y = 5;

    let house1_interior_pos_x = tile_x as f32 * tile_size;
    let house1_interior_pos_y = tile_y as f32 * tile_size;

    let house1dm_texture = asset_server.load("Sprites/Interior/door_mat.png");
    let house1f_texture = asset_server.load("Sprites/Interior/floor.png");
    let empty_texture = asset_server.load("Sprites/Others/Empty.png");

    for y in 0..map_height {
        for x in 0..map_width {
            commands.spawn((
                SpriteBundle {
                    texture: house1f_texture.clone(),
                    transform: Transform::from_xyz(house1_interior_pos_x, house1_interior_pos_y, -1.0),
                    ..Default::default()
                },
                House1Interior,
            ));
        }
    }

    commands.spawn((
        SpriteBundle {
            texture: house1dm_texture.clone(),
            transform: Transform::from_xyz(house1_interior_pos_x, house1_interior_pos_y-24.0, 0.0),
            ..Default::default()
        },
        Door {
            target_state: GameState::Outside,
        },
        House1Interior,
    ));

    //World bounds
    commands.spawn((
        SpriteBundle { //TOP
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(house1_interior_pos_x, house1_interior_pos_y+33.0, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(50.0, 0.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        House1Interior,
    ));
    commands.spawn((
        SpriteBundle { //RIGHT
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(house1_interior_pos_x+42.0, house1_interior_pos_y, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.0, 43.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        House1Interior,
    ));
    commands.spawn((
        SpriteBundle { //BOTTOM
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(house1_interior_pos_x, house1_interior_pos_y-32.0, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(43.0, 0.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        House1Interior,
    ));
    commands.spawn((
        SpriteBundle { //LEFT
            texture: empty_texture.clone(),
            transform: Transform::from_xyz(house1_interior_pos_x-42.0, house1_interior_pos_y, 0.0),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        KinematicCharacterController::default(),
        House1Interior,
    ));
}

fn despawn_house1_map(
    mut commands: Commands,
    query: Query<Entity, With<House1Interior>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
