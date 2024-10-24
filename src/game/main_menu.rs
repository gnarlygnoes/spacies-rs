use macroquad::{
    color::WHITE,
    input::{is_key_pressed, KeyCode},
    text::draw_text,
};

use super::game_loop::{init_game, Game, GameState};

pub fn update_menu(g: &mut Game) {
    if is_key_pressed(KeyCode::Enter) {
        g.game_state = GameState::InGame;
    }
}

pub fn draw_menu() {
    draw_text("Hit enter to start the bloody game!", 50., 500., 36., WHITE);
}

pub fn game_paused() {
    draw_text("The bloody game's been paused!", 50., 500., 36., WHITE);
}
