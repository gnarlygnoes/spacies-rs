use macroquad::{
    color::Color,
    input::{is_key_down, KeyCode},
    math::Rect,
    shapes::draw_rectangle,
};

use crate::WINDOW_WIDTH;

pub struct Player {
    pub rec: Rect,
    pub colour: Color,
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
    return Player {
        rec: Rect {
            x: (crate::WINDOW_WIDTH) / 2. - 25.,
            y: (crate::WINDOW_HEIGHT) - 100.,
            w: 50.,
            h: 80.,
        },
        colour,
    };
}

pub fn update_player(p: &mut Player, dt: f32) {
    if is_key_down(KeyCode::Left) {
        p.move_player(-1, dt);
    }
    if is_key_down(KeyCode::Right) {
        p.move_player(1, dt);
    }
}
