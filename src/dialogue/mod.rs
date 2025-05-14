// src/dialogue/mod.rs
pub mod logic;
pub mod ui;

use bevy::prelude::*;
use logic::*;
use ui::*;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DialogueState::from_ron_file("assets/dialogue/dialogue.ron"))
            .add_systems(Startup, |mut commands: Commands, asset_server: Res<AssetServer>| {
                spawn_dialogue_box(&mut commands, &asset_server);
            })
            .add_systems(Update, handle_dialogue_input)
            .add_systems(Update, update_dialogue_ui);
    }
}
