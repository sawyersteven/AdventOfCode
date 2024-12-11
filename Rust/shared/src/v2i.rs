/*
Based mostly on Unity's Vector2:
https://github.com/Unity-Technologies/UnityCsReference/blob/master/Runtime/Export/Math/Vector2.cs
*/

use std::{
    f64::consts::PI,
    fmt::Display,
    hash::Hash,
    ops::{self},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Vector2Int {
    pub x: isize,
    pub y: isize,
}

#[allow(unused)]
impl Vector2Int {
    pub const CARDINALS: [Vector2Int; 4] = [Vector2Int::UP, Vector2Int::RIGHT, Vector2Int::DOWN, Vector2Int::LEFT];

    /// All 8 basic vector2 directions starting with UP going clockwise
    pub const ALL_DIRS: [Vector2Int; 8] = [
        Vector2Int::UP,
        Vector2Int::UR,
        Vector2Int::RIGHT,
        Vector2Int::DR,
        Vector2Int::DOWN,
        Vector2Int::DL,
        Vector2Int::LEFT,
        Vector2Int::UL,
    ];

    /// Converts passed usize to isize without concern for incompatible values
    pub fn new_from_usize(x: usize, y: usize) -> Self {
        return Self {
            x: x as isize,
            y: y as isize,
        };
    }

    pub const fn new(x: isize, y: isize) -> Self {
        return Vector2Int { x, y };
    }

    /// Check if in range inclusive
    #[inline(always)]
    pub fn in_range(&self, min: &Vector2Int, max: &Vector2Int) -> bool {
        return self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y;
    }

    pub const ZERO: Vector2Int = Vector2Int { x: 0, y: 0 };
    pub const ONE: Vector2Int = Vector2Int { x: 1, y: 1 };

    pub const UP: Vector2Int = Vector2Int { x: 0, y: 1 };
    pub const DOWN: Vector2Int = Vector2Int { x: 0, y: -1 };
    pub const LEFT: Vector2Int = Vector2Int { x: -1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x: 1, y: 0 };

    pub const UR: Vector2Int = Vector2Int { x: 1, y: 1 };
    pub const UL: Vector2Int = Vector2Int { x: -1, y: 1 };
    pub const DL: Vector2Int = Vector2Int { x: -1, y: -1 };
    pub const DR: Vector2Int = Vector2Int { x: 1, y: -1 };

    pub fn manhattan(&self, rhs: &Vector2Int) -> isize {
        return (self.x - rhs.x).abs() + (self.y - rhs.y).abs();
    }

    pub fn angle_to(&self, other: &Vector2Int) -> f64 {
        let ang = -((180f64 / PI)
            * f64::atan2(
                ((self.x * other.x) + (self.y * other.y)) as f64,
                ((self.x * other.y) + (self.y * other.x)) as f64,
            ))
            + 90f64;
        return if ang < 0f64 { ang + 360f64 } else { ang };
    }

    pub fn neighbors(&self) -> [Vector2Int; 4] {
        let mut a = [*self; 4];
        for i in 0..4 {
            a[i] += Vector2Int::CARDINALS[i];
        }
        return a;
    }
}

impl Display for Vector2Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl From<(usize, usize)> for Vector2Int {
    fn from(value: (usize, usize)) -> Self {
        return Self {
            x: value.0 as isize,
            y: value.1 as isize,
        };
    }
}

// +
impl ops::Add<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x + rhs.x, self.y + rhs.y);
    }
}

// + (isize, isize)
impl ops::Add<(isize, isize)> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        return Vector2Int::new(self.x + rhs.0, self.y + rhs.1);
    }
}

// +=
impl ops::AddAssign<Vector2Int> for Vector2Int {
    fn add_assign(&mut self, rhs: Vector2Int) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// -
impl ops::Sub<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn sub(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x - rhs.x, self.y - rhs.y);
    }
}

// -=
impl ops::SubAssign<Vector2Int> for Vector2Int {
    fn sub_assign(&mut self, rhs: Vector2Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// *
impl ops::Mul<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x * rhs.x, self.y * rhs.y);
    }
}

// * <isize>
impl ops::Mul<isize> for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, rhs: isize) -> Self::Output {
        return Vector2Int::new(self.x * rhs, self.y * rhs);
    }
}

// *=
impl ops::MulAssign<Vector2Int> for Vector2Int {
    fn mul_assign(&mut self, rhs: Vector2Int) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

// /
impl ops::Div<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn div(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x / rhs.x, self.y / rhs.y);
    }
}

// / <isize>
impl ops::Div<isize> for Vector2Int {
    type Output = Vector2Int;
    fn div(self, rhs: isize) -> Self::Output {
        return Vector2Int::new(self.x / rhs, self.y / rhs);
    }
}

// /=
impl ops::DivAssign<Vector2Int> for Vector2Int {
    fn div_assign(&mut self, rhs: Vector2Int) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// %
// Returns <self.x % rhs.x, self.y % rhs.y>
impl ops::Rem<Vector2Int> for Vector2Int {
    type Output = Vector2Int;

    fn rem(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x % rhs.x, self.y % rhs.y);
    }
}
