use rand::Rng;
// use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct Vertex<const D: usize> {
    pub id: u32,
    pub coords: [f32; D],
    pub val: f32,
}

impl<const D: usize> Vertex<D> {
    pub fn create(id: u32) -> Self {
        if D == 0 {
            let coords = [0.0; D];
            let val = rand::thread_rng().gen_range(0.0..=1.0);
            Self { id, coords, val }
        } else {
            let mut coords = [0.0; D];
            let val = 0.0;
            for i in 0..D {
                coords[i] = rand::thread_rng().gen_range(0.0..=1.0);
            }
            Self { id, coords, val }
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
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VertexWeight<const D: usize> {
    pub vertex: Vertex<D>,
    pub weight: f32,
}

impl<const D: usize> VertexWeight<D> {
    pub fn create(vertex: Vertex<D>, weight: f32) -> Self {
        Self { vertex, weight }
    }
}

// impl<const D: usize> Eq for VertexWeight<D> {}

// impl<const D: usize> PartialEq for VertexWeight<D> {
//     fn eq(&self, other: &Self) -> bool {
//         self.vertex == other.vertex
//     }
// }

// impl<const D: usize> Ord for VertexWeight<D> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.weight.total_cmp(&self.weight)
//         // self.weight.total_cmp(&other.weight)
//     }
// }

// impl<const D: usize> PartialOrd for VertexWeight<D> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         other.weight.partial_cmp(&self.weight)
//         // self.weight.partial_cmp(&other.weight)
//     }
// }

#[derive(Debug)]
pub struct BinaryHeap<const D: usize> {
    len: usize,
    weights: Vec<VertexWeight<D>>,
    indices: HashMap<Vertex<D>, usize>,
}

impl<const D: usize> BinaryHeap<D> {
    pub fn new(n: usize) -> Self {
        Self {
            len: 0,
            weights: Vec::<VertexWeight<D>>::with_capacity(n),
            indices: HashMap::<Vertex<D>, usize>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn l_child(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn r_child(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn sort(&mut self, i: usize) {
        let mut j = i;
        if j != 0 {
            while j != 0 {
                if let Some(vw) = self.weights.get(j) {
                    if let Some(p_vw) = self.weights.get(self.parent(j)) {
                        if p_vw.weight > vw.weight {
                            let p_index = self.parent(j);
                            self.indices.insert(vw.vertex, p_index);
                            self.indices.insert(p_vw.vertex, j);
                            self.weights.swap(p_index, j);
                            j = p_index;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, vw: VertexWeight<D>) {
        match self.indices.get(&vw.vertex) {
            Some(index) => {
                if let Some(found_vw) = self.weights.get_mut(*index) {
                    *found_vw = vw;
                    self.sort(*index);
                }
            }
            None => {
                let i = self.len;
                self.len += 1;
                self.indices.insert(vw.vertex, i);
                self.weights.push(vw);
                self.sort(i);
            }
        }
    }

    pub fn pop(&mut self) -> Option<VertexWeight<D>> {
        if self.len == 0 {
            return None;
        } else {
            self.len -= 1;
            let last_vw = self.weights.get(self.len).unwrap().clone();
            let vw = self.weights.get_mut(0).unwrap();
            let out = vw.clone();
            self.indices.remove(&vw.vertex);
            *vw = last_vw;
            self.indices.insert(last_vw.vertex, 0);
            self.min_heapify(0);
            return Some(out);
        }
    }

    fn min_heapify(&mut self, i: usize) {
        let l_child_i = self.l_child(i);
        let r_child_i = self.r_child(i);
        let mut min_i = i;
        if l_child_i > self.len {
            if let Some(vw) = self.weights.get(i) {
                if let Some(l_vw) = self.weights.get(l_child_i) {
                    if l_vw.weight < vw.weight {
                        min_i = l_child_i;
                    }
                }
            }
        }
        if r_child_i > self.len {
            if let Some(vw) = self.weights.get(min_i) {
                if let Some(r_vw) = self.weights.get(r_child_i) {
                    if r_vw.weight < vw.weight {
                        min_i = r_child_i;
                    }
                }
            }
        }
        if min_i != i {
            if let Some(vw) = self.weights.get(i) {
                if let Some(min_vw) = self.weights.get(min_i) {
                    self.indices.insert(vw.vertex, min_i);
                    self.indices.insert(min_vw.vertex, i);
                    self.weights.swap(min_i, i);
                }
            }
            self.min_heapify(min_i);
        }
    }
}
