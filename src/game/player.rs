use macroquad::{
    color::Color,
    input::{is_key_down, KeyCode},
    math::{Circle, Rect, Vec2},
    shapes::draw_rectangle,
};

use crate::WINDOW_WIDTH;

use super::weapon::{Bullet, Weapon};

pub struct Player {
    pub rec: Rect,
    pub colour: Color,
    pub weapon: Weapon,
}
impl Player {
    pub fn draw(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }

    pub fn move_player(&mut self, direction: i8, delta_time: f32) {
        // if is_key_down(macroquad::input::KeyCode::A) {
        if direction < 0 && self.rec.x > 0. {
            self.rec.x -= 1000. * delta_time;
        }
        if direction > 0 && self.rec.x + self.rec.w < WINDOW_WIDTH {
            self.rec.x += 1000. * delta_time
        }
        self.weapon.pos.x = self.rec.x + self.rec.w / 2.;
        self.weapon.pos.y = self.rec.y;

        if self.rec.x + self.rec.w > WINDOW_WIDTH {
            self.rec.x = WINDOW_WIDTH - self.rec.w
        }
        if self.rec.x < 0. {
            self.rec.x = 0.
        }
    }
}

pub fn create_player(colour: Color) -> Player {
    let x = (crate::WINDOW_WIDTH) / 2. - 25.;
    let y = (crate::WINDOW_HEIGHT) - 100.;
    let w = 50.;
    let h = 80.;
    return Player {
        rec: Rect { x, y, w, h },
        colour,
        weapon: Weapon {
            pos: Vec2 { x: x + w / 2., y },
            weapon_lock: false,
            shooting: false,
            cur_time: 0.,
            bullets: vec![Bullet::default(); 0],
            reload_time: 0.2,
            muzzle: Circle {
                x: 0.,
                y: 0.,
                r: 0.,
            },
            muzzle_active: false,
        },
    };
}

pub fn update_player(p: &mut Player, dt: f32) {
    if is_key_down(KeyCode::Left) {
        p.move_player(-1, dt);
    }
    if is_key_down(KeyCode::Right) {
        p.move_player(1, dt);
    }

    p.weapon.update_weapon(dt);
}
