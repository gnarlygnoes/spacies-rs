use macroquad::{
    color::WHITE,
    input::{is_key_pressed, KeyCode},
    text::draw_text,
};

use super::game_loop::{init_game, Game, GameState};

pub fn victory_mode(g: &mut Game) {
    if is_key_pressed(KeyCode::Enter) {
        *g = init_game();
        g.game_state = GameState::InGame;
    }
}

pub fn draw_victory(g: &Game) {
    draw_text(
        "You won! Well done! \n
        Spend your hard-earned points to unlock new stuff, or to repair your shooter.\n
        Or hit enter to go to the next round.",
        50.,
        500.,
        36.,
        WHITE,
    );
    draw_text(
        format!("Final Score: {}", g.player_score).as_str(),
        50.,
        700.,
        36.,
        WHITE,
    );
}

pub fn defeat_mode(g: &mut Game) {
    if is_key_pressed(KeyCode::Enter) {
        *g = init_game();
        g.game_state = GameState::InGame;
    }
}

pub fn draw_defeat() {
    draw_text("You have been defecated.", 50., 500., 48., WHITE);
    draw_text(
        "Your planet has been over-run,
        but at least you are not alive to experience
        the suffering that your defeat has caused others",
        50.,
        600.,
        36.,
        WHITE,
    );
}
