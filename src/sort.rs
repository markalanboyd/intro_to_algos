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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut test_vec: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
        let expected_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        println!("Test:     {:?}", test_vec);
        insertion_sort(&mut test_vec);
        println!("Sorted:   {:?}", test_vec);
        println!("Expected: {:?}", expected_vec);
        assert_eq!(test_vec, expected_vec);
    }
}
