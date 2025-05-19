use bevy::prelude::*;
use crate::player::{CharacterAnimation, Player};
use crate::game_state::AppState;

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
}

#[derive(Resource)]
pub struct StaminaPrintTimer(pub Timer);

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).clamp(0.0, self.max);
    }

    pub fn drain(&mut self, rate: f32, delta: f32) {
        self.current = (self.current - rate * delta).clamp(0.0, self.max);
    }
}

pub fn stamina_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut print_timer: ResMut<StaminaPrintTimer>,
    mut query: Query<(&mut Stamina, &CharacterAnimation), With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    print_timer.0.tick(time.delta());

    for (mut stamina, anim) in query.iter_mut() {
        let delta = time.delta_seconds();
        let drain_rate = if anim.moving { 50.0 } else { 0.1 };

        stamina.drain(drain_rate, delta);

        if keyboard_input.just_pressed(KeyCode::KeyU) {
            stamina.restore(250.0);
            println!("Drank coffee! Stamina: {:.1}/{}", stamina.current, stamina.max);
        }

        if print_timer.0.finished() {
            println!("Stamina: {:.1}/{}", stamina.current, stamina.max);
        }

        // Trigger Game Over
        if stamina.current <= 0.0 {
            println!("🟥 Game Over: stamina depleted!");
            next_state.set(AppState::GameOver);
        }
    }
}
