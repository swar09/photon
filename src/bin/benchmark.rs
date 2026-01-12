use colored_text::Colorize;
use nano_rag::Graph;
use std::time::Instant;

const DIM: usize = 12; // Dimen = k . RAM 
const N_VECTORS: usize = 1_000;
const N_QUERIES: usize = 100;

fn main() {
    let mut graph = Graph::new(N_VECTORS, DIM);
    // let time = Instant::now();
    // println!("      All Created");
    for u in 0..N_VECTORS {
        let random_nodes: Vec<usize> = (0..10)
            .into_iter()
            .map(|_| rand::random_range(0..N_VECTORS))
            .collect();
        for v in random_nodes {
            graph.add_edge(u, v, false);
        }
    }
    // println!("      Edges Added");
    // println!("      BF started");

    let start_bf = Instant::now();
    for _ in 0..N_QUERIES {
        let query: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();
        graph.brute_force_search(&query); // we will store result later 
    }
    let duration_bf = start_bf.elapsed();
    let w = 35;
    println!();
    println!();
    println!();
    println!(
        "      {}",
        format!("{:>w$}", "Initializing NANO-RAG DB.....")
            .green()
            .bold()
    );

    println!(
        "      {}",
        format!("{:>w$}", "Starting Benchmark.....").green().bold()
    );

    println!();

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Brute Force Search").blue().bold(),
        duration_bf
    );

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Avg").green(),
        duration_bf / N_QUERIES as u32
    );

    let start_gs = Instant::now();
    for _ in 0..N_QUERIES {
        let query: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();
        graph.greedy_search(&query); // we will store result later 
    }
    let duration_gs = start_gs.elapsed();

    println!();

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Brute Force Search").blue().bold(),
        duration_gs
    );

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Avg").green(),
        duration_gs / N_QUERIES as u32
    );
    println!("");
    println!("");
    println!("");
}
