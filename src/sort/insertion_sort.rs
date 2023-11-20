// INSERTION-SORT(A, n)
// for i == 2 to n
//  key == A[i]
//  Insert A[i] into the sorted subarray A[1: i - 1].
//  j = i - 1
//  while j > 0 and A[j] > key
//      A[j + 1] = A[j]
//      j = j - 1
//  A[j + 1] = key

pub fn insertion_sort<T, F>(v: &mut Vec<T>, mut compare: F)
where
    T: Copy + Ord,
    F: FnMut(&T, &T) -> bool,
{
    let n = v.len();
    for i in 1..n {
        let key = v[i];
        let mut j = i;
        while j > 0 && compare(&v[j - 1], &key) {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{random, validate};

    #[test]
    fn test_insertion_sort_increasing_fixed() {
        let mut test_vec: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
        let expected_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        println!("::Increasing::");
        println!("Test:     {:?}", test_vec);
        insertion_sort(&mut test_vec, |a: &i32, b: &i32| a > b);
        println!("Sorted:   {:?}", test_vec);
        println!("Expected: {:?}", expected_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn test_insertion_sort_decreasing_fixed() {
        let mut test_vec: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
        let expected_vec: Vec<i32> = vec![6, 5, 4, 3, 2, 1];
        println!("::Decreasing::");
        println!("Test:     {:?}", test_vec);
        insertion_sort(&mut test_vec, |a: &i32, b: &i32| a < b);
        println!("Sorted:   {:?}", test_vec);
        println!("Expected: {:?}", expected_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn test_insert_sort_random_increasing() {
        println!("Generating random Vec<i32>...");
        let mut rand_vec: Vec<i32> = random::generate_rand_vec_i32(10_000);
        println!("Sorting...");
        insertion_sort(&mut rand_vec, |a: &i32, b: &i32| a > b);
        println!("Verifying results...");
        assert!(validate::vec_is_sorted(&rand_vec, |a, b| a > b))
    }
}
