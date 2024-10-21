use macroquad::math::Rect;

pub fn check_collision(rec1: Rect, rec2: Rect) -> bool {
    if (rec1.y < rec2.y + rec2.h && rec1.y + rec1.h > rec2.h)
        && (rec1.x < rec2.x + rec2.w && rec1.x + rec1.w > rec2.x)
    {
        return true;
    }
    false
}

// pub fn update_collision() {}
