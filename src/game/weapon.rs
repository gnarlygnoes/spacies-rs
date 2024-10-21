use macroquad::{
    color::{Color, GRAY, YELLOW},
    input::{is_key_pressed, KeyCode},
    math::{Circle, Rect, Vec2},
    shapes::{draw_circle, draw_rectangle},
};

pub struct Weapon {
    pub pos: Vec2,
    pub weapon_lock: bool,
    pub shooting: bool,
    pub cur_time: f32,
    pub bullets: Vec<Bullet>,
    pub reload_time: f32,
    pub muzzle: Circle,
    pub muzzle_active: bool,
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
        if self.cur_time > self.reload_time / 2. && self.weapon_lock {
            let w = 5.;
            self.bullets.push(Bullet {
                colour: GRAY,
                rec: Rect {
                    x: self.pos.x - w / 2.,
                    y: self.pos.y,
                    w,
                    h: 20.,
                },
                // active: true,
            });
            self.weapon_lock = false
        }
        if self.cur_time >= self.reload_time {
            self.shooting = false;
            self.muzzle_active = false;
            // self.weapon_lock = false;
            self.cur_time = 0.;
        }

        for b in &mut self.bullets {
            b.update_bullet(dt);
        }
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
    colour: Color,
    rec: Rect,
    // active: bool,
}
impl Default for Bullet {
    fn default() -> Self {
        Self {
            colour: GRAY,
            rec: Rect {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
            // active: false,
        }
    }
}
impl Bullet {
    fn update_bullet(&mut self, dt: f32) {
        self.rec.y -= 2000. * dt;
    }

    pub fn draw_bullet(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}
