use std::{fmt::Display, hash::Hash, ops};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector2Int {
    pub x: isize,
    pub y: isize,
}

#[allow(unused)]
impl Vector2Int {
    pub const CARDINALS: [Vector2Int; 4] = [
        Vector2Int { x: 0, y: 1 },
        Vector2Int { x: 1, y: 0 },
        Vector2Int { x: 0, y: -1 },
        Vector2Int { x: -1, y: 0 },
    ];

    pub fn new(x: isize, y: isize) -> Self {
        return Vector2Int { x: x, y: y };
    }

    /// Check if in range inclusive
    pub fn in_range(&self, min: &Vector2Int, max: &Vector2Int) -> bool {
        return self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y;
    }

    /// All 8 basic vector2 directions starting with UP going clockwise
    pub fn all_directions() -> [Vector2Int; 8] {
        return [
            Vector2Int::up(),
            Vector2Int::ur(),
            Vector2Int::right(),
            Vector2Int::dr(),
            Vector2Int::down(),
            Vector2Int::dl(),
            Vector2Int::left(),
            Vector2Int::ul(),
        ];
    }

    pub fn zero() -> Self {
        return Vector2Int { x: 0, y: 0 };
    }

    pub fn up() -> Self {
        return Vector2Int { x: 0, y: 1 };
    }

    pub fn down() -> Self {
        return Vector2Int { x: 0, y: -1 };
    }

    pub fn left() -> Self {
        return Vector2Int { x: -1, y: 0 };
    }

    pub fn right() -> Self {
        return Vector2Int { x: 1, y: 0 };
    }

    pub fn ur() -> Self {
        return Vector2Int { x: 1, y: 1 };
    }

    pub fn ul() -> Self {
        return Vector2Int { x: -1, y: 1 };
    }

    pub fn dl() -> Self {
        return Vector2Int { x: -1, y: -1 };
    }

    pub fn dr() -> Self {
        return Vector2Int { x: 1, y: -1 };
    }

    pub fn manhattan(&self, rhs: Vector2Int) -> isize {
        return (self.x - rhs.x).abs() + (self.y - rhs.y).abs();
    }
}

impl Display for Vector2Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

// +
impl ops::Add<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, rhs: Vector2Int) -> Self::Output {
        return Vector2Int::new(self.x + rhs.x, self.y + rhs.y);
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

// /=
impl ops::DivAssign<Vector2Int> for Vector2Int {
    fn div_assign(&mut self, rhs: Vector2Int) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
