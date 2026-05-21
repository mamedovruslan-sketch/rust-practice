mod hackerrank;

use hackerrank::task9::migratory_birds;

fn main() {
    let result = migratory_birds(vec![1, 4, 4, 4, 5, 3]);

    println!("{}", result);
}