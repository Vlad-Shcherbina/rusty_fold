use num_traits::FromPrimitive;
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

fn iter_edges(poly: &[Vec2]) -> impl Iterator<Item=(&Vec2, &Vec2)> {
    poly.iter().zip(
        poly.iter().skip(1).chain(std::iter::once(&poly[0]))
    )
}

pub fn area(poly: &[Vec2]) -> BigRational {
    let mut s: BigRational = num_traits::zero();
    for (a, b) in iter_edges(poly) {
        s += (&a.x + &b.x) * (&b.y - &a.y);
    }
    s / BigRational::from_i32(2).unwrap()
}
