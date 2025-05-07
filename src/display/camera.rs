use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::keyboard::KeyCode;
use crate::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 998.0),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.get_single(), camera_query.get_single_mut())
    {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut projection) = query.get_single_mut() {
        let mut zoom_delta = 0.0;

        for ev in scroll_evr.read() {
            zoom_delta -= ev.y * 0.1;
        }

        if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
            if keyboard_input.just_pressed(KeyCode::Equal) {
                zoom_delta -= 0.1;
            }
            if keyboard_input.just_pressed(KeyCode::Minus) {
                zoom_delta += 0.1;
            }
        }

        let new_scale = (projection.scale + zoom_delta).clamp(0.1, 5.0);
        projection.scale = new_scale;
    }
}
