use macroquad::color::ORANGE;

use super::player::{create_player, update_player, Player};

pub struct Game {
    pub player: Player,
    // enemies: Vec<Enemy>,
}

pub fn init_game() -> Game {
    // player = create_player(ORANGE);
    let g = Game {
        player: create_player(ORANGE),
    };
    // let mut player: Player = create_player(ORANGE);

    return g;
}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
}

pub fn draw_game(g: &mut Game) {
    g.player.draw();
}
