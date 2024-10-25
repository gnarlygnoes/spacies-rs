use macroquad::{
    color::{GREEN, RED, WHITE},
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
    draw_text("You won! Well done!", 50., 500., 36., GREEN);
    draw_text(
        "Spend your hard-earned points to unlock new stuff, or to repair your shooter.",
        50.,
        550.,
        36.,
        GREEN,
    );
    draw_text(
        "Or hit enter to go to the next round.",
        50.,
        600.,
        36.,
        GREEN,
    );
    draw_text(
        format!("Final Score: {}", g.player_score).as_str(),
        50.,
        700.,
        36.,
        GREEN,
    );
}

pub fn defeat_mode(g: &mut Game) {
    if is_key_pressed(KeyCode::Enter) {
        *g = init_game();
        g.game_state = GameState::InGame;
    }
}

pub fn draw_defeat() {
    draw_text("You have been defecated.", 50., 500., 48., RED);
    draw_text("Your planet has been over-run,", 50., 550., 36., RED);
    draw_text(
        "but at least you are not alive to experience",
        50.,
        600.,
        36.,
        RED,
    );
    draw_text(
        "the suffering that your defeat has caused others",
        50.,
        650.,
        36.,
        RED,
    );
}
