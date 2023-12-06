use super::Vector2;

pub const BULLET_SPEED: f32 = 20.;

//////////////////////////////////////////////
///
///
/// Bullet
///
///
//////////////////////////////////////////////

pub struct Bullet {
    pub pos: Vector2,
    pub dir: Vector2,
    pub id: usize,
}

impl Bullet {
    pub fn new(pos: Vector2, dir: Vector2, id: usize) -> Self {
        Bullet { pos, dir, id }
    }
}
