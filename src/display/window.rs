use bevy::prelude::*;
use bevy::window::WindowResolution;

/// This plugin sets up the window with a fixed resolution and title.
pub struct WindowConfigPlugin;

impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Tile Game".into(),
                resolution: WindowResolution::new(960.0, 576.0), //W=640 H=400
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())); // Makes pixels sharp
    }
}