use std::collections::HashMap;

use macroquad::{
    color::{Color, RED},
    math::Rect,
    shapes::draw_rectangle,
};

pub struct Enemy {
    pub rec: Rect,
    pub colour: Color,
    pub id: u8,
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
            id: 0,
            alive: true,
        }
    }
}
impl Enemy {
    pub fn draw_enemy(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}

// pub fn create_enemies() -> Vec<Vec<Enemy>> {
//     let w = 40.;
//     let h = 60.;
//     let mut enemies = vec![vec![]];
//     for i in 0..5 {
//         enemies.push(vec![Enemy {
//             rec: Rect {
//                 x: 10.,
//                 y: 10.,
//                 w: 40.,
//                 h: 60.,
//             },
//             colour: RED,
//             alive: true,
//         }]);
//         for j in 0..10 {
//             enemies[i].push(Enemy {
//                 rec: Rect {
//                     x: (1.5 * j as f32 * w) + 10.,
//                     y: (1.5 * i as f32 * h) + 10.,
//                     w,
//                     h,
//                 },
//                 ..Default::default()
//             });
//         }
//     }
//     // println!("{}", enemies[0].len());
//     return enemies;
// }

pub fn create_enemies() -> HashMap<u8, Enemy> {
    let w = 45.;
    let h = 80.;
    let mut enemies = HashMap::new();
    for i in 0..50 {
        enemies.insert(
            i,
            Enemy {
                rec: Rect {
                    x: 1.5 * (i % 10) as f32 * w + 10.,
                    y: 1.5 * (i / 10) as f32 * h + 10.,
                    w,
                    h,
                },
                id: i,
                ..Default::default()
            },
        );
    }
    return enemies;
}

// pub fn update_enemies(mut enemies: HashMap<u8, Enemy>) {
//     // cleanup_dead();
//     let mut dead: Vec<u8> = vec![];
//     for (_, e) in &enemies {
//         // for j in 0..enemies[i].len() - 1 {
//         if !e.alive {
//             dead.push(e.id);
//         }
//         // }
//     }
//     for val in dead {
//         enemies.remove(&val);
//     }
// }

// pub fn draw_enemes(enemies: &Vec<Vec<Enemy>>) {
//     for i in enemies {
//         for j in i {
//             j.draw_enemy();
//         }
//     }
// }

pub fn draw_enemes(enemies: &HashMap<u8, Enemy>) {
    for (_, e) in enemies {
        // for j in i {
        if e.alive {
            e.draw_enemy();
        }
        // }
    }
    println!("Enemies alive: {}", enemies.len());
}
