use rand::thread_rng;
use rand::Rng;

pub fn generate_rand_vec_i32(length: u32) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}
