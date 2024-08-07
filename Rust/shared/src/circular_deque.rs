use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

pub struct CircularDeque<T> {
    inner: VecDeque<T>,
    head: usize,
}

impl<T> CircularDeque<T> {
    pub fn new() -> Self {
        return CircularDeque {
            inner: VecDeque::<T>::new(),
            head: 0,
        };
    }

    // Adds new value left of head
    pub fn add_last(&mut self, val: T) {
        self.inner.insert(self.head, val);
        self.head += 1;
        self.head %= self.inner.len();
    }

    // Adds new value at head, pushing current head right
    pub fn add_first(&mut self, val: T) {
        self.inner.insert(self.head, val)
    }

    pub fn swap(&mut self, a: isize, b: isize) {
        let mut a = a;
        let mut b = b;
        while a < 0 {
            a += self.inner.len() as isize;
        }
        while b < 0 {
            b += self.inner.len() as isize;
        }

        self.inner.swap(a as usize, b as usize);
    }

    pub fn head(&self) -> usize {
        return self.head;
    }

    pub fn len(&self) -> usize {
        return self.inner.len();
    }

    pub fn move_head_right(&mut self, steps: usize) {
        self.head = (self.head + steps) % self.inner.len();
    }

    pub fn move_head_left(&mut self, steps: usize) {
        let steps = steps % self.inner.len();
        self.head = (self.head + self.inner.len() - steps) % self.inner.len();
    }

    /// reverses elements from head..(head+len)
    pub fn reverse_segment(&mut self, len: usize) {
        let mut h = self.head;
        let mut l = (self.head + len) % self.inner.len();
        loop {
            self.inner.swap(h, l);
            h += 1;
            l -= 1;
            if h >= l {
                break;
            }
        }
    }

    pub fn first(&self) -> &T {
        return &self.inner[self.head % self.inner.len()];
    }

    pub fn get(&self, ind: usize) -> &T {
        let i = (self.head + ind) % self.inner.len();
        return &self.inner[i];
    }

    pub fn remove(&mut self, ind: usize) -> T {
        let ind = (self.head + ind) % self.inner.len();
        if ind < self.head {
            self.head -= 1;
        }
        return self.inner.remove(ind).unwrap();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut indexes = (self.head..self.inner.len()).collect::<Vec<usize>>();
        indexes.extend(0..self.head);
        let mut i = 0;
        std::iter::from_fn(move || {
            i += 1;
            if i > indexes.len() {
                return None;
            }
            return Some(&self.inner[indexes[i - 1]]);
        })
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::<T>::with_capacity(self.inner.len());
        for i in self.head..self.inner.len() {
            result.push(self.inner[i].clone());
        }
        for i in 0..self.head {
            result.push(self.inner[i].clone());
        }
        return result;
    }

    /// Inserts new value at index, moving all values with indexes equal to
    /// or higher to the right
    pub fn insert(&mut self, index: usize, value: T) {
        let index = (index + self.head) % self.inner.len();
        self.inner.insert(index, value);
        if index < self.head {
            self.head += 1;
            self.head %= self.inner.len();
        }
    }

    pub fn first_index_of(&self, val: T) -> Option<usize>
    where
        T: PartialEq,
    {
        for i in self.head..self.inner.len() {
            if self.inner[i] == val {
                return Some((i - self.head) % self.inner.len());
            }
        }
        for i in 0..self.head {
            if self.inner[i] == val {
                return Some((i + self.head) % self.inner.len());
            }
        }

        return None;
    }
}

impl<T> Index<usize> for CircularDeque<T> {
    type Output = T;

    fn index<'a>(&'a self, ind: usize) -> &'a T {
        return self.get(ind);
    }
}

impl<T> IndexMut<usize> for CircularDeque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = (self.head + index) % self.inner.len();
        return &mut self.inner[i];
    }
}
