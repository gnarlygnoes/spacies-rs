use macroquad::{
    color::{Color, RED},
    math::Rect,
    shapes::draw_rectangle,
};

pub struct Enemy {
    pub rec: Rect,
    pub colour: Color,
}
impl Default for Enemy {
    fn default() -> Self {
        Self {
            rec: Rect {
                x: 0.,
                y: 0.,
                w: 40.,
                h: 60.,
            },
            colour: RED,
        }
    }
}
impl Enemy {
    pub fn draw_enemy(&self) {
        draw_rectangle(self.rec.x, self.rec.y, self.rec.w, self.rec.h, self.colour);
    }
}

pub fn create_enemies() -> Vec<Vec<Enemy>> {
    let w = 40.;
    let h = 60.;
    let mut enemies = vec![vec![]];
    for i in 0..5 {
        enemies.push(vec![Enemy {
            rec: Rect {
                x: 10.,
                y: 10.,
                w: 40.,
                h: 60.,
            },
            colour: RED,
        }]);
        for j in 0..10 {
            enemies[i].push(Enemy {
                rec: Rect {
                    x: (1.5 * j as f32 * w) + 10.,
                    y: (1.5 * i as f32 * h) + 10.,
                    w,
                    h,
                },
                ..Default::default()
            });
        }
    }
    // println!("{}", enemies[0].len());
    return enemies;
}

pub fn draw_enemes(enemies: &Vec<Vec<Enemy>>) {
    for i in enemies {
        for j in i {
            j.draw_enemy();
        }
    }
}
