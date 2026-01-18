// use colored::*;
use colored_text::Colorize;
use nano_rag::Graph;
use std::time::{Duration, Instant};

const DIM: usize = 1536;
const N_VECTORS: usize = 1_000_000;
const N_QUERIES: usize = 100;

fn main() {
    let mut graph = Graph::new(N_VECTORS, DIM);
    
    for u in 0..N_VECTORS {
        let random_nodes: Vec<usize> = (0..16)
            .into_iter()
            .map(|_| rand::random_range(0..N_VECTORS))
            .collect();
        for v in random_nodes {
            graph.add_edge(u, v, false);
        }
    }

    let mut duration_bf = Duration::ZERO;
    let mut duration_gs = Duration::ZERO;
    let mut correct_matches = 0;

    for _ in 0..N_QUERIES {
        let query: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();

        let start = Instant::now();
        let bf_result = graph.brute_force_search(&query);
        duration_bf += start.elapsed();

        let start = Instant::now();
        let gs_result = graph.greedy_search(&query);
        duration_gs += start.elapsed();

        if let (Some((bf_id, _)), Some(gs_id)) = (bf_result, gs_result) {
            if bf_id == gs_id {
                correct_matches += 1;
            }
        }
    }

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

    println!();

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Greedy Search").blue().bold(),
        duration_gs
    );

    println!(
        "      {} : {:?}",
        format!("{:>w$}", "Avg").green(),
        duration_gs / N_QUERIES as u32
    );

    println!();
    println!(
        "      {} : {:.2}%",
        format!("{:>w$}", "Recall").red().bold(),
        (correct_matches as f64 / N_QUERIES as f64) * 100.0
    );

    println!();
    println!();
    println!();
}