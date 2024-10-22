use game::game_loop::{draw_game, init_game, update_game};
use macroquad::prelude::*;
pub mod game;

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 1200.;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Spacies"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

// fn load_the_damn_textures() {}
//
#[macroquad::main(window_conf)]
async fn main() {
    // let tileset: Texture2D = load_texture("assets/SpaceInvaders.png").await.unwrap();
    // tileset.set_filter(FilterMode::Nearest);

    let mut game = init_game();

    loop {
        let dt = get_frame_time();

        // update stuff
        update_game(&mut game, dt);

        // draw stuff
        clear_background(BLACK);

        draw_game(&mut game);

        let fps = get_fps();
        draw_text(format!("{fps}").as_str(), 20., 50., 36., ORANGE);
        next_frame().await
    }
}
