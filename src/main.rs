mod hackerrank;

use hackerrank::task11::diagonal_difference;

fn main() {
    let arr = vec![
        vec![11, 2, 4],
        vec![4, 5, 6],
        vec![10, 8, -12],
    ];

    println!("{}", diagonal_difference(arr));
}