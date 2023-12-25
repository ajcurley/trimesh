use std::ops::{Index, IndexMut};

use num_traits::float::Float;

use crate::geometry::{BoundingBox, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Triangle<T>
where
    T: Float,
{
    p: Vector3<T>,
    q: Vector3<T>,
    r: Vector3<T>,
}

impl<T> Triangle<T>
where
    T: Float,
{
    /// Construct a Triangle from its vertices
    pub fn new(p: Vector3<T>, q: Vector3<T>, r: Vector3<T>) -> Triangle<T> {
        Triangle { p, q, r }
    }

    /// Compute the unit normal
    pub fn normal(&self) -> Vector3<T> {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::<T>::cross(&u, &v).unit()
    }

    /// Compute the BoundingBox
    pub fn bbox(&self) -> BoundingBox<T> {
        let mut min = self.p;
        let mut max = self.p;

        for i in 0..3 {
            min[i] = min[i].min(self.q[i]).min(self.r[i]);
            max[i] = max[i].max(self.q[i]).max(self.r[i]);
        }

        BoundingBox::<T>::new(min, max)
    }
}

impl Triangle<f32> {
    /// Compute the area
    pub fn area(&self) -> f32 {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::<f32>::cross(&u, &v).mag() * 0.5_f32
    }
}

impl Triangle<f64> {
    /// Compute the area
    pub fn area(&self) -> f64 {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::<f64>::cross(&u, &v).mag() * 0.5_f64
    }
}

impl<T> Index<usize> for Triangle<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            2 => &self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl<T> IndexMut<usize> for Triangle<T>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            2 => &mut self.r,
            _ => panic!("index out of range"),
        }
    }
}
