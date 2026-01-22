use ordered_float::OrderedFloat;
use rayon::prelude::*;
// use serde::{Serialize, Deserialize};
use rkyv::*;
use std::cmp::{Ordering, min};
use rand::Rng;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
// Expreimenting 
use rkyv::{deserialize, Deserialize, rancor::Error, Archive, Serialize};


#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct VectorStore {
    data: Vec<f32>,
    dim: usize,
}

impl VectorStore {
    pub fn new(n: usize, dim: usize) -> Self {
        Self {
            data: Vec::with_capacity(n * dim),
            dim,
        }
    }
    
    pub fn insert(&mut self, vec: &[f32]) -> usize {
        let id = self.data.len() / self.dim;
        self.data.extend_from_slice(vec);
        id
    }
    
    fn squared_distance(&self, v1_id: usize, v2_id: usize) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        let vec2 = &self.data[v2_id * self.dim..(v2_id + 1) * self.dim];
        
        let mut sum = 0.0;
        let chunks1 = vec1.chunks_exact(8);
        let chunks2 = vec2.chunks_exact(8);
        let rem1 = chunks1.remainder();
        let rem2 = chunks2.remainder();
        
        for (a, b) in chunks1.zip(chunks2) {
            let mut sub_sum = 0.0;
            for i in 0..8 {
                let diff = a[i] - b[i];
                sub_sum += diff * diff;
            }
            sum += sub_sum;
        }
        
        for (a, b) in rem1.iter().zip(rem2.iter()) {
            let diff = a - b;
            sum += diff * diff;
        }
        sum
    }
    
    pub fn squared_distance_to_query(&self, v1_id: usize, query: &[f32]) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        
        let mut sum = 0.0;
        let chunks1 = vec1.chunks_exact(8);
        let chunks2 = query.chunks_exact(8);
        let rem1 = chunks1.remainder();
        let rem2 = chunks2.remainder();
        
        for (a, b) in chunks1.zip(chunks2) {
            let mut sub_sum = 0.0;
            for i in 0..8 {
                let diff = a[i] - b[i];
                sub_sum += diff * diff;
            }
            sum += sub_sum;
        }
        
        for (a, b) in rem1.iter().zip(rem2.iter()) {
            let diff = a - b;
            sum += diff * diff;
        }
        sum
    }
}


#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]

pub struct GraphLayers {
    base_layer: Vec<Vec<usize>>,
    upper_layers: Vec<HashMap<usize, Vec<usize>>>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]

struct Distibution {}

impl GraphLayers {
    fn initialize_node(&mut self, node_id: usize, target_level: usize) {
        self.base_layer.push(Vec::new());

        // it must startfrom layer 1
        // and index from 0 so zero is writen here
        for l in 0..target_level {
            if l >= self.upper_layers.len() {
                self.upper_layers.push(HashMap::new());
            }
            self.upper_layers[l].insert(node_id, Vec::new());
        }
    }
    fn add_neighbors(&mut self) {
        // TO-DO
    }

    fn shrink_edge() {
        // TODO
    }

    fn add_edge(&mut self, node_id_1: usize, node_id_2: usize, layer: usize, d: bool) {
        if layer > 0 {
            if let Some(nodes) = self.upper_layers.get_mut(layer - 1) {
                nodes
                    .entry(node_id_1)
                    .or_insert_with(Vec::new)
                    .push(node_id_2);
                if !d {
                    nodes
                        .entry(node_id_2)
                        .or_insert_with(Vec::new)
                        .push(node_id_1);
                }
            }
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

    pub fn new(max_level: usize) -> Self {
        Self {
            base_layer: Vec::new(),
            upper_layers: Vec::with_capacity(max_level),
        }
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct HNSW {
    pub layers: GraphLayers,
    pub vectors: VectorStore,
    pub entry_point: Option<usize>,
    pub max_level: usize,
    pub ef_construction: usize,
    pub m: usize,
}

impl HNSW {
    pub fn new(max_elements: usize, dim: usize) -> Self {
        let layers = GraphLayers::new(16); // Default max levels
        let vectors = VectorStore::new(max_elements, dim);
        HNSW {
            layers,
            vectors,
            entry_point: None,
            max_level: 16,
            ef_construction: 64, // Default
            m: 16,               // Default
        }
    }

    pub fn insert(&mut self, q: usize, M: usize, Mmax: usize, ef_construction: usize, m_l: f32) {
        let mut w: BinaryHeap<Reverse<(OrderedFloat<f32>, usize)>> = BinaryHeap::new(); // Min Heap to get nearest dist_sq or node_id 

        let u: f32 = rand::random();
        let level = (-(1.0 - u).ln() * m_l).floor() as usize;
        self.layers.initialize_node(q, level);

        let mut ep = match self.entry_point {
            Some(ep) => ep,
            None => {
                // First element becomes entry point
                self.entry_point = Some(q);
                return;
            }
        };

        let top_level = self.layers.upper_layers.len();

        for lc in ((level + 1)..=top_level).rev() {
            let k = self.search_layer(
                &self.vectors.data[q * self.vectors.dim..(q + 1) * self.vectors.dim],
                ep,
                1,
                lc,
            );
            // ep = nearest element in W
            if let Some((OrderedFloat(_), best_node)) = k.peek() {
                ep = *best_node;
            }
        }

        for lc in (0..=min(top_level, level)).rev() {
            let k = self.search_layer(
                &self.vectors.data[q * self.vectors.dim..(q + 1) * self.vectors.dim],
                ep,
                ef_construction,
                lc,
            );
            for (OrderedFloat(dist_sq), node_id) in k {
                w.push(Reverse((OrderedFloat(dist_sq), node_id)));
            }
            // ep = nearest element in W
            if let Some(Reverse((OrderedFloat(_), best_node))) = w.peek() {
                ep = *best_node;
            }

            // neighbors ← SELECT-NEIGHBORS(q, W, M, lc)

            let candidates = w.clone().into_vec();

            let neighbors = HNSW::select_neighbors_simple(
                &self.vectors.data[q * self.vectors.dim..(q + 1) * self.vectors.dim],
                candidates,
                M,
                lc,
            );

            for node in &neighbors {
                self.layers.add_edge(q, *node, lc, false);
            }

            for e in &neighbors {
                // Shrink connections
                let eConn = self.layers.get_neighbors(lc, *e);
                if eConn.len() > Mmax {
                    // Calculate distances for eConn to create candidates
                    let mut conn_candidates = Vec::new();
                    for &n in eConn {
                        let dist = self.vectors.squared_distance(*e, n);
                        conn_candidates.push(Reverse((OrderedFloat(dist), n)));
                    }
                    let _e_new_conn = HNSW::select_neighbors_simple(
                        &self.vectors.data[e * self.vectors.dim..(e + 1) * self.vectors.dim],
                        conn_candidates,
                        Mmax,
                        lc,
                    );
                }
            }
            w.clear(); // Clear w for next layer
        }
    }

    // greedy beam search
    pub fn search_layer(
        &self,
        q: &[f32],
        ep: usize,
        ef_construction: usize,
        lc: usize,
    ) -> BinaryHeap<(OrderedFloat<f32>, usize)> {
        // let ep = self.entry_point.expect("ENTRY POINT ERROR");
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

            let (OrderedFloat(dist_worst), _furthest_element) = *found_neighbours.peek().unwrap();

            if dist_c > dist_worst {
                break;
            }

            for e in GraphLayers::get_neighbors(&self.layers, lc, closest_candidate) {
                if !visited.contains(e) {
                    let dist_e = VectorStore::squared_distance_to_query(&self.vectors, *e, q);
                    let _dist_e_wrapped = OrderedFloat(dist_e);

                    let (OrderedFloat(current_worst_dist), _) = *found_neighbours.peek().unwrap();
                    visited.insert(*e);
                    if dist_e < current_worst_dist || found_neighbours.len() < ef_construction {
                        candidates.push(Reverse((OrderedFloat(dist_e), *e)));
                        found_neighbours.push((OrderedFloat(dist_e), *e));
                        if found_neighbours.len() > ef_construction {
                            found_neighbours.pop();
                        }
                    }
                }
            }
        }
        found_neighbours
        // found_neighbours.into_iter().map(|(_, idx)| idx).collect()
    }

    // Algorithm 5: K-NN-SEARCH
    pub fn search(&self, query: &[f32], k: usize, ef_search: usize) -> Vec<(f32, usize)> {
        let mut ep = match self.entry_point {
            Some(ep) => ep,
            None => return Vec::new(),
        };

        let top_level = self.layers.upper_layers.len();
        // Phase 1: Greedy search from top to 1
        for lc in (1..=top_level).rev() {
            let w = self.search_layer(query, ep, 1, lc);
            if let Some((OrderedFloat(_), best_node)) = w.peek() {
                ep = *best_node;
            }
        }

        let mut w = self.search_layer(query, ep, ef_search, 0);

        let mut result = Vec::new();
        while let Some((OrderedFloat(dist), node_id)) = w.pop() {
            result.push((dist, node_id));
            if result.len() >= k {}
        }

        result.reverse();
        result.into_iter().take(k).collect()
    }

    pub fn brute_force_search(&self, query: &[f32], k: usize) -> Vec<(f32, usize)> {
        let n = self.vectors.data.len() / self.vectors.dim;
        let mut results: Vec<_> = (0..n)
            .into_par_iter()
            .map(|i| {
                let dist = self.vectors.squared_distance_to_query(i, query);
                (OrderedFloat(dist), i)
            })
            .collect();

        if k >= results.len() {
             results.sort_unstable();
             return results.into_iter().map(|(OrderedFloat(d), i)| (d, i)).collect();
        }

        results.select_nth_unstable(k);
        results.truncate(k);
        results.sort_unstable();

        results.into_iter().map(|(OrderedFloat(d), i)| (d, i)).collect()
    }

    fn select_neighbors_simple(
        _q: &[f32],
        mut candidates: Vec<Reverse<(OrderedFloat<f32>, usize)>>,
        m: usize,
        _lc: usize,
    ) -> Vec<usize> {
        if candidates.len() <= m {
            candidates.sort_unstable();
            return candidates.into_iter().map(|Reverse((_, id))| id).collect();
        }

        candidates.select_nth_unstable(m);
        candidates.truncate(m);

        candidates.into_iter().map(|Reverse((_, id))| id).collect()
    }
}
