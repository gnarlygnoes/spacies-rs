use macroquad::{
    color::Color,
    input::{is_key_down, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
};

use crate::WINDOW_WIDTH;

use super::weapon::{update_weapon, Weapon};

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
            self.rec.x -= 1000. * delta_time
        }
        if direction > 0 && self.rec.x + self.rec.w < WINDOW_WIDTH {
            self.rec.x += 1000. * delta_time
        }

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
        rec: Rect {
            x,
            y,
            w: 50.,
            h: 80.,
        },
        colour,
        weapon: Weapon {
            pos: Vec2 { x: x + w / 2., y },
            weapon_lock: false,
            shooting: false,
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
    update_weapon(&mut p.weapon, dt);
}
