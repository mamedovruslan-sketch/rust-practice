pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut best = scores[0];
    let mut worst = scores[0];
    let mut best_count = 0;
    let mut worst_count = 0;

    for score in scores.iter().skip(1) {
        if *score > best {
            best = *score;
            best_count += 1;
        }

        if *score < worst {
            worst = *score;
            worst_count += 1;
        }
    }

    vec![best_count, worst_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let result = breaking_records(scores);

        assert_eq!(result, vec![2, 4]);
    }
}