use std::collections::HashMap;

use macroquad::{
    color::{Color, GRAY},
    math::Rect,
    shapes::draw_rectangle,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

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
            health: 20,
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
                    y: WINDOW_HEIGHT - 200.,
                    w,
                    h,
                },
                ..Default::default()
            },
        );
    }
    defences
}

pub fn draw_defences(defences: &HashMap<u8, Defence>) {
    for (_, d) in defences {
        draw_rectangle(d.rec.x, d.rec.y, d.rec.w, d.rec.h, d.colour);
    }
}
