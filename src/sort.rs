// INSERTION-SORT(A, n)
// for i == 2 to n
//  key == A[i]
//  Insert A[i] into the sorted subarray A[1: i - 1].
//  j = i - 1
//  while j > 0 and A[j] > key
//      A[j + 1] = A[j]
//      j = j - 1
//  A[j + 1] = key

pub fn insertion_sort(v: &mut Vec<i32>) {
    let n: usize = v.len();
    for i in 1..n {
        let key: i32 = v[i];
        let mut j: usize = i;
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = key;
    }
}

use rand::thread_rng;
use rand::Rng;

pub fn generate_rand_vec_i32(length: u32) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

pub fn check_if_sorted(v: &Vec<i32>) -> Result<&'static str, &'static str> {
    if v.is_empty() {
        return Ok("Vector is empty.");
    }

    for (i, &num) in v.iter().enumerate() {
        if i > 0 && v[i - 1] > num {
            return Err("Error: Vector is not sorted.");
        }
    }

    Ok("Vector is sorted.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut test_vec: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
        let expected_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        println!("Fixed Test");
        println!("----------");
        println!("Test:     {:?}", test_vec);
        insertion_sort(&mut test_vec);
        println!("Sorted:   {:?}", test_vec);
        println!("Expected: {:?}", expected_vec);
        assert_eq!(test_vec, expected_vec);
        println!();
        println!("Random Test");
        println!("-----------");
        println!("Generating random Vec<i32>...");
        let mut rand_vec: Vec<i32> = generate_rand_vec_i32(10_000);
        println!("Sorting...");
        insertion_sort(&mut rand_vec);
        println!("Verifying results...");
        match check_if_sorted(&rand_vec) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("{}", error),
        };
    }
}
