pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = 0;
    let size = arr.len();

    for i in 0..size {
        left_sum += arr[i][i];
        right_sum += arr[i][size - 1 - i];
    }

    (left_sum - right_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];

        assert_eq!(diagonal_difference(arr), 15);
    }
}