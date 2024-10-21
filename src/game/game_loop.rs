use std::collections::HashMap;

use macroquad::color::ORANGE;

use super::{
    collision::update_collision,
    enemies::{create_enemies, draw_enemes, Enemy},
    player::{create_player, update_player, Player},
};

pub struct Game {
    pub player: Player,
    // pub enemies: Vec<Vec<Enemy>>,
    pub enemies: HashMap<u8, Enemy>,
    // pub defences: HashMap(<u8, Defence>)
}
impl Game {
    pub fn update_enemies(&mut self) {
        let mut dead: Vec<u8> = vec![];
        for (_, e) in &self.enemies {
            // for j in 0..enemies[i].len() - 1 {
            if !e.alive {
                dead.push(e.id);
            }
            // }
        }
        for val in dead {
            self.enemies.remove(&val);
        }
    }
}

pub fn init_game() -> Game {
    Game {
        player: create_player(ORANGE),
        // enemies: create_enemies(),
        enemies: create_enemies(),
    }
}

pub fn update_game(g: &mut Game, dt: f32) {
    update_player(&mut g.player, dt);
    // update_enemies(g.enemies2);
    g.update_enemies();
    update_collision(g);
    // for b in &mut g.player.weapon.bullets {}
}

pub fn draw_game(g: &mut Game) {
    if g.player.weapon.muzzle_active {
        g.player.weapon.draw_muzzle();
    }
    g.player.draw();
    draw_enemes(&g.enemies);

    for (_, b) in &g.player.weapon.bullets {
        b.draw_bullet();
    }
}
