use std::ops::{Index, IndexMut};

use num_traits::float::Float;

#[derive(Debug, Clone)]
pub struct PolygonSoup<T>
where
    T: Float,
{
    vertices: Vec<Vertex<T>>,
    faces: Vec<Face>,
    patches: Vec<String>,
}

impl<T> PolygonSoup<T>
where
    T: Float,
{
    /// Construct a PolygonSoup from its vertices, faces, and patches
    pub fn new(vertices: Vec<Vertex<T>>, faces: Vec<Face>, patches: Vec<String>) -> PolygonSoup<T> {
        PolygonSoup {
            vertices,
            faces,
            patches,
        }
    }

    /// Return the borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<Vertex<T>> {
        &self.vertices
    }

    /// Return the borrowed reference to the faces
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    /// Return the borrowed reference to the patches
    pub fn patches(&self) -> &Vec<String> {
        &self.patches
    }
}

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
    patch: usize,
}

impl Face {
    /// Construct a Face from its vertices and patch
    pub fn new(vertices: Vec<usize>, patch: usize) -> Face {
        Face { vertices, patch }
    }

    /// Return the borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    /// Return the patch
    pub fn patch(&self) -> usize {
        self.patch
    }

    /// Return true if the Face is a triangle
    pub fn is_triangle(&self) -> bool {
        self.vertices.len() == 3
            && self.vertices[0] != self.vertices[1]
            && self.vertices[1] != self.vertices[2]
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
