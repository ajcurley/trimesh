use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use num_traits::float::Float;

#[derive(Debug, Copy, Clone)]
pub struct Vector3<T>
where
    T: Float,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: Float,
{
    /// Construct a Vector3 from its components
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    /// Compute the vector dot product u * v
    pub fn dot(u: &Vector3<T>, v: &Vector3<T>) -> T {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Compute the vector cross product u x v
    pub fn cross(u: &Vector3<T>, v: &Vector3<T>) -> Vector3<T> {
        let i = u.y * v.z - u.z * v.y;
        let j = v.x * u.z - u.z * v.x;
        let k = u.x * v.y - u.y * v.z;

        Vector3::<T>::new(i, j, k)
    }

    /// Compute the magnitude (L2-norm)
    pub fn mag(&self) -> T {
        Vector3::<T>::dot(&self, &self).sqrt()
    }

    /// Compute the unit Vector3
    pub fn unit(&self) -> Vector3<T> {
        *self / self.mag()
    }
}

impl Vector3<f32> {
    /// Construct a Vector3<f32> of all zeros
    pub fn zeros() -> Vector3<f32> {
        Vector3::<f32>::new(0., 0., 0.)
    }

    /// Construct a Vector3<f32> of all ones
    pub fn ones() -> Vector3<f32> {
        Vector3::<f32>::new(1., 1., 1.)
    }
}

impl Vector3<f64> {
    /// Construct a Vector3<f64> of all zeros
    pub fn zeros() -> Vector3<f64> {
        Vector3::<f64>::new(0., 0., 0.)
    }

    /// Construct a Vector3<f64> of all ones
    pub fn ones() -> Vector3<f64> {
        Vector3::<f64>::new(1., 1., 1.)
    }
}

impl<T> Index<usize> for Vector3<T>
where
    T: Float,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn add(self, v: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<T> Add<T> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn add(self, v: T) -> Self::Output {
        Vector3 {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v,
        }
    }
}

impl<T> AddAssign<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    fn add_assign(&mut self, v: Vector3<T>) {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
        self.z = self.z + v.z;
    }
}

impl<T> AddAssign<T> for Vector3<T>
where
    T: Float,
{
    fn add_assign(&mut self, v: T) {
        self.x = self.x + v;
        self.y = self.y + v;
        self.z = self.z + v;
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn sub(self, v: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl<T> Sub<T> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn sub(self, v: T) -> Self::Output {
        Vector3 {
            x: self.x - v,
            y: self.y - v,
            z: self.z - v,
        }
    }
}

impl<T> SubAssign<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    fn sub_assign(&mut self, v: Vector3<T>) {
        self.x = self.x - v.x;
        self.y = self.y - v.y;
        self.z = self.z - v.z;
    }
}

impl<T> SubAssign<T> for Vector3<T>
where
    T: Float,
{
    fn sub_assign(&mut self, v: T) {
        self.x = self.x - v;
        self.y = self.y - v;
        self.z = self.z - v;
    }
}

impl<T> Mul<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn mul(self, v: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn mul(self, v: T) -> Self::Output {
        Vector3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}

impl<T> MulAssign<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    fn mul_assign(&mut self, v: Vector3<T>) {
        self.x = self.x * v.x;
        self.y = self.y * v.y;
        self.z = self.z * v.z;
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: Float,
{
    fn mul_assign(&mut self, v: T) {
        self.x = self.x * v;
        self.y = self.y * v;
        self.z = self.z * v;
    }
}

impl<T> Div<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn div(self, v: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn div(self, v: T) -> Self::Output {
        Vector3 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}

impl<T> DivAssign<Vector3<T>> for Vector3<T>
where
    T: Float,
{
    fn div_assign(&mut self, v: Vector3<T>) {
        self.x = self.x / v.x;
        self.y = self.y / v.y;
        self.z = self.z / v.z;
    }
}

impl<T> DivAssign<T> for Vector3<T>
where
    T: Float,
{
    fn div_assign(&mut self, v: T) {
        self.x = self.x / v;
        self.y = self.y / v;
        self.z = self.z / v;
    }
}

impl<T> Neg for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
