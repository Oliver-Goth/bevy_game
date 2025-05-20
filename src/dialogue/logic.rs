use bevy::prelude::*;
use serde::Deserialize;
use ron::de::from_str;
use crate::dialogue::ui::DialogueBox;
use crate::dialogue::ui::spawn_dialogue_box;

#[derive(Resource, Default)]
pub struct DialogueState {
    pub active: bool,
    pub current_line: usize,
    pub lines: Vec<DialogueLine>,
}

#[derive(Deserialize, Clone)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
}

impl DialogueState {
    pub fn from_ron_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("❌ Failed to read .ron file at {}", path);
            String::new()
        });

        let lines: Vec<DialogueLine> = from_str(&content).unwrap_or_else(|err| {
            eprintln!("❌ Failed to parse .ron: {}", err);
            vec![]
        });

        DialogueState {
            active: true,
            current_line: 0,
            lines,
        }
    }

    pub fn load_file(&mut self, path: &str) {
        let content = std::fs::read_to_string(format!("assets/{}", path)).unwrap_or_else(|_| {
            eprintln!("❌ Failed to load .ron dialogue: {}", path);
            String::new()
        });

        let lines: Vec<DialogueLine> = from_str(&content).unwrap_or_else(|err| {
            eprintln!("❌ RON parse error: {}", err);
            vec![]
        });

        self.lines = lines;
        self.active = true;
        self.current_line = 0;
    }
}

pub fn handle_dialogue_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut dialogue: ResMut<DialogueState>,
    mut commands: Commands,
    box_query: Query<Entity, With<DialogueBox>>,
) {
    if dialogue.active && keys.just_pressed(KeyCode::Space) {
        dialogue.current_line += 1;
        if dialogue.current_line >= dialogue.lines.len() {
            dialogue.active = false;

            // 🟡 Despawn the UI
            if let Ok(entity) = box_query.get_single() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Call this when interacting with an NPC
pub fn trigger_npc_dialogue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut dialogue: ResMut<DialogueState>,
) {
    dialogue.load_file("dialogue/npcs/nora.ron");
    spawn_dialogue_box(&mut commands, &asset_server);
}