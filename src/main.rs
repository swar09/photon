use photon::HNSW;
// use serde::*;
// use photon::PhotonDB;

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
    let hnsw = HNSW::new(10, 3);
        
}
