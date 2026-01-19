use ordered_float::OrderedFloat;
#[warn(unused)]
// #[warn(dead_code)]
use rayon::prelude::*;
use rayon::vec;
use std::arch::naked_asm;
use std::cmp::{Ordering, min};

use rand::Rng;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

// const N

pub struct VectorStore {
    data: Vec<f32>,
    dim: usize,
}

impl VectorStore {
    fn new(n: usize, dim: usize) -> Self {
        Self {
            data: Vec::with_capacity(n * dim),
            dim,
        }
    }

    fn insert(&mut self, vec: &[f32]) -> usize {
        let id = self.data.len() / self.dim;
        self.data.extend_from_slice(vec);
        id
    }

    fn squared_distance(&self, v1_id: usize, v2_id: usize) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        let vec2 = &self.data[v2_id * self.dim..(v2_id + 1) * self.dim];
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum()
    }

    fn squared_distance_to_query(&self, v1_id: usize, query: &[f32]) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        vec1.iter()
            .zip(query.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum()
    }
}

pub struct GraphLayers {
    base_layer: Vec<Vec<usize>>,
    upper_layers: Vec<HashMap<usize, Vec<usize>>>,
}

struct Distibution {}

impl GraphLayers {
    fn add_neighbors(&mut self) {
        // TO-DO
    }

    fn add_edge(&mut self, node_id_1: usize, node_id_2: usize, layer: usize, d: bool) {
        // Check if thye both exsist in the same layer ? ! :(
        if layer > 0 {
            // Handle the upper layer addedge logic ! i GUESS WE DONT NEED THIS
            // self.upper_layers[layer-1].get(&node_id_1);
        } else {
            self.base_layer[node_id_1].push(node_id_2);
            if !d {
                self.base_layer[node_id_2].push(node_id_1);
            }
        }
    }

    fn insert_node(&mut self) -> usize {
        let node_id = self.base_layer.len(); // index + 1 
        self.base_layer.push(Vec::new());
        node_id
    }

    fn insert_node_upper_layers(&mut self, layer: usize, node_id: usize) {
        self.upper_layers[layer - 1].insert(node_id, Vec::new());
    }

    fn get_neighbors(&self, layer: usize, node_id: usize) -> &[usize] {
        if layer == 0 {
            return &self.base_layer[node_id];
        } else {
            if layer <= self.upper_layers.len() {
                match self.upper_layers[layer - 1].get(&node_id) {
                    Some(slice) => slice,
                    None => &[],
                }
            } else {
                return &[];
            }
        }
    }

    fn new() -> Self {
        // Distrubution of nodes in the upper layer by exponetioal decay probalistic function
        // Choose random nodes and promote them to upper layers
        todo!()
    }
}

pub struct HNSW {
    pub layers: GraphLayers,
    pub vectors: VectorStore,
    pub entry_point: Option<usize>,
    pub max_level: usize,
    pub ef_construction: usize,
    pub m: usize,
}

impl HNSW {
    

    // greedy beam search
    pub fn search_layer(&self, q: &[f32], lc: usize) -> BinaryHeap<(OrderedFloat<f32>, usize)> {
        let ep = self.entry_point.expect("ENTRY POINT ERROR");
        let sq_dist = VectorStore::squared_distance_to_query(&self.vectors, ep, q);
        // Candidates is Min Que
        let mut candidates: BinaryHeap<Reverse<(OrderedFloat<f32>, usize)>> = BinaryHeap::new(); // (Dist , node_id)
        // Found Neighbors is Max Que
        let mut found_neighbours: BinaryHeap<(OrderedFloat<f32>, usize)> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();

        visited.insert(ep);
        candidates.push(Reverse((OrderedFloat(sq_dist), ep)));
        found_neighbours.push((OrderedFloat(sq_dist), ep));

        while !candidates.is_empty() {
            let Reverse((OrderedFloat(dist_c), closest_candidate)) = candidates.pop().unwrap();

            let (OrderedFloat(dist_worst), furthest_element) = *found_neighbours.peek().unwrap();

            if dist_c > dist_worst {
                break;
            }

            for e in GraphLayers::get_neighbors(&self.layers, lc, closest_candidate) {
                if !visited.contains(e) {
                    let dist_e = VectorStore::squared_distance_to_query(&self.vectors, *e, q);
                    let dist_e_wrapped = OrderedFloat(dist_e);

                    let (OrderedFloat(current_worst_dist), _) = *found_neighbours.peek().unwrap();
                    visited.insert(*e);
                    if dist_e < current_worst_dist || found_neighbours.len() < self.ef_construction
                    {
                        candidates.push(Reverse((OrderedFloat(dist_e), *e)));
                        found_neighbours.push((OrderedFloat(dist_e), *e));
                        if found_neighbours.len() > self.ef_construction {
                            found_neighbours.pop();
                        }
                    }
                }
            }
        }
        found_neighbours
        // found_neighbours.into_iter().map(|(_, idx)| idx).collect()
    }

    fn select_neighbors_simple(_q: &[f32], c: BinaryHeap<Reverse<(OrderedFloat<f32>, usize)>>, m: usize) -> Vec<usize> {
        let mut candidates = c.into_vec();

    
        if candidates.len() <= m {
            candidates.sort_unstable(); 
            return candidates.into_iter().map(|Reverse((_, id))| id).collect();
        }

        candidates.select_nth_unstable(m);
        candidates.truncate(m);

        candidates
            .into_iter()
            .map(|Reverse((_, id))| id)
            .collect()
    }
}
