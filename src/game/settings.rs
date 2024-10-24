use macroquad::input::{is_key_pressed, KeyCode};

use super::game_loop::{Game, GameState};

// pub enum Controls {
//     Pause(KeyCode),
// }

pub fn handle_inputs(g: &mut Game) {
    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Pause) {
        match g.game_state {
            GameState::Paused => g.game_state = GameState::InGame,
            GameState::InGame => g.game_state = GameState::Paused,
            _ => {}
        }
    }
}
