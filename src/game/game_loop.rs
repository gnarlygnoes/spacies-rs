use std::collections::HashMap;

use super::{
    collision::update_collision,
    defence::{draw_defences, init_defences, Defence},
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{create_player, update_player, Player},
    weapon::Bullet,
};

pub struct Game {
    // pub game_texture: Texture2D,
    pub player: Player,

    pub enemies: [[Enemy; 10]; 5],
    pub move_time: f32,
    pub game_speed: f32,
    pub enemy_direction: f32,
    pub enemy_drop_proc: bool,
    pub cur_shoot_time: f32,
    pub enemy_shoot_timer: f32,
    pub enemy_bullets: HashMap<u32, Bullet>,
    pub e_bullet_id: u32,
    pub defences: HashMap<u8, Defence>,
}

pub fn init_game() -> Game {
    Game {
        // game_texture,
        player: create_player(),
        enemies: create_enemies(),
        move_time: 0.,
        game_speed: 1.,
        enemy_direction: 1.,
        enemy_drop_proc: false,
        enemy_bullets: HashMap::new(),
        e_bullet_id: 0,
        cur_shoot_time: 0.,
        enemy_shoot_timer: 2.,
        defences: init_defences(),
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
    draw_defences(&g.defences);

    for (_, b) in &g.player.weapon.bullets {
        b.draw_bullet();
    }
    for (_, b) in &g.enemy_bullets {
        b.draw_bullet();
    }
}
