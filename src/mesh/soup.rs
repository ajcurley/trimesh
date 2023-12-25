use num_traits::float::Float;

use crate::mesh::exchange::ObjImporter;
use crate::mesh::primitives::{Face, Patch, Vertex};
use crate::mesh::MeshImporter;

#[derive(Debug, Clone)]
pub struct PolygonSoup<T>
where
    T: Float,
{
    vertices: Vec<Vertex<T>>,
    faces: Vec<Face>,
    patches: Vec<Patch>,
}

impl<T> PolygonSoup<T>
where
    T: Float,
{
    /// Construct a PolygonSoup from its vertices, faces, and patches
    pub fn new(vertices: Vec<Vertex<T>>, faces: Vec<Face>, patches: Vec<Patch>) -> PolygonSoup<T> {
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

    /// Insert a vertex
    pub fn insert_vertex(&mut self, vertex: Vertex<T>) {
        self.vertices.push(vertex);
    }

    /// Return the borrowed reference to the faces
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    /// Insert a face
    pub fn insert_face(&mut self, face: Face) {
        self.faces.push(face);
    }

    /// Return the borrowed reference to the patches
    pub fn patches(&self) -> &Vec<Patch> {
        &self.patches
    }

    /// Insert a patch
    pub fn insert_patch(&mut self, patch: Patch) {
        self.patches.push(patch);
    }
}

impl<T> PolygonSoup<T>
where
    T: Float + std::str::FromStr<Err = T>,
{
    /// Import from an OBJ (WaveFront) file
    pub fn from_obj(path: &str) -> std::io::Result<PolygonSoup<T>> {
        ObjImporter::new(path).import()
    }
}

impl<T> Default for PolygonSoup<T>
where
    T: Float,
{
    fn default() -> PolygonSoup<T> {
        PolygonSoup::<T>::new(vec![], vec![], vec![])
    }
}
