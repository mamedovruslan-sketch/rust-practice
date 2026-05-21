use std::collections::HashMap;

pub fn sock_merchant(ar: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for color in ar {
        let counter = counts.entry(color).or_insert(0);
        *counter += 1;
    }

    let mut pairs = 0;

    for count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

        assert_eq!(sock_merchant(socks), 3);
    }
}