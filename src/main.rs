 use std::cmp::max;

#[test]
 fn test(){}
struct Graph {
    adj_list: Vec<Vec<usize>>,
    vectors: Vec<Vec<f32>>,
}

impl Graph {
     fn new(n: usize) -> Graph {
        let mut adj_list = Vec::with_capacity(n);
        let mut vectors = Vec::with_capacity(n);
        for _ in 0..n {
            adj_list.push(Vec::new());
            vectors.push(Vec::<f32>::new());
        }
        return Graph { adj_list, vectors };
    }

    fn new_node(){} // do new node and test cases 

    fn distance(vec1: &mut Vec<f32>, vec2: &mut Vec<f32>) -> f32{
        let mut sum: f32 = 0.0;
        // Check for empty vector here first handlethe error properly 
        
        if !(vec1.len() == vec2.len()) {
            
            let len = max(vec1.len(), vec2.len());
            vec1.resize(len, 0.0);
            vec2.resize(len, 0.0);
        }
        for i in 0..vec1.len() {
            let diff = vec1[i] - vec2[i];
            sum = sum + diff*diff;
        }
        return sum.sqrt()
    }
    
    fn modulus(vec1: &Vec<f32>) -> f32 {
        let mut sum: f32 = 0.0;
        for i in 0..vec1.len() {
            let diff = vec1[i];
            sum = sum + diff*diff;
        }
        return sum.sqrt()
    }

    fn greedy_search() {}
}

fn main() {}
