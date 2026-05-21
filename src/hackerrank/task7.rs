pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();
    let mut count = 0;

    for number in start..=end {
        let first_condition = a.iter().all(|value| number % value == 0);
        let second_condition = b.iter().all(|value| value % number == 0);

        if first_condition && second_condition {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        assert_eq!(get_total_x(a, b), 3);
    }
}