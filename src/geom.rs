use num_traits::{FromPrimitive, ToPrimitive, Zero, One};
use crate::prelude::*;

#[derive(serde::Serialize)]
#[serde(into="(f64, f64)")]
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pt {
    pub x: BigRational,
    pub y: BigRational,
}

impl std::fmt::Debug for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Pt> for (f64, f64) {
    fn from(p: Pt) -> Self {
        (p.x.to_f64().unwrap(), p.y.to_f64().unwrap())
    }
}

impl Pt {
    pub fn parse(s: &str) -> Pt {
        let (x, y) = s.split_once(',').unwrap();
        Pt {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn cross(&self, other: &Pt) -> BigRational {
        &self.x * &other.y - &self.y * &other.x
    }

    fn rot_cw90(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        self.y = -&self.y;
    }

    pub fn angle(mut self) -> BigRational {
        assert!(!self.x.is_zero() || !self.y.is_zero());
        let mut s = BigRational::zero();
        loop {
            if self.x >= BigRational::zero() &&
               self.y >= BigRational::zero() {
                return s + &self.y / (&self.x + &self.y);
            }
            s += BigRational::one();
            self.rot_cw90();
        }
    }
}

#[test]
fn test_angle() {
    assert_eq!(Pt::parse("10,0").angle().to_string(), "0");
    assert_eq!(Pt::parse("0,10").angle().to_string(), "1");
    assert_eq!(Pt::parse("-10,0").angle().to_string(), "2");
    assert_eq!(Pt::parse("0,-10").angle().to_string(), "3");

    assert_eq!(Pt::parse("1,1").angle().to_string(), "1/2");
    assert_eq!(Pt::parse("-1,-3").angle().to_string(), "11/4");
}

impl std::ops::Sub<&Pt> for &Pt {
    type Output = Pt;

    fn sub(self, rhs: &Pt) -> Self::Output {
        Pt {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
        }
    }
}

impl std::ops::Mul<&BigRational> for &Pt {
    type Output = Pt;

    fn mul(self, rhs: &BigRational) -> Self::Output {
        Pt {
            x: &self.x * rhs,
            y: &self.y * rhs,
        }
    }
}

pub fn iter_edges(poly: &[Pt]) -> impl Iterator<Item=(&Pt, &Pt)> {
    poly.iter().zip(
        poly.iter().skip(1).chain(std::iter::once(&poly[0]))
    )
}

pub fn area(poly: &[Pt]) -> BigRational {
    let mut s: BigRational = num_traits::zero();
    for (a, b) in iter_edges(poly) {
        s += (&a.x + &b.x) * (&b.y - &a.y);
    }
    s / BigRational::from_i32(2).unwrap()
}

pub fn segment_intersection(seg1: (&Pt, &Pt), seg2: (&Pt, &Pt)) -> Option<Pt> {
    let bb_x1 = (&seg1.0.x).min(&seg1.1.x).max((&seg2.0.x).min(&seg2.1.x));
    let bb_y1 = (&seg1.0.y).min(&seg1.1.y).max((&seg2.0.y).min(&seg2.1.y));
    let bb_x2 = (&seg1.0.x).max(&seg1.1.x).min((&seg2.0.x).max(&seg2.1.x));
    let bb_y2 = (&seg1.0.y).max(&seg1.1.y).min((&seg2.0.y).max(&seg2.1.y));

    let d1 = seg1.1 - seg1.0;
    let d2 = seg2.1 - seg2.0;

    let c = d1.cross(&d2);
    if c.is_zero() {
        cov_mark::hit!(collinear);
        return None;
    }
    let alpha = d1.cross(&(seg2.0 - seg1.0));
    let  beta = d1.cross(&(seg2.1 - seg1.0));
    assert_ne!(alpha, beta);

    let pt = &(&(seg2.0 * &beta) - &(seg2.1 * &alpha)) * &(beta - alpha).recip();
    if *bb_x1 <= pt.x && pt.x <= *bb_x2 &&
       *bb_y1 <= pt.y && pt.y <= *bb_y2 {
        Some(pt)
    } else {
        cov_mark::hit!(intersection_outside);
        None
    }
}

#[test]
fn test_segment_interseciton() {
    let res = segment_intersection(
        (&Pt::parse("1,0"), &Pt::parse("2,2")),
        (&Pt::parse("2,0"), &Pt::parse("1,1")));
    assert_eq!(res, Some(Pt::parse("4/3,2/3")));

    {
        cov_mark::check!(collinear);
        let res = segment_intersection(
            (&Pt::parse("1,2"), &Pt::parse("2,4")),
            (&Pt::parse("0,0"), &Pt::parse("3,6")));
        assert_eq!(res, None);
    }

    {
        cov_mark::check!(intersection_outside);
        let res = segment_intersection(
            (&Pt::parse("0,0"), &Pt::parse("1,1")),
            (&Pt::parse("2,1"), &Pt::parse("3,0")));
        assert_eq!(res, None);
    }
}
