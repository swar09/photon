#[warn(unused)]
// #[warn(dead_code)]
use rayon::prelude::*;
use std::cmp::Ordering;

// const N

pub struct HNSW {
    layers: Vec<Vec<Vec<usize>>>,

}

impl HNSW {
    pub fn insert(&mut self, q: &[f32], m: usize, m_max: usize, efConstruction: usize , m_l: f32){
        
        // update hnsw inserting element q
        todo!()
    }

    pub fn search_layer(q: &[f32], ep: Vec<usize>, ef: usize, lc: usize) -> Vec<usize> {
        //  ef: Vec<usize> = [node_ids] closest neighbors to q
        todo!()
    }
}

#[derive(Debug)]
pub struct Graph {
    adj_list: Vec<Vec<usize>>,
    vectors: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct ScoredNode {
    id: usize,
    score: f32,
}

impl PartialEq for ScoredNode {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for ScoredNode {}
impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}
impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Graph {
    pub fn new(n: usize, dim: usize) -> Graph {
        // Initialize with random vectors for demo purposes
        // let mut _rng = rand::rng();
        let vectors: Vec<Vec<f32>> = (0..n)
            .map(|_| (0..dim).map(|_| rand::random::<f32>()).collect())
            .collect();

        Graph {
            adj_list: vec![Vec::new(); n],
            vectors,
        }
    }

    // Will add edge between nodes
    pub fn add_edge(&mut self, u: usize, v: usize, d: bool) {
        if d {
            self.adj_list[u].push(v);
        } else {
            self.adj_list[u].push(v);
            self.adj_list[v].push(u);
        }
        // println!("{} --> {}", u, v);
    }

    pub fn insert_hnsw(&mut self, u: usize, v: usize) {
        // Step1 Find the actual nearest neighobours
        // let nearest_n =
        // add edge between them
        // What about NAvigable > HNSW ??
    }

    pub fn distance(vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() {
            panic!(
                "Vector dimension mismatch: {} vs {}",
                vec1.len(),
                vec2.len()
            );
        }

        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn brute_force_search(&self, query: &[f32]) -> Option<(usize, f32)> {
        // rayon is used here because we are searching MANY vectors
        self.vectors
            .par_iter()
            .enumerate()
            .map(|(i, v)| (i, Graph::distance(v, query)))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
    }

    pub fn greedy_search(&self, query: &[f32]) -> Option<usize> {
        if self.vectors.is_empty() {
            return None;
        }

        let mut current_node = rand::random_range(0..self.vectors.len());
        let mut min_dist = Graph::distance(&self.vectors[current_node], query);

        // Simple Greedy Descent
        loop {
            let mut best_neighbor = None;

            // Check all neighbors of current node
            for &neighbor in &self.adj_list[current_node] {
                let d = Graph::distance(&self.vectors[neighbor], query);
                if d < min_dist {
                    min_dist = d;
                    best_neighbor = Some(neighbor);
                }
            }

            match best_neighbor {
                Some(n) => current_node = n,
                None => break,
            }
        }

        Some(current_node)
    }
}
