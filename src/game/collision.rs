use macroquad::math::Rect;

use super::game_loop::Game;

pub fn check_collision(rec1: Rect, rec2: Rect) -> bool {
    if (rec1.y < rec2.y + rec2.h && rec1.y + rec1.h > rec2.y)
        && (rec1.x < rec2.x + rec2.w && rec1.x + rec1.w > rec2.x)
    {
        return true;
    }
    false
}

pub fn update_collision(game: &mut Game) {
    for row in &mut game.enemies {
        for e in row {
            // player bullet collision
            for (_, b) in &mut game.player.weapon.bullets {
                if e.alive {
                    let bullet_rec = Rect {
                        x: b.circle.x,
                        y: b.circle.y,
                        w: b.circle.r,
                        h: b.circle.r,
                    };

                    if check_collision(bullet_rec, e.rec) {
                        b.active = false;
                        e.health -= 1;
                    }
                }
            }
        }
    }
    for (_, b) in &mut game.enemy_bullets {
        // enemy bullets collision
        if b.active {
            let bullet_rec = Rect {
                x: b.circle.x,
                y: b.circle.y,
                w: b.circle.r,
                h: b.circle.r,
            };

            if check_collision(game.player.rec, bullet_rec) {
                b.active = false;
                game.player.health -= 1;
                // println!("OWW! FUCCKKKK!");
            }

            for (_, d) in &mut game.defences {
                if check_collision(d.rec, bullet_rec) {
                    b.active = false;
                    d.health -= 1;
                }
            }
        }
    }

    for (_, b) in &mut game.player.weapon.bullets {
        for (_, d) in &mut game.defences {
            let bullet_rec = Rect {
                x: b.circle.x,
                y: b.circle.y,
                w: b.circle.r,
                h: b.circle.r,
            };

            if check_collision(bullet_rec, d.rec) {
                b.active = false;
                d.health -= 1;
            }
        }
    }
}
