use std::collections::HashMap;

fn main() {
    let vec_odd_length = vec![0, 1, 1, 2, 3, 4, 4, 4, 8, 9, 10, 10];
    let vec_even_length = vec![0, 1, 2, 3, 4, 8, 9, 10];

    println!("{:?}", vec_even_length);
    println!("{}", vector_median(&vec_even_length));

    println!("{:?}", vec_odd_length);
    println!("{}", vector_median(&vec_odd_length));

    println!(
        "{:?} should equal [(1, 3)]",
        vector_mode(&mut vec![1, 1, 1, 3, 2, 4, 7,])
    );
    println!(
        "{:?} should equal [(1, 3), (3, 3)]",
        vector_mode(&mut vec![1, 1, 1, 3, 3, 3, 2, 4, 7,])
    );
}

// Param vector should be sorted
fn vector_median(vector: &[i32]) -> f64 {
    let vec_half_len = vector.len() / 2;

    if vector.len() % 2 == 0 {
        println!("vector length is even");
        let sum = vector[vec_half_len - 1] + vector[vec_half_len];
        return (sum as f64) / 2.0;
    }

    vector[vec_half_len] as f64
}

fn vector_mode(vector: &mut Vec<i32>) -> Vec<(i32, i32)> {
    let mut map = HashMap::new();

    let mut highest_frequency: Vec<(i32, i32)> = vec![(0, 0)];

    for item in vector {
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    for (key, value) in map {
        if value == highest_frequency[0].1 {
            highest_frequency.append(&mut vec![(*key, value)]);
        }
        if value > highest_frequency[0].1 {
            highest_frequency = vec![(*key, value)];
        }
    }

    highest_frequency
}
