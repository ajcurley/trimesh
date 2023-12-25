use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use flate2::read::GzDecoder;
use num_traits::float::Float;

use crate::mesh::primitives::{Face, Patch, Vertex};
use crate::mesh::MeshImporter;
use crate::mesh::PolygonSoup;

#[derive(Debug, Clone)]
pub struct ObjImporter<'a> {
    path: &'a str,
}

impl<'a> ObjImporter<'a> {
    /// Construct an ObjImporter from its path
    pub fn new(path: &'a str) -> ObjImporter<'a> {
        ObjImporter { path }
    }

    /// Return true if the path is a GZIP file
    pub fn is_gzip(&self) -> bool {
        if let Some(ext) = Path::new(self.path).extension().and_then(OsStr::to_str) {
            return ext.to_lowercase() == "gz";
        }

        false
    }

    /// Read the contents of the file to string
    fn read_to_string(&self) -> std::io::Result<String> {
        let mut file = File::open(self.path)?;
        let mut contents = String::new();

        if self.is_gzip() {
            GzDecoder::new(file).read_to_string(&mut contents)?;
        } else {
            file.read_to_string(&mut contents)?;
        }

        Ok(contents)
    }

    /// Parse a vertex from an entry
    fn parse_vertex<T>(&self, args: &[&str], soup: &mut PolygonSoup<T>)
    where
        T: Float + std::str::FromStr<Err = T>,
    {
        let mut values: Vec<T> = vec![];

        for value in args.iter() {
            if let Ok(v) = value.parse::<T>() {
                values.push(v);
            }
        }

        if values.len() != 3 {
            panic!("vertex must be three-dimension");
        }

        let vertex = Vertex::<T>::new(values[0], values[1], values[2]);
        soup.insert_vertex(vertex);
    }

    /// Parse a face from an entry
    fn parse_face<T>(&self, args: &[&str], soup: &mut PolygonSoup<T>)
    where
        T: Float + std::str::FromStr<Err = T>,
    {
        let mut values: Vec<usize> = vec![];

        for value in args.iter() {
            if let Ok(v) = value.parse::<usize>() {
                values.push(v - 1);
            }
        }

        if values.len() < 3 {
            panic!("face must have at least 3 vertices");
        }

        if soup.patches().len() == 0 {
            soup.insert_patch(Patch::default());
        }

        let patch = soup.patches().len();
        let face = Face::with_patch(values, patch);
        soup.insert_face(face);
    }

    /// Parse a group from an entry
    fn parse_group<T>(&self, args: &[&str], soup: &mut PolygonSoup<T>)
    where
        T: Float + std::str::FromStr<Err = T>,
    {
        let name = args.join(" ");
        let patch = Patch::new(name);
        soup.insert_patch(patch);
    }
}

impl<'a, T> MeshImporter<T> for ObjImporter<'a>
where
    T: Float + std::str::FromStr<Err = T>,
{
    fn import(&self) -> std::io::Result<PolygonSoup<T>> {
        let mut soup = PolygonSoup::<T>::default();

        for line in self.read_to_string()?.lines() {
            let args: Vec<&str> = line.trim().split_whitespace().collect();

            match args.first() {
                Some(&"v") => self.parse_vertex(&args[1..], &mut soup),
                Some(&"f") => self.parse_face(&args[1..], &mut soup),
                Some(&"g") => self.parse_group(&args[1..], &mut soup),
                _ => {}
            }
        }

        Ok(soup)
    }
}
