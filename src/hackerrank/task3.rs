pub fn staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;

        result.push_str(&" ".repeat(spaces as usize));
        result.push_str(&"#".repeat(hashes as usize));

        if i != n {
            result.push('\n');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
        assert_eq!(staircase(6), expected);
    }
}