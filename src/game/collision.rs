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
        for row in &mut game.enemies {
            for e in row {
                if e.alive {
                    let bullet_rec = Rect {
                        x: b.circle.x,
                        y: b.circle.y,
                        w: b.circle.r,
                        h: b.circle.r,
                    };
                    if check_collision(bullet_rec, e.rec) {
                        b.active = false;
                        e.alive = false;
                    }
                }
            }
        }
    }
}
