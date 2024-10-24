use game::{
    game_loop::{draw_game, init_game, update_game, GameState},
    main_menu::{draw_menu, game_paused, update_menu},
    settings::handle_inputs,
    victory_defeat::{defeat_mode, draw_defeat, draw_victory, victory_mode},
};
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

        // draw stuff
        clear_background(BLACK);

        // if game.game_state == GameState::InGame {
        handle_inputs(&mut game);
        match game.game_state {
            GameState::Menu => {
                update_menu(&mut game);
                draw_menu();
            }
            GameState::InGame => {
                update_game(&mut game, dt);
                draw_game(&game);
            }
            GameState::Paused => {
                draw_game(&game);
                game_paused();
            }
            GameState::Victorious => {
                victory_mode(&mut game);
                draw_victory(&game);
            }
            GameState::Defeated => {
                defeat_mode(&mut game);
                draw_defeat();
            }
        }
        // }

        let fps = get_fps();
        draw_text(format!("{fps}").as_str(), 20., 50., 36., ORANGE);
        next_frame().await
    }
}
