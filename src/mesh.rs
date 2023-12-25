use num_traits::float::Float;

pub mod exchange;
pub mod primitives;
pub mod soup;

// Re-exports
pub use soup::PolygonSoup;

pub trait MeshImporter<T>
where
    T: Float + std::str::FromStr<Err = T>,
{
    fn import(&self) -> std::io::Result<PolygonSoup<T>>;
}
