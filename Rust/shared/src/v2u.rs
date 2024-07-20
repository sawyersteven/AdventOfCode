/*
Based mostly on Unity's Vector2:
https://github.com/Unity-Technologies/UnityCsReference/blob/master/Runtime/Export/Math/Vector2.cs
*/

use std::{f64::consts::PI, fmt::Display, hash::Hash, ops};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Vector2Usize {
    pub x: usize,
    pub y: usize,
}

#[allow(unused)]
impl Vector2Usize {
    pub const fn new(x: usize, y: usize) -> Self {
        return Vector2Usize { x: x, y: y };
    }

    /// Check if in range inclusive
    pub fn in_range(&self, min: &Vector2Usize, max: &Vector2Usize) -> bool {
        return self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y;
    }

    pub const ZERO: Vector2Usize = Vector2Usize { x: 0, y: 0 };
    pub const ONE: Vector2Usize = Vector2Usize { x: 1, y: 1 };

    pub fn manhattan(&self, rhs: &Vector2Usize) -> usize {
        return if self.x > rhs.x { self.x - rhs.x } else { rhs.x - self.x }
            + if self.y > rhs.y { self.y - rhs.y } else { rhs.y - self.y };
    }

    pub fn angle_to(&self, other: &Vector2Usize) -> f64 {
        let ang = -((180f64 / PI)
            * f64::atan2(
                ((self.x * other.x) + (self.y * other.y)) as f64,
                ((self.x * other.y) + (self.y * other.x)) as f64,
            ))
            + 90f64;
        return if ang < 0f64 { ang + 360f64 } else { ang };
    }
}

impl Display for Vector2Usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

// +
impl ops::Add<Vector2Usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn add(self, rhs: Vector2Usize) -> Self::Output {
        return Vector2Usize::new(self.x + rhs.x, self.y + rhs.y);
    }
}

// + (isize, isize)
impl ops::Add<(usize, usize)> for Vector2Usize {
    type Output = Vector2Usize;
    fn add(self, rhs: (usize, usize)) -> Self::Output {
        return Vector2Usize::new(self.x + rhs.0, self.y + rhs.1);
    }
}

// +=
impl ops::AddAssign<Vector2Usize> for Vector2Usize {
    fn add_assign(&mut self, rhs: Vector2Usize) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// -
impl ops::Sub<Vector2Usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn sub(self, rhs: Vector2Usize) -> Self::Output {
        return Vector2Usize::new(self.x - rhs.x, self.y - rhs.y);
    }
}

// -=
impl ops::SubAssign<Vector2Usize> for Vector2Usize {
    fn sub_assign(&mut self, rhs: Vector2Usize) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// *
impl ops::Mul<Vector2Usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn mul(self, rhs: Vector2Usize) -> Self::Output {
        return Vector2Usize::new(self.x * rhs.x, self.y * rhs.y);
    }
}

// * <isize>
impl ops::Mul<usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn mul(self, rhs: usize) -> Self::Output {
        return Vector2Usize::new(self.x * rhs, self.y * rhs);
    }
}

// *=
impl ops::MulAssign<Vector2Usize> for Vector2Usize {
    fn mul_assign(&mut self, rhs: Vector2Usize) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

// /
impl ops::Div<Vector2Usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn div(self, rhs: Vector2Usize) -> Self::Output {
        return Vector2Usize::new(self.x / rhs.x, self.y / rhs.y);
    }
}

// / <isize>
impl ops::Div<usize> for Vector2Usize {
    type Output = Vector2Usize;
    fn div(self, rhs: usize) -> Self::Output {
        return Vector2Usize::new(self.x / rhs, self.y / rhs);
    }
}

// /=
impl ops::DivAssign<Vector2Usize> for Vector2Usize {
    fn div_assign(&mut self, rhs: Vector2Usize) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// %
// Returns <self.x % rhs.x, self.y % rhs.y>
impl ops::Rem<Vector2Usize> for Vector2Usize {
    type Output = Vector2Usize;

    fn rem(self, rhs: Vector2Usize) -> Self::Output {
        return Vector2Usize::new(self.x % rhs.x, self.y % rhs.y);
    }
}
