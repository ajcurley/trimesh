use std::ops::{Index, IndexMut};

use num_traits::float::Float;

#[derive(Debug, Copy, Clone)]
pub struct Vertex<T>
where
    T: Float,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vertex<T>
where
    T: Float,
{
    /// Construct a Vertex from its components
    pub fn new(x: T, y: T, z: T) -> Vertex<T> {
        Vertex { x, y, z }
    }
}

impl<T> Index<usize> for Vertex<T>
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

impl<T> IndexMut<usize> for Vertex<T>
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

#[derive(Debug, Clone)]
pub struct Face {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

impl Face {
    /// Construct a Face from its vertices
    pub fn new(vertices: Vec<usize>) -> Face {
        Face {
            vertices,
            patch: None,
        }
    }

    /// Construct a Face from its vertices and patch
    pub fn with_patch(vertices: Vec<usize>, patch: usize) -> Face {
        Face {
            vertices,
            patch: Some(patch),
        }
    }

    /// Return the borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    /// Return the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }
}

impl Index<usize> for Face {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl IndexMut<usize> for Face {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

#[derive(Debug, Clone)]
pub struct Patch {
    name: String,
}

impl Patch {
    /// Construct a Patch from its name
    pub fn new(name: String) -> Patch {
        Patch { name }
    }

    /// Return the borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for Patch {
    fn default() -> Patch {
        Patch::new("_DEFAULT".to_string())
    }
}
