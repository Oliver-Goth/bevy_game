use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use crate::player::Player;
use crate::stamina::Stamina;

/// This layer is rendered by the second UI camera.
pub const STAMINA_UI_LAYER: u8 = 1;

#[derive(Component)]
pub struct StaminaBar;

pub fn spawn_stamina_bar_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Sprites/UI/StaminaBar.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(7.0, 32.0), 25, 1, None, None);
    let layout_handle = texture_atlases.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
            transform: Transform {
                translation: Vec3::new(10.0, -130.0, 0.0), // on-screen position
                scale: Vec3::splat(3.0), // scale UI up
                ..default()
            },
            ..default()
        },
        StaminaBar,
        RenderLayers::layer(STAMINA_UI_LAYER), // ensure this is rendered only by the UI camera
    ));
}

pub fn update_stamina_bar(
    stamina_query: Query<&Stamina, With<Player>>,
    mut bar_query: Query<&mut TextureAtlas, With<StaminaBar>>,
) {
    let Ok(stamina) = stamina_query.get_single() else { return };
    let Ok(mut atlas) = bar_query.get_single_mut() else { return };

    let ratio = stamina.current / stamina.max;
    let frame = ((1.0 - ratio) * 24.0).round().clamp(0.0, 24.0) as usize;
    atlas.index = frame;
}