use rand::Rng;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct Vertex<const D: usize> {
    pub id: u32,
    pub coords: [f32; D],
}

impl<const D: usize> Vertex<D> {
    pub fn new(id: u32) -> Self {
        if D == 0 {
            let coords = [0.0; D];
            Self { id, coords }
        } else {
            let mut coords = [0.0; D];
            let mut rng = rand::thread_rng();
            for i in 0..D {
                coords[i] = rng.gen_range(0.0..=1.0);
            }
            Self { id, coords }
        }
    }
}

impl<const D: usize> Eq for Vertex<D> {}

impl<const D: usize> PartialEq for Vertex<D> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<const D: usize> Hash for Vertex<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VertexWeight<const D: usize> {
    pub vertex: Vertex<D>,
    pub weight: f32,
}

impl<const D: usize> VertexWeight<D> {
    pub fn new(vertex: Vertex<D>, weight: f32) -> Self {
        Self { vertex, weight }
    }
}
