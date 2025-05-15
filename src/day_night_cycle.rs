use bevy::prelude::*;

const SECONDS_PER_DAY: f32 = 60.0;
const MAX_ALPHA: f32 = 0.90;

#[derive(Component)]
pub struct NightLayer;

#[derive(Resource)]
pub struct GameTime {
    pub timer: Timer,
    pub elapsed: f32,
    pub last_minute: u32,
    pub last_alpha: f32,
}

pub struct DayNightCyclePlugin;

impl Plugin for DayNightCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameTime {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                elapsed: 0.0,
                last_minute: 0,
                last_alpha: 0.0,
            })
            .add_systems(Startup, spawn_night_layer)
            .add_systems(Update, update_day_night_cycle);
    }
}

fn spawn_night_layer(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("Sprites/nightLayer.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            ..default()
        },
        NightLayer,
    ));
}

fn update_day_night_cycle(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut query: Query<&mut Sprite, With<NightLayer>>,
) {
    game_time.elapsed += time.delta_seconds();

    let cycle_progress = (game_time.elapsed % SECONDS_PER_DAY) / SECONDS_PER_DAY;
    let in_game_minutes = (cycle_progress * 10.0) as u32;
    let hours = in_game_minutes / 60;
    let minutes = in_game_minutes % 60;

    if minutes != game_time.last_minute {
        game_time.last_minute = minutes;
        println!("In-game time: {:02}:{:02}", hours, minutes);
    }

    let alpha = if cycle_progress < 0.2 {
    // Night fading out (sunrise)
    1.0 - (cycle_progress / 0.2)
    } else if cycle_progress < 0.6 {
        // Daytime
        0.0
    } else if cycle_progress < 0.8 {
        // Night fading in (sunset)
        (cycle_progress - 0.6) / 0.2
    } else {
        // Full night
        1.0
    };

    let final_alpha = alpha * MAX_ALPHA;

    if let Ok(mut sprite) = query.get_single_mut() {
        sprite.color.set_a(final_alpha);

        if (final_alpha - game_time.last_alpha).abs() > 0.01 {
            println!("Alpha: {:.2}", final_alpha);
            game_time.last_alpha = final_alpha;
        }
    }
}
