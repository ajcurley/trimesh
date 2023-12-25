use num_traits::float::Float;

use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox<T>
where
    T: Float,
{
    min: Vector3<T>,
    max: Vector3<T>,
}

impl<T> BoundingBox<T>
where
    T: Float,
{
    /// Construct a BoundingBox from its min/max bounds
    pub fn new(min: Vector3<T>, max: Vector3<T>) -> BoundingBox<T> {
        BoundingBox { min, max }
    }

    /// Return the min bound
    pub fn min(&self) -> Vector3<T> {
        self.min
    }

    /// Return the max bound
    pub fn max(&self) -> Vector3<T> {
        self.max
    }

    /// Compute the size
    pub fn size(&self) -> Vector3<T> {
        self.max - self.min
    }
}

impl BoundingBox<f32> {
    /// Compute the center
    pub fn center(&self) -> Vector3<f32> {
        (self.max + self.min) * 0.5_f32
    }

    /// Compute the half size
    pub fn halfsize(&self) -> Vector3<f32> {
        (self.max - self.min) * 0.5_f32
    }
}

impl BoundingBox<f64> {
    /// Compute the center
    pub fn center(&self) -> Vector3<f64> {
        (self.max + self.min) * 0.5_f64
    }

    /// Compute the half size
    pub fn halfsize(&self) -> Vector3<f64> {
        (self.max - self.min) * 0.5_f64
    }
}
