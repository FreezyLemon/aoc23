use std::ops::{Index, IndexMut};

use super::Position;

// trait Coord :
//     TryFrom<usize> +
//     PartialOrd +
//     std::ops::Add<Output = Self> +
//     std::ops::Mul<Output = Self> +
//     std::ops::Div<Output = Self> +
// {}

// TODO: Consider making this generic
type C = u32;

pub struct Cartesian<T> {
    data: Vec<T>,
    cols: C,
}

impl<T> Cartesian<T>
where
    <C as TryFrom<usize>>::Error : core::fmt::Debug,
    usize : TryFrom<C>
{
    pub fn new(data: Vec<T>, cols: C) -> Self {
        Self {
            data,
            cols
        }
    }

    pub fn rows(&self) -> C {
        C::try_from(self.data.len()).expect("len fits into C") / self.cols
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        if pos.col >= self.cols {
            None
        } else if pos.row >= self.rows() {
            None
        } else {
            Some(&self.data[usize::try_from(pos.row * self.cols + pos.col).unwrap()])
        }
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        if pos.col >= self.cols {
            None
        } else if pos.row >= self.rows() {
            None
        } else {
            Some(&mut self.data[usize::try_from(pos.row * self.cols + pos.col).unwrap()])
        }
    }
}

impl<T> Index<Position> for Cartesian<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index).expect("is within bounds")
    }
}

impl<T> IndexMut<Position> for Cartesian<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.get_mut(index).expect("is within bounds")
    }
}
