use crate::prelude::*;

pub struct Vec2 {
    pub x: BigRational,
    pub y: BigRational,
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn parse(s: &str) -> Vec2 {
        let (x, y) = s.split_once(',').unwrap();
        Vec2 {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
