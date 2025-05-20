use bevy::prelude::*;
use crate::game_state::AppState;

#[derive(Component)]
pub struct GameOverUI;

pub fn spawn_game_over_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        },
        GameOverUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: asset_server.load("Fonts/pokemon-ds.ttf"),
                    font_size: 48.0,
                    color: Color::RED,
                },
            ),
        ));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0), // Bevy 0.13 way
                    height: Val::Px(50.0), // Bevy 0.13 way
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            RestartButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font: asset_server.load("Fonts/pokemon-ds.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

#[derive(Component)]
pub struct RestartButton;

pub fn handle_restart_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("🔁 Restart button clicked!");
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *bg = Color::GRAY.into();
            }
            Interaction::None => {
                *bg = Color::DARK_GRAY.into();
            }
        }
    }
}

pub fn despawn_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    println!("🧹 Despawning Game Over UI...");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
