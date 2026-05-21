pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut counts = vec![0; 6];

    for bird in arr {
        counts[bird as usize] += 1;
    }

    let mut best_type = 1;
    let mut best_count = counts[1];

    for bird_type in 2..=5 {
        if counts[bird_type] > best_count {
            best_count = counts[bird_type];
            best_type = bird_type as i32;
        }
    }

    best_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let birds = vec![1, 4, 4, 4, 5, 3];

        assert_eq!(migratory_birds(birds), 4);
    }

    #[test]
    fn test_migratory_birds_smallest_type() {
        let birds = vec![1, 1, 2, 2, 3];

        assert_eq!(migratory_birds(birds), 1);
    }
}