use macroquad::{color::Color, math::Rect};

pub struct Weapon {
    // Shooter that fires a bullet
    bullets: Vec<Bullet>,
}
impl Weapon {
    // pub fn shoot(self) {
    //     draw_circle(self.x + (self.w / 2.), y, r, color);
    //     create_bullet();
    // }
}

pub struct Bullet {
    colour: Color,
    rec: Rect,
}
