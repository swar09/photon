// use colored::*;
use colored_text::Colorize;
use nano_rag::HNSW;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use rand::prelude::*;
use rand::rngs::StdRng;
use serde::*;


const DIM: usize = 32; 
const N_VECTORS: usize = 10000; 
const N_QUERIES: usize = 8;

fn main() {
    println!("{}", r#"
                                                        
                                                       
                                                       
                                                       
                  :#################-                  
              =#########################+              
             #############################             
             :###########################=             
                =#####################=                
             ##=         .:::.         =##             
             #########*+===-====**########             
             #############################             
              -+#######################*=              
             #:    :=++++*****++++=:    :*             
             ########=-.       .-=########             
             #############################             
              :#########################=              
             +:      -=+**#****+=-.     :=             
             ######+-::.       ..:-+######             
             #############################             
               +#######################+.              
             #=      .:-=======-::      :#             
             #########+=:     :=+*########             
             #############################             
              ###########################              
                  +#################*.                 
                                                       
                                                       
                                                       
                                                       
    NANO RAG DATABASE
    Author: Swarnit Ghule
    GitHub: https://github.com/swar09/nano-rag
    "#.cyan().bold());
    
    // Generate vectors in parallel with deterministic seed
    let vectors: Vec<Vec<f32>> = (0..N_VECTORS)
        .into_par_iter()
        .map(|i| {
            let mut rng = StdRng::seed_from_u64(i as u64);
            (0..DIM).map(|_| rng.random::<f32>()).collect()
        })
        .collect();

    // Insert vectors
    let mut graph = HNSW::new(N_VECTORS, DIM);
    for vec in vectors {
        let id = graph.vectors.insert(&vec);
        graph.insert(id, 16, 32, 100, 0.5); 
    }
    
    println!("    {}", "Starting Benchmark...".blue().bold());
    
    let (total_duration_bf, total_duration_hnsw, correct_matches) = (0..N_QUERIES)
        .into_par_iter()
        .map(|i| {
            // Seed query generation deterministically (offset by N_VECTORS to avoid duplicate of first vector)
            let mut rng = StdRng::seed_from_u64((N_VECTORS + i) as u64);
            let query: Vec<f32> = (0..DIM).map(|_| rng.random::<f32>()).collect();
            
            // ... (rest of the loop same)
            
            // Brute Force
            let start = Instant::now();
            let bf_results = graph.brute_force_search(&query, 1);
            let duration_bf = start.elapsed();
            
            // HNSW Search
            let start = Instant::now();
            let hnsw_results = graph.search(&query, 1, 64);
            let duration_hnsw = start.elapsed();
            
            let mut match_count = 0;
            if !bf_results.is_empty() && !hnsw_results.is_empty() {
                 if bf_results[0].1 == hnsw_results[0].1 {
                     match_count = 1;
                 }
            }
            (duration_bf, duration_hnsw, match_count)
        })
        .reduce(
            || (Duration::ZERO, Duration::ZERO, 0),
            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2)
        );
    
    println!("    {}: {}", "Brute Force Avg".blue().bold(), format!("{:?}", total_duration_bf / N_QUERIES as u32).green().bold());
    println!("    {}: {}", "HNSW Search Avg".blue().bold(), format!("{:?}", total_duration_hnsw / N_QUERIES as u32).green().bold());
    println!("    {}: {}", "Recall".blue().bold(), format!("{:.2}%", (correct_matches as f64 / N_QUERIES as f64) * 100.0).green().bold());
}