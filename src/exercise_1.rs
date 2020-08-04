use crate::utils;
pub fn run() {
    let n_1 = utils::input(&"Enter the first number: ")
        .trim()
        .parse::<usize>()
        .unwrap();
    let n_2 = utils::input(&"Enter the second number: ")
        .trim()
        .parse::<usize>()
        .unwrap();
    let upper_bound = utils::input(&"Enter the upper bound: ")
        .trim()
        .parse::<usize>()
        .unwrap()
        - 1; // under upper bound, which implies non-inclusive range
    println!(
        "THE ANSWER: {}",
        arithmetic_sum(n_1, upper_bound / n_1) + arithmetic_sum(n_2, upper_bound / n_2)
            - arithmetic_sum(n_2 * n_1, upper_bound / n_1 / n_2)
    );
}
fn arithmetic_sum(step: usize, number: usize) -> usize {
    (((number * step + step) as f64 / 2.0) * number as f64) as usize
}
