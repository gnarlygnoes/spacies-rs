use macroquad::{
    color::{Color, GREEN, RED},
    math::Rect,
    shapes::draw_rectangle,
};

use crate::WINDOW_WIDTH;

use super::game_loop::Game;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub rec: Rect,
    pub colour: Color,
    pub alive: bool,
    can_shoot: bool,
}
impl Default for Enemy {
    fn default() -> Self {
        Self {
            rec: Rect {
                x: 0.,
                y: 0.,
                w: 50.,
                h: 50.,
            },
            colour: RED,
            alive: true,
            can_shoot: false,
        }
    }
}
impl Enemy {
    pub fn draw_enemy(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}
impl Game {
    pub fn update_enemies(&mut self, dt: f32) {
        self.who_can_shoot();

        self.event_time += dt;

        if self.event_time > self.game_speed {
            self.move_enemies();

            self.event_time = 0.
        }
    }

    fn move_enemies(&mut self) {
        for row in &mut self.enemies {
            for e in row {
                // if !self.enemy_drop_proc {
                e.rec.x += 40. * self.enemy_direction;
                // }
                if self.enemy_drop_proc {
                    e.rec.y += 40.;
                }
            }
        }
        self.enemy_drop_proc = false;
        for i in (0..self.enemies.len()).rev() {
            for j in (0..self.enemies[i].len()).rev() {
                if self.enemies[i][j].alive {
                    if self.enemies[i][j].rec.x + self.enemies[i][j].rec.w > WINDOW_WIDTH - 50. {
                        self.enemy_direction = -1.;
                        self.enemy_drop_proc = true
                    }
                }
            }
        }
        for i in 0..self.enemies.len() {
            for j in 0..self.enemies[i].len() {
                if self.enemies[i][j].alive {
                    if self.enemies[i][j].rec.x < 50. {
                        self.enemy_direction = 1.;
                        self.enemy_drop_proc = true
                    }
                }
            }
        }
    }

    fn who_can_shoot(&mut self) {
        let r = self.enemies.len();
        let c = self.enemies[0].len();
        for _ in 0..r {
            for j in 0..c {
                for k in 1..r + 1 {
                    if self.enemies[r - k][j].alive {
                        self.enemies[r - k][j].colour = GREEN;
                        self.enemies[r - k][j].can_shoot = true;
                        break;
                    }
                    if !self.enemies[r - k][j].alive {
                        self.enemies[r - k][j].can_shoot = false;
                    }
                }
            }
        }
    }
}

pub fn create_enemies() -> [[Enemy; 10]; 5] {
    let w = 50.;
    let h = 50.;

    let mut enemies = [[Enemy::default(); 10]; 5];

    for i in 0..enemies.len() {
        for j in 0..enemies[i].len() {
            enemies[i][j].rec = Rect {
                x: (1.5 * j as f32 * w) + 10.,
                y: (1.5 * i as f32 * h) + 10.,
                w,
                h,
            }
        }
    }
    return enemies;
}

pub fn draw_enemes(enemies: [[Enemy; 10]; 5]) {
    for row in enemies {
        for e in row {
            // for j in i {
            if e.alive {
                e.draw_enemy();
            }
        }
    }
}
