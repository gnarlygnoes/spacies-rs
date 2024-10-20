use macroquad::math::Rect;

use super::game_loop::Game;

pub fn check_collision(rec1: Rect, rec2: Rect) -> bool {
    if (rec1.y < rec2.y + rec2.h && rec1.y + rec1.h > rec2.h)
        && (rec1.x < rec2.x + rec2.w && rec1.x + rec1.w > rec2.x)
    {
        return true;
    }
    false
}

pub fn update_collision(game: &mut Game) {
    for (_, b) in &mut game.player.weapon.bullets {
        for (_, e) in &mut game.enemies {
            // for e in i {
            if check_collision(b.rec, e.rec) {
                b.active = false;
                e.alive = false;
            }
            // }
        }
        // if check_collision(b, )
    }
}
