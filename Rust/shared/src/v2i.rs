use std::hash::Hash;
use std::marker::Copy;
use std::ops;

pub struct Vector2Int {
    pub x: isize,
    pub y: isize,
}

pub const UP: Vector2Int = new_v2(0, 1);
pub const DOWN: Vector2Int = new_v2(0, -1);
pub const LEFT: Vector2Int = new_v2(-1, 0);
pub const RIGHT: Vector2Int = new_v2(1, 0);

pub const fn new_v2(x: isize, y: isize) -> Vector2Int {
    return Vector2Int { x, y };
}

impl Vector2Int {
    pub fn new(x: isize, y: isize) -> Self {
        return Vector2Int { x, y };
    }
}

impl Copy for Vector2Int {}
impl Clone for Vector2Int {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl Hash for Vector2Int {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

// a + b -> c
impl ops::Add<Vector2Int> for Vector2Int {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// a += b
impl ops::AddAssign<Vector2Int> for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// a - b -> c
impl ops::Sub<Vector2Int> for Vector2Int {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// a -= b
impl ops::SubAssign<Vector2Int> for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// a * b
impl ops::Mul<Vector2Int> for Vector2Int {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// a *= b
impl ops::MulAssign<Vector2Int> for Vector2Int {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl std::cmp::Eq for Vector2Int {}
impl std::cmp::PartialEq for Vector2Int {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
