use crate::graph::elements::Vertex;
use crate::graph::elements::VertexWeight;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BinaryHeap<const D: usize> {
    weights: Vec<VertexWeight<D>>,
    indices: HashMap<Vertex<D>, usize>,
}

impl<const D: usize> BinaryHeap<D> {
    pub fn new() -> Self {
        Self {
            weights: Vec::<VertexWeight<D>>::new(),
            indices: HashMap::<Vertex<D>, usize>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.weights.len()
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

    pub fn insert(&mut self, vw: VertexWeight<D>) {
        match self.indices.get(&vw.vertex) {
            Some(index) => {
                self.weights[*index] = vw;
                self.sort(*index);
            }
            None => {
                let i = self.len();
                self.indices.insert(vw.vertex, i);
                self.weights.push(vw);
                self.sort(i);
            }
        }
    }

    pub fn pop(&mut self) -> Option<VertexWeight<D>> {
        if self.len() == 0 {
            return None;
        } else {
            let last_vw = self.weights.get(self.len() - 1).unwrap().clone();
            self.weights.remove(self.len() - 1);
            if self.len() == 0 {
                self.indices.remove(&last_vw.vertex);
                return Some(last_vw);
            } else {
                let &vw = self.weights.get(0).unwrap();
                let out = vw.clone();
                self.weights[0] = last_vw;
                self.indices.remove(&vw.vertex);
                self.indices.insert(last_vw.vertex, 0);
                self.min_heapify(0);
                return Some(out);
            }
        }
    }

    fn min_heapify(&mut self, i: usize) {
        let l_child_i = self.l_child(i);
        let r_child_i = self.r_child(i);
        let mut min_i = i;
        if l_child_i < self.len() {
            if let Some(vw) = self.weights.get(i) {
                if let Some(l_vw) = self.weights.get(l_child_i) {
                    if l_vw.weight < vw.weight {
                        min_i = l_child_i;
                    }
                }
            }
        }
        if r_child_i < self.len() {
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
