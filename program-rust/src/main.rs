use crate::graph::elements::Vertex;
use crate::graph::elements::VertexWeight;
use crate::heap::BinaryHeap;
use std::collections::HashMap;
use std::env;
mod graph;
mod heap;

/*The strategy is to iterate through vertices, mark verfied after removing from min heap, but also  remove it from the linked list */
fn run_trial<const D: usize>(num_vertices: u32, main_flag: u32) -> f32 {
    let vertices: Vec<Vertex<D>> = (0..num_vertices)
        .into_iter()
        .map(Vertex::<D>::create)
        .collect();
    let mut heap = BinaryHeap::new(num_vertices as usize);
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
        heap.insert(VertexWeight::create(*root_vertex, 0.0));
        let mut to_insert = Vec::new();
        while heap.len() != 0 {
            if main_flag == 1 {
                println!("Heap before pop...");
                println!("\t{:#?}", &heap);
            }
            if let Some(vertex_weight) = heap.pop() {
                let vertex_v = &vertex_weight.vertex;
                if main_flag == 1 {
                    println!("Popped {:?}...", vertex_v);
                    println!("\t{:#?}", &heap);
                }
                total_weight += vertex_weight.weight;
                map.remove(&vertex_v);
                for vertex_w in map.keys() {
                    let mut weight = 0.0;
                    if D == 0 {
                        weight += (vertex_v.val + vertex_w.val) / 2.0;
                    } else {
                        for i in 0..D {
                            weight += f32::powi(vertex_v.coords[i] - vertex_w.coords[i], 2);
                        }
                        weight = f32::sqrt(weight);
                    }
                    if let Some(added_weight) = map.get(&vertex_w) {
                        if *added_weight > weight {
                            to_insert.push((*vertex_w, weight));
                            if main_flag == 1 {
                                println!("Heap before insert...");
                                println!("\t{:#?}", &heap);
                            }
                            heap.insert(VertexWeight::create(*vertex_w, weight));
                            if main_flag == 1 {
                                println!(
                                    "Inserted {:?}...",
                                    VertexWeight::create(*vertex_w, weight)
                                );
                                println!("\t{:#?}", &heap);
                            }
                        }
                    }
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
    let main_flag: u32 = args[1].trim().parse().expect("Argument is not a number.");
    let num_vertices: u32 = args[2].trim().parse().expect("Argument is not a number.");
    let num_trials: u32 = args[3].trim().parse().expect("Argument is not a number.");
    let dim: u32 = args[4].trim().parse().expect("Argument is not a number.");
    let mut total_weight: f32 = 0.0;
    for _ in 0..num_trials {
        if dim == 0 {
            total_weight += run_trial::<0>(num_vertices, main_flag);
        } else if dim == 1 {
            total_weight += run_trial::<1>(num_vertices, main_flag);
        } else if dim == 2 {
            total_weight += run_trial::<2>(num_vertices, main_flag);
        } else if dim == 3 {
            total_weight += run_trial::<3>(num_vertices, main_flag);
        } else if dim == 4 {
            total_weight += run_trial::<4>(num_vertices, main_flag);
        } else {
            total_weight += run_trial::<1>(num_vertices, main_flag);
        }
    }
    println!("Average: {}", total_weight / (num_trials as f32));
}
