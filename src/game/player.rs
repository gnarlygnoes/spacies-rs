use macroquad::{color::Color, math::Rect, shapes::draw_rectangle};

pub struct Player {
    pub rec: Rect,
    pub colour: Color,
}

impl Player {
    pub fn draw(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}
