use std::collections::HashMap;

use macroquad::{
    color::{Color, GRAY, YELLOW},
    input::{is_key_pressed, KeyCode},
    math::{Circle, Vec2},
    shapes::draw_circle,
};

pub struct Weapon {
    pub pos: Vec2,
    pub weapon_lock: bool,
    pub shooting: bool,
    pub cur_time: f32,
    // pub bullets: Vec<Bullet>,
    pub bullets: HashMap<u32, Bullet>,
    pub reload_time: f32,
    pub muzzle: Circle,
    pub muzzle_active: bool,
    pub bullet_id: u32,
}
impl Weapon {
    pub fn update_weapon(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Space) && !self.weapon_lock {
            self.shooting = true;
            self.weapon_lock = true;
            self.muzzle_active = true;
        }
        if self.shooting {
            if self.cur_time < self.reload_time {
                self.muzzle = self.init_muzzle(self.cur_time);
                // self.cur_time += dt;
            }
            self.cur_time += dt;
        }
        if self.cur_time > self.reload_time / 1.2 && self.weapon_lock {
            let r = 5.;
            // self.bullets.push(Bullet {
            //     colour: GRAY,
            //     rec: Rect {
            //         x: self.pos.x - w / 2.,
            //         y: self.pos.y,
            //         w,
            //         h: 20.,
            //     },
            //     // active: true,
            // });
            self.bullets.insert(
                self.bullet_id,
                Bullet {
                    colour: GRAY,
                    circle: Circle {
                        x: self.pos.x,
                        y: self.pos.y,
                        r,
                        // h: 20.,
                    },
                    active: true,
                },
            );
            self.bullet_id += 1;
            self.weapon_lock = false
        }
        if self.cur_time >= self.reload_time {
            self.shooting = false;
            self.muzzle_active = false;
            // self.weapon_lock = false;
            self.cur_time = 0.;
        }

        // let mut inactive: Vec<u32>::new();
        let mut inactive: Vec<u32> = vec![];
        for (id, b) in &mut self.bullets {
            b.update_bullet(dt);
            if b.circle.y < -100. {
                b.active = false;
            }
            if !b.active {
                inactive.push(*id);
            }
        }
        self.bullets = delete_bullets(self.bullets.clone(), inactive);

        // println!(
        //     "Length of Bullets hashmap: {}. Its capacity: {}.",
        //     self.bullets.len(),
        //     self.bullets.capacity()
        // );
    }

    fn init_muzzle(&self, time: f32) -> Circle {
        Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: 20. * time,
        }
    }
    pub fn draw_muzzle(&self) {
        draw_circle(self.muzzle.x, self.muzzle.y, self.muzzle.r, YELLOW);
    }
}

// fn shoot_spark(pos: Vec2, dt: f32) {}
#[derive(Clone)]
pub struct Bullet {
    pub colour: Color,
    pub circle: Circle,
    pub active: bool,
}
impl Default for Bullet {
    fn default() -> Self {
        Self {
            colour: GRAY,
            circle: Circle {
                x: 0.,
                y: 0.,
                r: 0.,
                // h: 0.,
            },
            active: false,
        }
    }
}
impl Bullet {
    fn update_bullet(&mut self, dt: f32) {
        self.circle.y -= 2000. * dt;
    }

    pub fn draw_bullet(&self) {
        draw_circle(
            self.circle.x,
            self.circle.y,
            self.circle.r,
            // self.circle.h,
            self.colour,
        );
    }

    // pub fn bullet_collision(&mut self, enemies: Vec<Vec<Enemy>>) {
    //     for i in enemies {
    //         for e in i {

    //             if check_collision(self.circle, e.rec) {
    //                 self.active = false;
    //                 println!("BANG!");
    //             }
    //         }
    //     }
    // }
}

pub fn delete_bullets(mut b: HashMap<u32, Bullet>, i: Vec<u32>) -> HashMap<u32, Bullet> {
    for id in i {
        b.remove(&id);
    }
    return b;
}
