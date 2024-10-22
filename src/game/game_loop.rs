use macroquad::color::ORANGE;

use super::{
    collision::update_collision,
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{create_player, update_player, Player},
};

pub struct Game {
    pub player: Player,
    // pub enemies: Vec<Vec<Enemy>>,
    pub enemies: [[Enemy; 10]; 5], // pub enemies: HashMap<u8, Enemy>,
}

pub fn init_game() -> Game {
    Game {
        player: create_player(ORANGE),
        // enemies: create_enemies(),
        enemies: create_enemies(),
    }
}

// pub fn save_game() {}
// pub fn load_game() {}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
    g.update_enemies();
    // g.update_enemy_state();
    update_collision(g);
    // for b in &mut g.player.weapon.bullets {}
}

pub fn draw_game(g: &mut Game) {
    if g.player.weapon.muzzle_active {
        g.player.weapon.draw_muzzle();
    }
    g.player.draw();
    draw_enemes(g.enemies);

    for (_, b) in &g.player.weapon.bullets {
        b.draw_bullet();
    }
}
