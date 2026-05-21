pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .filter(|&&distance| {
            let position = a + distance;
            position >= s && position <= t
        })
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&distance| {
            let position = b + distance;
            position >= s && position <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let result = count_apples_and_oranges(
            7,
            11,
            5,
            15,
            vec![-2, 2, 1],
            vec![5, -6],
        );

        assert_eq!(result, (1, 1));
    }
}