mod hackerrank;

use hackerrank::task4::grading_students;

fn main() {
    let result = grading_students(vec![73, 67, 38, 33]);

    for grade in result {
        println!("{}", grade);
    }
}