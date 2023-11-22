// A + B | SUM | CARRY
// 0 + 0 | 0   | 0
// 1 + 0 | 1   | 0
// 0 + 1 | 1   | 0
// 1 + 1 | 0   | 1

// Little-endian

pub fn add_binary_integers(v_a: &Vec<u8>, v_b: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut carry: u8 = 0;
    let max_len: usize = usize::max(v_a.len(), v_b.len());

    for i in 0..max_len {
        let a: u8 = *v_a.get(i).unwrap_or(&0);
        let b: u8 = *v_b.get(i).unwrap_or(&0);
        let sum: u8 = a + b + carry;

        result.push(sum % 2);
        carry = sum / 2;
    }

    if carry != 0 {
        result.push(carry);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_integers() {
        let v_a: Vec<u8> = vec![1, 0, 1, 1];
        let v_b: Vec<u8> = vec![1, 1, 0, 1];
        let result: Vec<u8> = add_binary_integers(&v_a, &v_b);
        let expected: Vec<u8> = vec![0, 0, 0, 1, 1];
        println!("A:        {:?}", v_a);
        println!("B:        {:?}", v_b);
        println!("Result:   {:?}", result);
        println!("Expected: {:?}", expected);
        assert_eq!(result, expected);
    }
}

// pub fn add_binary_integers(v_a: &Vec<u32>, v_b: &Vec<u32>) -> Vec<u32> {
//     let base: u32 = 2;
//     let mut result: Vec<u32> = vec![];
//     for i in 0..v_a.len() {
//         let multiplier = base.pow(i as u32);
//         result.push((v_a[i] + v_b[i]) * multiplier);
//     }
//     result
// }
