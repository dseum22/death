use core::num;
use rand::Rng;
use std::cmp::Ordering;
// use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
struct Vertex<const D: usize> {
    id: u32,
    coords: [f32; D],
}

impl<const D: usize> Vertex<D> {
    fn create(id: u32) -> Self {
        let mut coords = [0.0; D];
        for i in 0..D {
            coords[i] = rand::thread_rng().gen_range(0.0..=1.0);
        }
        Self { id, coords }
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
        self.id.hash(state);
    }
}

struct VertexWeight<const D: usize> {
    vertex: Vertex<D>,
    weight: f32,
}

impl<const D: usize> VertexWeight<D> {
    fn create(vertex: Vertex<D>, weight: f32) -> Self {
        Self { vertex, weight }
    }
}
impl<const D: usize> Eq for VertexWeight<D> {}

impl<const D: usize> PartialEq for VertexWeight<D> {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

impl<const D: usize> Ord for VertexWeight<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.total_cmp(&other.weight)
    }
}

impl<const D: usize> PartialOrd for VertexWeight<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

/*The strategy is to iterate through vertices, mark verfied after removing from min heap, but also  remove it from the linked list */
fn run_trial<const D: usize>(num_vertices: u32) -> f32 {
    let vertices: Vec<Vertex<D>> = (0..num_vertices)
        .into_iter()
        .map(Vertex::<D>::create)
        .collect();
    let mut heap = BinaryHeap::<VertexWeight<D>>::new();
    let mut map = HashMap::<Vertex<D>, f32>::new();
    let mut total_weight: f32 = 0.0;
    for vertex in &vertices {
        if vertex.id == 0 {
            map.insert(*vertex, 0.0);
        } else {
            map.insert(*vertex, 10.0);
        }
    }
    if let Some(root_vertex) = vertices.get(0) {
        heap.push(VertexWeight::create(*root_vertex, 0.0));
        let mut to_insert = Vec::new();
        while heap.len() != 0 {
            if let Some(vertex_weight) = heap.pop() {
                let vertex_v = &vertex_weight.vertex;
                total_weight += f32::sqrt(vertex_weight.weight);
                map.remove(&vertex_v);
                for vertex_w in map.keys() {
                    let mut weight = 0.0;
                    for i in 0..D {
                        weight += f32::powi(vertex_v.coords[i] - vertex_w.coords[i], 2);
                    }
                    if let Some(added_weight) = map.get(&vertex_w) {
                        if added_weight > &weight {
                            to_insert.push((*vertex_w, weight));
                            heap.push(VertexWeight::create(*vertex_w, weight));
                        }
                    }
                    // do sqrt at the veryend
                }
                for (vertex, weight) in &to_insert {
                    map.insert(*vertex, *weight);
                }
                to_insert.clear();
            }
        }
    }
    total_weight
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let env: u32 = args[1].trim().parse().expect("Argument is not a number.");
    let num_vertices: u32 = args[2].trim().parse().expect("Argument is not a number.");
    let num_trials: u32 = args[3].trim().parse().expect("Argument is not a number.");
    let dim: u32 = args[3].trim().parse().expect("Argument is not a number.");
    let mut total_weight: f32 = 0.0;
    for _ in 0..num_trials {
        total_weight += run_trial::<2>(num_vertices);
    }
    println!("Average: {}", total_weight / (num_trials as f32));
}
