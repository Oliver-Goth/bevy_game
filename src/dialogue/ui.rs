// src/dialogue/ui.rs
use bevy::prelude::*;
use crate::dialogue::logic::DialogueState;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct SpeakerText;

#[derive(Component)]
pub struct DialogueBox;


pub fn update_dialogue_ui(
    dialogue: Res<DialogueState>,
    mut speaker_query: Query<&mut Text, With<SpeakerText>>,
    mut line_query: Query<&mut Text, (With<DialogueText>, Without<SpeakerText>)>,
) {
    if !dialogue.active {
        return;
    }

    if let Some(line) = dialogue.lines.get(dialogue.current_line) {
        if let Ok(mut speaker_text) = speaker_query.get_single_mut() {
            speaker_text.sections[0].value = format!("{}:", line.speaker);
        }

        if let Ok(mut dialogue_text) = line_query.get_single_mut() {
            dialogue_text.sections[0].value = line.text.clone();
        }
    }
}

pub fn spawn_dialogue_box(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
    .spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(120.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        },
        DialogueBox,
    ))
    // Speaker Name
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("Fonts/pokemon-ds.ttf"),
                    font_size: 18.0,
                    color: Color::GOLD,
                },
            ),
            SpeakerText,
        ));
        // Dialogue Line
        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("Fonts/pokemon-ds.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ),
            DialogueText,
        ));
    });
}