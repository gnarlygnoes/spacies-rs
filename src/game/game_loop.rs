use macroquad::color::ORANGE;

use super::{
    collision::update_collision,
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{create_player, update_player, Player},
};

pub struct Game {
    pub player: Player,
    pub enemies: [[Enemy; 10]; 5],
    pub event_time: f32,
    pub game_speed: f32,
    pub enemy_direction: f32,
    pub enemy_drop_proc: bool,
}

pub fn init_game() -> Game {
    Game {
        player: create_player(ORANGE),
        enemies: create_enemies(),
        event_time: 0.,
        game_speed: 1.,
        enemy_direction: 1.,
        enemy_drop_proc: false,
    }
}

// pub fn save_game() {}
// pub fn load_game() {}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
    g.update_enemies(dt);
    update_collision(g);
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
