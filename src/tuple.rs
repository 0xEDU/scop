use std::ops;
use std::fmt;

/* Tuple/Point/Vector declaration and implementation ======================= */
#[derive(Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// pub type Point = Tuple;
pub type Vector = Tuple;

impl Tuple {
    pub fn new() -> Self {
        Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

/* ========================================================================= */

/* Operator overloads for Tuple ============================================ */
impl std::cmp::PartialEq<Tuple> for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.to_bits() == rhs.x.to_bits()
        && self.y.to_bits() == rhs.y.to_bits()
        && self.z.to_bits() == rhs.z.to_bits()
        && self.w.to_bits() == rhs.w.to_bits()
    }
}

impl std::cmp::Eq for Tuple {}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: if self.w == 0.0 {self.w} else {-self.w},
        }
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * 1.0 / rhs,
            y: self.y * 1.0 / rhs,
            z: self.z * 1.0 / rhs,
            w: self.w * 1.0 / rhs,
        }
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:10.5} ", self.x)?;
        write!(f, "{:10.5} ", self.y)?;
        write!(f, "{:10.5}", self.z)?;
        writeln!(f)?;
        Ok(())
    }
}
/* ========================================================================= */

/* Operations with Tuples ================================================== */
pub fn magnitude(tup: Tuple) -> f32 {
    (tup.x.powi(2) + tup.y.powi(2) + tup.z.powi(2) + tup.w.powi(2))
    .sqrt()
}

pub fn normalize(tup: Tuple) -> Tuple {
    tup / magnitude(tup)
}

pub fn dot(a: Tuple, b: Tuple) -> f32 {
    a.x * b.x +
    a.y * b.y +
    a.z * b.z +
    a.w * b.w
}

pub fn cross(a: Tuple, b: Tuple) -> Tuple {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}
/* ========================================================================= */

/* Factory functions ======================================================= */
// pub fn point(x: f32, y: f32, z: f32) -> Point {
//     Point { x, y, z, w: 1.0 }
// }

pub fn vector(x: f32, y: f32, z: f32) -> Vector {
    Vector { x, y, z, w: 0.0 }
}
/* ========================================================================= */
