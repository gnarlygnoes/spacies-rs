use macroquad::{
    color::{Color, GREEN, RED},
    math::Rect,
    shapes::draw_rectangle,
};

use super::game_loop::Game;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub rec: Rect,
    pub colour: Color,
    // pub id: u8,
    pub alive: bool,
}
impl Default for Enemy {
    fn default() -> Self {
        Self {
            rec: Rect {
                x: 0.,
                y: 0.,
                w: 40.,
                h: 60.,
            },
            colour: RED,
            // id: 0,
            alive: true,
        }
    }
}
impl Enemy {
    pub fn draw_enemy(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}
impl Game {
    // fn update_enemy_state(&mut self) {
    //     let mut dead: Vec<u8> = vec![];
    //     for e in &self.enemies {
    //         // for j in 0..enemies[i].len() - 1 {
    //         if !e.alive {
    //             dead.push(e.id);
    //         }
    //         // }
    //     }
    //     for val in dead {
    //         self.enemies.remove(&val);
    //     }
    // }
    pub fn update_enemies(&mut self) {
        // self.update_enemy_state();
        self.who_can_shoot();
        // println!("{}", self.enemies.len())
    }
    fn who_can_shoot(&mut self) {
        // for row in &self.enemies {
        //     println!("rows: {}, columns: {}", self.enemies.len(), row.len());
        // }
        // let r = self.enemies.len() - 2;
        // let c = self.enemies[r].len() - 0;
        // for _ in 0..r {
        //     for j in 0..c {
        //         for k in 0..r {
        //             if self.enemies[r - k][j].alive {
        //                 self.enemies[r - k][j].colour = GREEN;
        //                 // println!("{} {}", r - k, j);
        //                 break;
        //             }
        //             // println!("{k}");
        //         }
        //     }
        // }
        let r = self.enemies.len();
        let c = self.enemies[0].len();
        for _ in 0..r {
            for j in 0..c {
                for k in 1..r + 1 {
                    if self.enemies[r - k][j].alive {
                        self.enemies[r - k][j].colour = GREEN;
                        break;
                    }
                }
            }
        }
        println!("Arr rows: {}; Arr columns: {}", self.enemies.len(), c);
    }
    // fn who_can_shoot(&mut self) {
    //     let length = self.enemies.len();
    //     let mut can_shoot: Vec<u8> = vec![];
    //     for (_, e) in &self.enemies {
    //         if e.id >= length as u8 - 10 {
    //             // for (_, e2) in &self.enemies {}
    //             if !alien_in_front(&self.enemies, &e) {
    //                 can_shoot.push(e.id)
    //             } else {
    //             }
    //         }
    //     }
    //     for i in can_shoot.iter() {
    //         // println!("{i}");
    //         for (_, e) in &mut self.enemies {
    //             if i == &e.id {
    //                 e.colour = GREEN
    //             }
    //         }
    //     }
    // }
}
// fn alien_in_front(enemies: &HashMap<u8, Enemy>, e: &Enemy) -> bool {
//     // for (_, e) in self.enemies {
//     for (_, e2) in enemies {
//         if e.rec.x as i32 != e2.rec.x as i32 {
//             if e2.rec.y > e.rec.y {
//                 true;
//             }
//         }
//     }
//     false
//     // }
//     // return alien_in_front(enemies, e);
// }

pub fn create_enemies() -> [[Enemy; 10]; 5] {
    let w = 40.;
    let h = 60.;

    // let mut enemies = vec![vec![Enemy::default(); 10]; 5];
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
    // for i in 0..rows - 1 {
    //     enemies.push(vec![Enemy {
    //         rec: Rect {
    //             x: 10.,
    //             y: 10.,
    //             w: 40.,
    //             h: 60.,
    //         },
    //         colour: RED,
    //         alive: true,
    //     }]);
    //     for j in 0..columns - 1 {
    //         enemies[i].push(Enemy {
    //             rec: Rect {
    //                 x: (1.5 * j as f32 * w) + 10.,
    //                 y: (1.5 * i as f32 * h) + 10.,
    //                 w,
    //                 h,
    //             },
    //             ..Default::default()
    //         });
    //     }
    // }
    // for row in 0..enemies.len() {
    //     println!("{}", enemies.len());
    // }
    return enemies;
}

// pub fn create_enemies() -> HashMap<u8, Enemy> {
//     let w = 45.;
//     let h = 80.;
//     let mut enemies = HashMap::new();
//     for i in 0..50 {
//         enemies.insert(
//             i,
//             Enemy {
//                 rec: Rect {
//                     x: 1.5 * (i % 10) as f32 * w + 10.,
//                     y: 1.5 * (i / 10) as f32 * h + 10.,
//                     w,
//                     h,
//                 },
//                 id: i,
//                 ..Default::default()
//             },
//         );
//     }
//     return enemies;
// }

// pub fn update_enemies(mut enemies: HashMap<u8, Enemy>) {
// cleanup_dead();
// let mut dead: Vec<u8> = vec![];
// for (_, e) in &enemies {
//     // for j in 0..enemies[i].len() - 1 {
//     if !e.alive {
//         dead.push(e.id);
//     }
//     // }
// }
// for val in dead {
//     enemies.remove(&val);
// }
// }

// pub fn draw_enemes(enemies: &Vec<Vec<Enemy>>) {
//     for i in enemies {
//         for j in i {
//             j.draw_enemy();
//         }
//     }
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
    // println!("Enemies alive: {}", enemies.len());
}
