use rand::{Rng};
use std::collections::HashMap;

pub fn generate_random_sequence(keys_tones: &HashMap<usize, &&str>) {
    let keys_tones_length = keys_tones.len();
    let mut nums: Vec<usize> = (0..keys_tones_length).collect();

    for i in 1..46 {
        for j in 0..keys_tones_length {
            let secret_number = rand::thread_rng().gen_range(0, keys_tones_length);
            nums.swap(j, secret_number);

        }
        for t in &nums {
            print!("{}", keys_tones.get(t).unwrap());
        }
        if i%3==0 {
            println!();
        }
    }
}
