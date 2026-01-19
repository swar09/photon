use nano_rag::HNSW;
use std::sync::{Arc, RwLock};
use std::{result, thread};

const EPSILON: f32 = 1e-5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_logic() {
        // Simplified test that doesn't rely on private implementation details locally
        let vec1: Vec<f32> = vec![1.0, 2.0, 3.0];
        let vec2: Vec<f32> = vec![4.0, 5.0, 6.0];
        
        let mut dist: f32 = 0.0;
        for (a, b) in vec1.iter().zip(vec2.iter()) {
            dist += (a - b) * (a - b);
        }
        
        assert!((dist - 27.0).abs() < EPSILON);
    }
}

fn main() {
    let hnsw = HNSW::new(100, 128);
    let shared_db = Arc::new(RwLock::new(hnsw));
    println!("Database initialized and locked globally.");
    println!("HNSW Library Compiles!");
    let db_writer = Arc::clone(&shared_db);

    let insert_handle = thread::spawn(move || {
        let mut guard = db_writer.write().unwrap();
        println!("Writer: Acquired lock. Inserting data...");
        
        // Simulating heavy work
        for _i in 0..100 {
            let vec = vec![1.5; 128]; // 1. Create the data
            
            // 2. PUT DATA IN STORE FIRST (This was missing!)
            // This returns the ID of the new vector
            let id = guard.vectors.insert(&vec); 
            
            // 3. NOW Insert into HNSW Graph using that ID
            // M=16, Mmax=32, ef=64, m_l=0.5
            guard.insert(id, 16, 32, 64, 0.5); 
        }
        println!("Writer: Finished. Releasing lock.");
    });
    
    let db_reader = Arc::clone(&shared_db);

    let search_handle = thread::spawn(move || {
        let mut gaurd = db_reader.read().unwrap();
        println!("Reader: Acquired lock. Reading data...");
        // write  a search vector function in the hnsw here to test this out 

        let query = vec![2.56; 128];

        let results = gaurd.search(&query, 2, 1);
        println!("{:?}", results);
        println!("Reader: Finished. Releasing lock.");
    });

    insert_handle.join().unwrap();
    search_handle.join().unwrap();



}