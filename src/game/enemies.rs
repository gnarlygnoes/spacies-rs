use ::rand::Rng;

use macroquad::{
    color::{Color, GREEN, RED},
    math::{Circle, Rect},
    shapes::draw_rectangle,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    game_loop::{set_player_score, Game},
    weapon::{delete_bullets, Bullet},
};

#[derive(Clone, Copy)]
pub struct Enemy {
    pub rec: Rect,
    pub colour: Color,
    pub alive: bool,
    can_shoot: bool,
    pub health: u8,
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
            health: 1,
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
        self.move_time += dt;
        self.cur_shoot_time += dt;

        if self.move_time > self.game_speed * 0.8 {
            self.move_enemies();

            self.move_time = 0.
        }
        if self.cur_shoot_time > self.enemy_shoot_timer * 0.9 {
            self.shoot();

            self.cur_shoot_time = 0.;

            self.enemy_shoot_timer = ::rand::thread_rng().gen_range(1.5..3.0)
        }

        for i in 0..self.enemies.len() {
            for j in 0..self.enemies[i].len() {
                if self.enemies[i][j].health <= 0 {
                    self.enemies[i][j].alive = false;
                    self.player_score = set_player_score(self);
                }
            }
        }

        self.update_enemy_bullets(dt);
    }

    fn move_enemies(&mut self) {
        for row in &mut self.enemies {
            for e in row {
                e.rec.x += 40. * self.enemy_direction;
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
                        // self.enemies[r - k][j].colour = GREEN;
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

    fn shoot(&mut self) {
        let num_can_shoot: usize = 10;
        let mut rng = ::rand::thread_rng();

        // for row in self.enemies {
        //     for e in row {
        //         if e.can_shoot {
        //             num_can_shoot += 1;
        //         }
        //     }
        // }
        // println!("{num_can_shoot}");
        let mut random_number: usize = rng.gen_range(0..num_can_shoot);
        if self.player_score < 5000 {
            while !self.enemies[0][random_number].alive {
                random_number = rng.gen_range(0..num_can_shoot);
            }
        }
        for i in 0..self.enemies.len() {
            if self.enemies[i][random_number].can_shoot {
                // self.enemies[i][random_number].bullet =
                self.enemy_bullets.insert(
                    self.e_bullet_id,
                    Bullet {
                        colour: GREEN,
                        circle: Circle {
                            x: self.enemies[i][random_number].rec.x
                                + self.enemies[i][random_number].rec.w / 2.
                                - 2.5,
                            y: self.enemies[i][random_number].rec.y
                                + self.enemies[i][random_number].rec.h,
                            r: 5.,
                        },
                        active: true,
                    },
                );
                self.e_bullet_id += 1;
            }
        }
    }
    fn update_enemy_bullets(&mut self, dt: f32) {
        for (_, b) in &mut self.enemy_bullets {
            if b.active {
                b.circle.y += 1500. * dt;
                if b.circle.y > WINDOW_HEIGHT {
                    b.active = false;
                }
            }
        }
        let mut inactive: Vec<u32> = vec![];
        for (id, b) in &self.enemy_bullets {
            if !b.active {
                inactive.push(*id);
            }
        }
        self.enemy_bullets = delete_bullets(self.enemy_bullets.clone(), inactive);
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
            };
            if i < 3 {
                enemies[i][j].health = 2
            }
            if i < 1 {
                enemies[i][j].health = 3
            }
            // enemies[i][j].health = 5 - i as u8;
        }
    }
    return enemies;
}

// pub fn create_enemies2() -> HashMap<[[u8; 10]; 5], Enemy> {
//     let w = 50.;
//     let h = 50.;

//     let mut iterator = [[0u8; 10]; 5];
//     // let mut i: usize = 0;
//     let mut enemies: HashMap<[[u8; 10]; 5], Enemy> = HashMap::new();
//     for i in 0..iterator.len() {
//         for j in 0..iterator[i].len() {
//             iterator[i][j] = j as u8;

//             // println!("{} {}", i, j);
//             // println!("{}", iterator[2][3]);
//         }
//     }
//     enemies.insert(
//         iterator,
//         Enemy::default(),
//         // {
//         // rec: Rect {
//         //     x: (1.5 * j as f32 * w) + 10.,
//         //     y: (1.5 * i as f32 * h) + 10.,
//         //     w,
//         //     h,
//         // },
//         // alive: true,
//         //     ..Default::default()
//         // },
//     );
//     for i in 0..iterator.len() {
//         for j in 0..iterator[i].len() {
//             for (id, e) in &mut enemies {
//                 e.alive = true;
//                 e.rec.x = (1.5 * j as f32 * w) + 10.;
//                 e.rec.y = (1.5 * i as f32 * h) + 10.;
//             }
//         }
//     }

//     enemies
// }

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
