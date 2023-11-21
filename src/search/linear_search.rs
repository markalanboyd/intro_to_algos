// Input: A sequence of n numbers store in an array A[1:n] and a value x
// Output: An index i such that x equals A[i] or NIL if x does not appear

pub fn linear_search<T: PartialEq>(slice: &[T], sequence: &[T]) -> Option<usize> {
    if sequence.len() > slice.len() {
        return None;
    }
    for (i, window) in slice.windows(sequence.len()).enumerate() {
        if window == sequence {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::random;

    #[test]
    fn test_linear_search_fixed() {
        let test_vec: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
        let test_seq: Vec<i32> = vec![4, 6, 1];
        let expected: Option<usize> = Some(2);
        let result: Option<usize> = linear_search(&test_vec, &test_seq);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_linear_search_random() {
        let length: usize = 10_000;
        let rand_vec: Vec<i32> = random::generate_rand_vec_i32(length);
        let i: Option<usize> = Some(random::generate_bounded_index(&rand_vec));
        let slice: &[i32] = match i {
            Some(index) => &rand_vec[index..],
            None => &[],
        };
        let result: Option<usize> = linear_search(&rand_vec, slice);
        assert_eq!(i, result);
    }
}
