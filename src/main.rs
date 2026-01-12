use nano_rag::Graph;
const EPSILON: f32 = 1e-5;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_logic() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        
        let dist = Graph::distance(&vec1, &vec2);
        
        assert!((dist - 5.196152).abs() < EPSILON);
    }
}

fn main() {
    let graph = Graph::new(10000, 10000);
    // Write more tests 
}