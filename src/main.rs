use game::player::Player;
use macroquad::prelude::*;
pub mod game;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 1200.;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Spacies"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pos_x = 200.;
    let mut player = Player {
        rec: Rect {
            x: (WINDOW_WIDTH) / 2. - 25.,
            y: (WINDOW_HEIGHT) - 100.,
            w: 50.,
            h: 80.,
        },
        colour: ORANGE,
    };

    loop {
        // draw stuff
        clear_background(BLACK);

        player.draw();

        next_frame().await
    }
}
