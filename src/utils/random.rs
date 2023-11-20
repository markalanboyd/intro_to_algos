use rand::thread_rng;
use rand::Rng;

pub fn generate_rand_vec_i32(length: usize) -> Vec<i32> {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

pub fn generate_bounded_index(v: &Vec<i32>) -> usize {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    rng.gen_range(0..v.len().try_into().unwrap())
}

// let random_number_in_range: u32 = rng.gen_range(1..101);
