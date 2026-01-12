use nano_rag::Graph;
use std::time::Instant;

const DIM: usize = 128; // Dimen = k . RAM 
const N_VECTORS: usize = 1_000_000;
const N_QUERIES: usize = 100;

fn main() {
    let mut graph = Graph::new(N_VECTORS, DIM);
    let time = Instant::now();
    println!("      All Created");
    for u in 0..N_VECTORS {
        let random_nodes: Vec<usize> = (0..10)
            .into_iter()
            .map(|n| rand::random_range(0..N_VECTORS))
            .collect();
        for v in random_nodes {
            graph.add_edge(u, v, false);
        }
    }
    println!("      Edges Added");
    println!("      BF started");

    let start_bf = Instant::now();
    for i in 0..N_QUERIES {
        let query: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();
        graph.brute_force_search(&query); // we will store result later 
    }
    let duration_bf = start_bf.elapsed();
    println!("      {:?}",duration_bf);
}
