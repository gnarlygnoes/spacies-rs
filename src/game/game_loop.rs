use std::collections::HashMap;

use macroquad::{color::WHITE, text::draw_text};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    collision::update_collision,
    defence::{draw_defences, init_defences, Defence},
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{update_player, Player},
    weapon::Bullet,
};

pub struct Game {
    // pub game_texture: Texture2D,
    pub player: Player,
    pub enemies: [[Enemy; 10]; 5],
    // pub enemies2: HashMap<[[u8; 10]; 5], Enemy>,
    pub move_time: f32,
    pub game_speed: f32,
    pub enemy_direction: f32,
    pub enemy_drop_proc: bool,
    pub cur_shoot_time: f32,
    pub enemy_shoot_timer: f32,
    pub enemy_bullets: HashMap<u32, Bullet>,
    pub e_bullet_id: u32,
    pub defences: HashMap<u8, Defence>,

    pub player_score: i32,
    pub speed: u8,
}

pub fn init_game() -> Game {
    Game {
        // game_texture,
        player: Player::create_player(),
        enemies: create_enemies(),
        // enemies2: create_enemies2(),
        move_time: 0.,
        game_speed: 1.,
        enemy_direction: 1.,
        enemy_drop_proc: false,
        enemy_bullets: HashMap::new(),
        e_bullet_id: 0,
        cur_shoot_time: 0.,
        enemy_shoot_timer: 2.,
        defences: init_defences(),

        player_score: 0,
        speed: 1,
    }
}

// pub fn save_game() {}
// pub fn load_game() {}

fn check_speed(g: &mut Game) {
    match (g.player_score) {
        0..1000 => g.speed = 1,
        1000..2000 => g.speed = 2,
        2000..3000 => g.speed = 3,
        3000..4000 => g.speed = 4,
        4000..4500 => g.speed = 5,
        4500..4900 => g.speed = 6,
        _ => g.speed = 7,
    }

    match (g.speed) {
        1 => {
            g.game_speed = 1.5;
            g.enemy_shoot_timer = 2.
        }
        2 => {
            g.game_speed = 1.2;
            g.enemy_shoot_timer = 1.5
        }
        3 => {
            g.game_speed = 0.9;
            g.enemy_shoot_timer = 1.2
        }
        4 => {
            g.game_speed = 0.8;
            g.enemy_shoot_timer = 1.
        }
        5 => {
            g.game_speed = 0.6;
            g.enemy_shoot_timer = 0.8
        }
        6 => {
            g.game_speed = 0.5;
            g.enemy_shoot_timer = 0.8
        }
        7 => {
            g.game_speed = 0.2;
            g.enemy_shoot_timer = 0.6
        }
        _ => {
            g.game_speed = 0.1;
            g.enemy_shoot_timer = 0.5
        }
    }
}

pub fn set_player_score(g: &mut Game) -> i32 {
    let mut dead: i32 = 0;
    for row in g.enemies {
        for e in row {
            if !e.alive {
                dead += 1;
            }
        }
    }
    dead * 100
}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
    g.update_enemies(dt);
    update_collision(g);
    check_speed(g);
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
    draw_text(
        format!("Total score: {}", g.player_score).as_str(),
        20.,
        WINDOW_HEIGHT - 50.,
        36.,
        WHITE,
    );
    draw_text(
        format!("Speed: {}", g.speed).as_str(),
        WINDOW_WIDTH - 140.,
        WINDOW_HEIGHT - 50.,
        36.,
        WHITE,
    );
}
