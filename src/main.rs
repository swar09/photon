use core::f32;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::usize;

#[test]
fn unit() {
    // Handle test cases properly error are there
    let mut vec1 = Vec::with_capacity(10);
    let mut vec2 = Vec::with_capacity(10);
    vec1.resize(99, 1.45657);
    vec2.resize(99, 4.65757);

    let distance = Graph::distance(&vec1, &vec2); // pas the slice not the vec
    // println!("{}", distance);
    assert_eq!(distance, 31.849545);

    let modulus = Graph::modulus(&vec1);
    assert_eq!(modulus, 14.492692);
}

#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>,
    vectors: Vec<Vec<f32>>,
}

struct Cosine {
    sim: f32,
    dist: f32,
}

impl Graph {
    fn new(n: usize) -> Graph {
        let adj_list: Vec<Vec<usize>> = (0..n).into_par_iter().map(|_| Vec::new()).collect();

        let vectors: Vec<Vec<f32>> = (0..n).into_par_iter().map(|_| Vec::new()).collect();
        return Graph { adj_list, vectors };
    }

    fn new_node() {} // do new node and test cases 

    fn add_edge(&mut self, u: usize, v: usize, d: bool) {
        if d {
            // Directed
            self.adj_list[u].push(v);
            println!("{} --> {}", u, v);
        } else {
            // Undirected
            self.adj_list[u].push(v);
            self.adj_list[v].push(u);
            println!("{} <--> {}", u, v);
        }
    }

    // Write thread safe ques , use parallel but not rayon needed here
    // Parallel opreations using ques is tricky

    fn bfs_traversal(&self, s: usize) {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut vist: Vec<bool> = vec![false; self.adj_list.len()];

        que.push_back(s);
        vist[s] = true;

        while let Some(s) = que.pop_front() {
            for &v in &self.adj_list[s] {
                if !vist[v] {
                    vist[v] = true;
                    que.push_back(v);
                }
            }
        }
    }
    // show the traversal logic properly here last time it was not visible

    fn distance(vec1: &[f32], vec2: &[f32]) -> f32 {
        let mut sum: f32 = 0.0;
        // Check for empty vector here first handlethe error properly
        // n and m ds vectors
        if !(vec1.len() == vec2.len()) {
            let len = max(vec1.len(), vec2.len());
            // vec1.resize(len, 0.0);
            // vec2.resize(len, 0.0);
        }
        sum += vec1
            .par_iter()
            .zip(vec2.par_iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>();
        return sum.sqrt();
    }

    fn modulus(vec1: &[f32]) -> f32 {
        let mut sum: f32 = 0.0;
        sum += vec1.par_iter().map(|i| i * i).sum::<f32>();
        return sum.sqrt();
    }

    fn brute_force_search(&self, query: &[f32]) -> &Vec<f32> {
        let (best_vec, min_dist) = self
            .vectors
            .par_iter()
            .map(|v| (v, Graph::distance(v, query)))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        best_vec
    }

    fn greedy_search(&self, query: &[f32]) -> &[f32] {
        let mut bestVector = Vec::<f32>::new();
        let mut randNode: usize = 42; // add the rand usize funtion
        let mut minDist: f32 = Graph::distance(&self.vectors[randNode], query);
        
        while true {
            if Graph::distance(
                &self.vectors[*self.get_close_neighbour(randNode, query)], // Closet Node 
                query, // query vector 
            ) < minDist
            {
                minDist = Graph::distance(
                &self.vectors[*self.get_close_neighbour(randNode, query)], // Closet Node 
                query, // query vector 
            );

            randNode = *self.get_close_neighbour(randNode, query);

            } else {
                return &self.vectors[randNode];
            }
        }

        todo!()
    }

    fn get_close_neighbour(&self, node: usize, query: &[f32]) -> &usize {
        let closeNode = self.adj_list[node].par_iter().min_by(|n1, n2| {
            Graph::distance(&self.vectors[**n1], query)
                .total_cmp(&Graph::distance(&self.vectors[**n2], query))
        });

        match closeNode {
            Some(node) => node,
            None => {
                todo!()
            }
        }
    }

    fn cosine(vec1: &[f32], vec2: &[f32]) -> Cosine {
        let mut dot_sum: f32 = 0.0;
        // Dot product
        if vec1.len() == vec2.len() {
            dot_sum += vec1
                .par_iter()
                .zip(vec2.par_iter())
                .map(|(x, y)| x * y)
                .sum::<f32>();
        }

        let a: f32 = Graph::modulus(vec1);
        let b: f32 = Graph::modulus(vec2);
        let sim: f32 = dot_sum / (a * b); // CosT
        let dist: f32 = 1.0 - sim; // sim + dist = 1
        return Cosine { sim, dist };
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}

fn main() {}
