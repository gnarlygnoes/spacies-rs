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
            }
            self.cur_time += dt;
        }
        if self.cur_time > self.reload_time / 1.2 && self.weapon_lock {
            let r = 5.;
            self.bullets.insert(
                self.bullet_id,
                Bullet {
                    colour: GRAY,
                    circle: Circle {
                        x: self.pos.x,
                        y: self.pos.y,
                        r,
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
            self.cur_time = 0.;
        }
        self.is_bullet_active(dt);
    }

    pub fn is_bullet_active(&mut self, dt: f32) {
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

#[derive(Clone, Copy)]
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
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.colour);
    }
}

pub fn delete_bullets(mut b: HashMap<u32, Bullet>, i: Vec<u32>) -> HashMap<u32, Bullet> {
    for id in i {
        b.remove(&id);
    }
    return b;
}
