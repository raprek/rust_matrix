use std::{
    mem,
    ops::{Add, AddAssign, Deref, IndexMut, Mul, MulAssign},
};

#[derive(Debug)]
pub enum Error {
    InvalidLen,
}

#[derive(Debug)]
pub struct Matrix<const N: usize, T> {
    elements: Vec<T>,
}

impl<const N: usize, T> Matrix<N, T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(N),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Result<Self, Error> {
        if vec.len() % N != 0 {
            return Err(Error::InvalidLen);
        } else {
            Ok(Self { elements: vec })
        }
    }
}

impl<const N: usize, T> Add<T> for Matrix<N, T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self {
            elements: self
                .elements
                .into_iter()
                .map(|x| x + other.clone())
                .collect::<Vec<T>>(),
        }
    }
}

impl<const N: usize, T> Mul<T> for Matrix<N, T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self {
            elements: self
                .elements
                .into_iter()
                .map(|x| x * other.clone())
                .collect::<Vec<T>>(),
        }
    }
}
