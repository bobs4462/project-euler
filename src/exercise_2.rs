use crate::utils;
pub fn run() {
    let limit = utils::input(&"Enter the upper number: ") // get's the input from stdin
        .parse::<usize>()
        .expect("Failed to parse the input")
        - 1;
    let mut acc = 0;
    let mut last_two = (1, 1);
    while last_two.0 + last_two.1 < limit {
        acc += last_two.0 + last_two.1;
        last_two = (last_two.0 + 2 * last_two.1, 2 * last_two.0 + 3 * last_two.1);
    }
    println!("ANSWER {}", acc);
}
