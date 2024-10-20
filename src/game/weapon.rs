use macroquad::{
    color::{Color, YELLOW},
    input::{is_key_pressed, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_circle,
};

pub struct Weapon {
    // Shooter that fires a bullet
    // bullets: Vec<Bullet>,
    pub pos: Vec2,
    pub weapon_lock: bool,
    pub shooting: bool,
}
impl Weapon {
    fn draw_shoot(&mut self) {
        // if t < 25. {
        draw_circle(self.pos.x, self.pos.y - 50., 100., YELLOW);
        // t += dt;
        // println!("{t}");
        // }
        // create_bullet();
    }
}

// fn shoot_spark(pos: Vec2, dt: f32) {}

// pub struct Bullet {
//     colour: Color,
//     rec: Rect,
// }

// pub fn create_weapon(pos: Vec2) -> Weapon {
//     Weapon { pos }
// }

fn set_time() -> f32 {
    return 0.;
}

pub fn update_weapon(weapon: &mut Weapon, dt: f32) {
    // println!()
    if is_key_pressed(KeyCode::Space) && !weapon.weapon_lock {
        weapon.shooting = true;
        weapon.weapon_lock = true;
    }
    if weapon.shooting {
        let mut t = 0.;
        // while t < 25. {
        //     weapon.draw_shoot(dt, t);
        //     t += dt;
        //     println!("{t}");
        // }
        // weapon.weapon_lock = false;
        if weapon.shooting {
            if t < 1. {
                weapon.draw_shoot();
            }
        }
        t += dt;
        println!("{t}");
    }
}
