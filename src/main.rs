use crate::graph::Vertex;
use crate::graph::VertexWeight;
use crate::heap::BinaryHeap;
use std::collections::HashMap;
use std::env;
mod graph;
mod heap;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

/*The strategy is to iterate through vertices, mark verfied after removing from min heap, but also  remove it from the linked list */
fn run_trial<const D: usize>(num_vertices: u32, main_flag: u32) -> f32 {
    let vertices: Vec<Vertex<D>> = (0..num_vertices)
        .into_iter()
        .map(Vertex::<D>::new)
        .collect();
    let mut heap = BinaryHeap::new();
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
        heap.insert(VertexWeight::new(*root_vertex, 0.0));
        let mut to_insert = Vec::new();
        while heap.len() != 0 {
            if main_flag == 2 {
                println!("Heap before pop...");
                println!("\t{:#?}", &heap);
            }
            if let Some(vertex_weight) = heap.pop() {
                let vertex_v = &vertex_weight.vertex;
                if main_flag == 2 {
                    println!("Popped {:?}...", vertex_v);
                    println!("\t{:#?}", &heap);
                }
                total_weight += vertex_weight.weight;
                map.remove(&vertex_v);
                for vertex_w in map.keys() {
                    let mut weight = 0.0;
                    if D == 0 {
                        weight = rand::thread_rng().gen_range(0.0..=1.0);
                    } else {
                        for i in 0..D {
                            weight += f32::powi(vertex_v.coords[i] - vertex_w.coords[i], 2);
                        }
                        weight = f32::sqrt(weight);
                    }
                    if let Some(added_weight) = map.get(&vertex_w) {
                        if *added_weight > weight {
                            to_insert.push((*vertex_w, weight));
                            if main_flag == 2 {
                                println!("Heap before insert...");
                                println!("\t{:#?}", &heap);
                            }
                            heap.insert(VertexWeight::new(*vertex_w, weight));
                            if main_flag == 2 {
                                println!("Inserted {:?}...", VertexWeight::new(*vertex_w, weight));
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
    // main_flag 0 is default, 1 is pretty, 2 is debug
    let main_flag: u32 = args[1].trim().parse().expect("Argument is not a number.");
    let num_vertices: u32 = args[2].trim().parse().expect("Argument is not a number.");
    let num_trials: u32 = args[3].trim().parse().expect("Argument is not a number.");
    let dim: u32 = args[4].trim().parse().expect("Argument is not a number.");
    if !matches!(dim, 0 | 2 | 3 | 4) {
        panic!("Unsupported dimension.");
    }
    let now = Instant::now();
    if main_flag == 1 {
        println!(
            "Beginning {} trials for {} vertices of dimension {}...",
            num_trials, num_vertices, dim
        );
    }
    let average_weight = (0..num_trials)
        .into_par_iter()
        .map(|_| match dim {
            0 => run_trial::<0>(num_vertices, main_flag),
            2 => run_trial::<2>(num_vertices, main_flag),
            3 => run_trial::<3>(num_vertices, main_flag),
            4 => run_trial::<4>(num_vertices, main_flag),
            _ => unreachable!(),
        })
        .sum::<f32>()
        / (num_trials as f32);
    let elapsed = now.elapsed();
    if main_flag == 1 {
        println!("\tDone in {:.2?}.", elapsed);
    }
    match main_flag {
        0 => println!("{} {} {} {}", average_weight, num_vertices, num_trials, dim),
        1 => println!(
            "Average weight: {}; Average time: {:2?}; Total time: {:2?}",
            average_weight,
            elapsed.div_f32(num_trials as f32),
            elapsed
        ),
        2 => println!(
            "Average weight: {}; Average time: {:2?}; Total time: {:2?}",
            average_weight,
            elapsed.div_f32(num_trials as f32),
            elapsed
        ),
        _ => (),
    }
}