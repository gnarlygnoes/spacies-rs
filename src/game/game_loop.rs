use macroquad::color::ORANGE;

use super::{
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{create_player, update_player, Player},
};

pub struct Game {
    pub player: Player,
    enemies: Vec<Vec<Enemy>>,
}

pub fn init_game() -> Game {
    Game {
        player: create_player(ORANGE),
        enemies: create_enemies(),
    }
}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
}

pub fn draw_game(g: &mut Game) {
    if g.player.weapon.muzzle_active {
        g.player.weapon.draw_muzzle();
    }
    g.player.draw();
    draw_enemes(&g.enemies);

    for b in &g.player.weapon.bullets {
        b.draw_bullet();
    }
}
