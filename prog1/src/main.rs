use crate::graph::Vertex;
use crate::graph::VertexWeight;
use crate::heap::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
mod graph;
mod heap;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256Plus;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

fn run_trial<const D: usize>(num_vertices: u32, main_flag: u32) -> f32 {
    let vertices: Vec<Vertex<D>> = (0..num_vertices)
        .into_iter()
        .map(Vertex::<D>::new)
        .collect();
    // Use blocks only using adjacent blocks for looking at vertices. put into blocks. look at adjacent blocks in hashmap, and look at closest vertices.
    // put those closest vertices into the map and run algorithm until that heap is finished. add those weights to the overall weight and contineu to the next vertex, while making sure
    // the next's vertex's cloest vertices dos not inlucde the vertices from before.
    let mut heap = BinaryHeap::new();
    let mut map = HashMap::<Vertex<D>, f32>::new();
    let mut total_weight: f32 = 0.0;
    #[allow(unused)]
    let mut upper_bound: f32 = 0.0;
    if D == 0 {
        upper_bound =
            0.05 * (f32::powf(0.54, fast_math::log2_raw(num_vertices as f32) - 7.0) as f32);
    } else if D == 2 {
        upper_bound =
            0.09 * (f32::powf(0.75, fast_math::log2_raw(num_vertices as f32) - 7.0) as f32);
    } else if D == 3 {
        upper_bound =
            0.31 * (f32::powf(0.815, fast_math::log2_raw(num_vertices as f32) - 7.0) as f32);
    } else {
        upper_bound =
            0.42 * (f32::powf(0.855, fast_math::log2_raw(num_vertices as f32) - 7.0) as f32);
    }
    for vertex in &vertices {
        if vertex.id == 0 {
            map.insert(*vertex, 0.0);
        } else {
            map.insert(*vertex, 10.0);
        }
    }
    let _hi = 42;
    let mut max_weight: f32 = 0.0;
    if let Some(root_vertex) = vertices.get(0) {
        heap.insert(VertexWeight::new(*root_vertex, 0.0));
        let to_insert = Arc::new(RwLock::new(Vec::new()));
        let mut rng = Xoshiro256Plus::from_entropy();
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
                if vertex_weight.weight > max_weight {
                    max_weight = vertex_weight.weight;
                }
                map.remove(&vertex_v);
                let f_map = map
                    .keys()
                    .into_iter()
                    .filter_map(|vertex| {
                        let vertex_w = *vertex;
                        let mut weight = 0.0;
                        if D == 0 {
                            weight = rng.gen_range(0.0..=1.0);
                        } else {
                            for i in 0..D {
                                weight += f32::powi(vertex_v.coords[i] - vertex_w.coords[i], 2);
                            }
                            weight = f32::sqrt(weight);
                        }
                        if weight < upper_bound {
                            Some(VertexWeight::new(vertex_w, weight))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let f_map_len = f_map.len();
                // TODO: optimize chunk_size based on map_len
                let chunk_size = if f_map_len > 1024 {
                    f_map_len / 64 + 1
                } else {
                    f_map_len
                };
                if f_map_len != 0 {
                    f_map
                        .par_chunks(chunk_size)
                        .into_par_iter()
                        .for_each(|vertices| {
                            for vw in vertices {
                                let vertex_w = vw.vertex;
                                let weight = vw.weight;
                                if let Some(added_weight) = map.get(&vertex_w) {
                                    if *added_weight > weight {
                                        to_insert.write().unwrap().push((vertex_w, weight));
                                    }
                                }
                            }
                        });
                    for (vertex, weight) in to_insert.read().unwrap().iter() {
                        map.insert(*vertex, *weight);
                        if main_flag == 2 {
                            println!("Heap before insert...");
                            println!("\t{:#?}", &heap);
                        }
                        heap.insert(VertexWeight::new(*vertex, *weight));
                        if main_flag == 2 {
                            println!("Inserted {:?}...", VertexWeight::new(*vertex, *weight));
                            println!("\t{:#?}", &heap);
                        }
                    }
                    to_insert.write().unwrap().clear();
                }
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
            "Beginning {} trial{} for {} vertices of dimension {}...",
            num_trials,
            if num_trials == 1 { "" } else { "s" },
            num_vertices,
            dim
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
