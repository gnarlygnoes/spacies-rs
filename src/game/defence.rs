use std::collections::HashMap;

use macroquad::{
    color::{Color, GRAY},
    math::Rect,
    shapes::draw_rectangle,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::game_loop::Game;

pub struct Defence {
    pub rec: Rect,
    pub colour: Color,
    pub health: i32,
}
impl Default for Defence {
    fn default() -> Self {
        Self {
            rec: Rect {
                x: 0.,
                y: 0.,
                w: 150.,
                h: 100.,
            },
            colour: GRAY,
            health: 10,
        }
    }
}
pub fn init_defences() -> HashMap<u8, Defence> {
    let mut defences = HashMap::new();
    let w = 150.;
    let h = 100.;
    for i in 0..4 {
        defences.insert(
            i,
            Defence {
                rec: Rect {
                    // x: 50. + w * 1.5 * i as f32,
                    x: 50. + (WINDOW_WIDTH / 4.0) * i as f32,
                    y: WINDOW_HEIGHT - 220.,
                    w,
                    h,
                },
                ..Default::default()
            },
        );
    }
    defences
}

pub fn update_defences(g: &mut Game) {
    let mut iterator: Vec<u8> = vec![];
    for (id, d) in &mut g.defences {
        if d.health <= 0 {
            // g.defences.remove(id);
            iterator.push(*id);
        }
    }
    for id in iterator {
        g.defences.remove(&id);
    }
}

pub fn draw_defences(defences: &HashMap<u8, Defence>) {
    for (_, d) in defences {
        draw_rectangle(d.rec.x, d.rec.y, d.rec.w, d.rec.h, d.colour);
    }
}
