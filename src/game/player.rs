use std::collections::HashMap;

use macroquad::{
    color::{Color, ORANGE},
    input::{is_key_down, KeyCode},
    math::{Circle, Rect, Vec2},
    shapes::draw_rectangle,
};

use crate::WINDOW_WIDTH;

use super::weapon::Weapon;

pub struct Player {
    pub rec: Rect,
    // pub tex: Texture
    pub colour: Color,
    pub weapon: Weapon,

    pub health: u8,
}
impl Player {
    pub fn create_player() -> Self {
        let x = (crate::WINDOW_WIDTH) / 2. - 25.;
        let y = (crate::WINDOW_HEIGHT) - 100.;
        let w = 50.;
        let h = 80.;
        return Self {
            rec: Rect { x, y, w, h },
            colour: ORANGE,
            weapon: Weapon {
                pos: Vec2 { x: x + w / 2., y },
                weapon_lock: false,
                shooting: false,
                cur_time: 0.,
                // bullets: vec![Bullet::default(); 0],
                bullets: HashMap::new(),
                reload_time: 0.4,
                muzzle: Circle {
                    x: 0.,
                    y: 0.,
                    r: 0.,
                },
                muzzle_active: false,
                bullet_id: 0,
            },
            health: 1,
        };
    }

    pub fn draw(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
        // draw_texture(&tex, self.rec.x, self.rec.y, WHITE);
    }

    pub fn move_player(&mut self, direction: i8, delta_time: f32) {
        if direction < 0 && self.rec.x > 0. {
            self.rec.x -= 700. * delta_time;
        }
        if direction > 0 && self.rec.x + self.rec.w < WINDOW_WIDTH {
            self.rec.x += 700. * delta_time
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

pub fn update_player(p: &mut Player, dt: f32) {
    if is_key_down(KeyCode::Left) {
        p.move_player(-1, dt);
    }
    if is_key_down(KeyCode::Right) {
        p.move_player(1, dt);
    }

    p.weapon.update_weapon(dt);
}
