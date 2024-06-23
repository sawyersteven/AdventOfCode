#![allow(unused)]

use std::{iter::StepBy, ops::Index, slice::Iter};

use crate::v2i::Vector2Int;

#[derive(Clone, Copy)]
pub struct Size2D {
    pub x: usize,
    pub y: usize,
}

pub struct Grid2D<T> {
    inner: Vec<T>,
    size: Size2D,
}

impl Grid2D<char> {
    /*
    Creates Grid2D from chars in input string
    Rows must be separated by newline chars, which will not be included in the grid
     */
    pub fn from_string(input: &String) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut g: Grid2D<char> = Grid2D {
            inner: Vec::<char>::with_capacity(lines[0].len() * lines.len()),
            size: Size2D {
                x: lines[0].len(),
                y: lines.len(),
            },
        };

        for line in lines {
            g.inner.extend(line.chars());
        }

        return g;
    }
}

impl Grid2D<u8> {
    /*
    Creates Grid2D from bytes in input string
    Rows must be separated by newline chars, which will not be included in the grid
     */
    pub fn from_string(input: &String) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut g: Grid2D<u8> = Grid2D {
            inner: Vec::<u8>::with_capacity(lines[0].len() * lines.len()),
            size: Size2D {
                x: lines[0].len(),
                y: lines.len(),
            },
        };

        for line in lines {
            g.inner.extend(line.bytes());
        }

        return g;
    }
}

impl<T> Grid2D<T> {
    pub fn new(x_len: usize, y_len: usize) -> Self
    where
        T: Default,
    {
        let mut v = Vec::new();
        v.resize_with(x_len * y_len, T::default);
        return Grid2D {
            inner: v,
            size: Size2D { x: x_len, y: y_len },
        };
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(x_len: usize, y_len: usize, iter: I) -> Option<Self> {
        let v: Vec<T> = iter.into_iter().collect();
        if v.len() != x_len * y_len {
            return None;
        } else {
            return Some(Self {
                inner: v,
                size: Size2D { x: x_len, y: y_len },
            });
        }
    }

    pub fn size(&self) -> Size2D {
        return self.size;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        return &self.inner[self.xy_to_ind(x, y)];
    }

    // Converts x,y coords to linear coords
    fn xy_to_ind(&self, x: usize, y: usize) -> usize {
        return x + (y * self.size.x);
    }

    /// Returns iterator over selected Y column in grid
    pub fn iter_col(&self, column: usize) -> StepBy<Iter<T>> {
        return self.inner[column..].iter().step_by(self.size.x);
    }

    /// Returns iterator over selected X row in grid
    pub fn iter_row(&self, row: usize) -> Iter<T> {
        let start = (row * self.size.x);
        return self.inner[start..(start + self.size.x)].iter();
    }
}

impl<T> Index<Vector2Int> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: Vector2Int) -> &Self::Output {
        return &self.inner[self.xy_to_ind(index.x as usize, index.y as usize)];
    }
}

impl<T> Index<(usize, usize)> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.inner[self.xy_to_ind(index.0 as usize, index.1 as usize)];
    }
}

#[cfg(test)]
mod tests {
    use crate::v2i::Vector2Int;

    use super::Grid2D;

    const TEST_DATA_5X7: [[usize; 5]; 7] = [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
        [21, 22, 23, 24, 25],
        [26, 27, 28, 29, 30],
        [31, 32, 33, 34, 35],
    ];

    const TEST_DATA_3X3: [[usize; 3]; 3] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    #[test]
    fn test_get_simple() {
        // Test simple 3x3x3 grid
        let mut grid: Grid2D<usize> = Grid2D::new(3, 3);
        grid.inner = (0..(3 * 3)).collect();

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(TEST_DATA_3X3[y][x], *grid.get(x, y));
            }
        }
    }

    #[test]
    fn test_get_asymmetrical() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        for x in 0..5 {
            for y in 0..7 {
                assert_eq!(TEST_DATA_5X7[y][x], *grid.get(x, y));
            }
        }
    }

    #[test]
    fn test_v2i_index_asymmetrical() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        let mut ind = Vector2Int::ZERO;
        for x in 0..5 {
            ind.x = x as isize;
            for y in 0..7 {
                ind.y = y as isize;

                assert_eq!(TEST_DATA_5X7[y][x], grid[ind]);
            }
        }
    }

    #[test]
    fn test_iter_col() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        for col in 0..TEST_DATA_5X7[0].len() {
            let grid_col_vec: Vec<&usize> = grid.iter_col(col).collect();
            for row in 0..grid_col_vec.len() {
                assert_eq!(TEST_DATA_5X7[row][col], *grid_col_vec[row]);
            }
        }
    }

    #[test]
    fn test_iter_row() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();
        for row in 0..TEST_DATA_5X7.len() {
            let grid_row_vec: Vec<&usize> = grid.iter_row(row).collect();
            for col in 0..grid_row_vec.len() {
                assert_eq!(TEST_DATA_5X7[row][col], *grid_row_vec[col]);
            }
        }
    }
}
