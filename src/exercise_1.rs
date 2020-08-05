use crate::utils;
pub fn run() {
    let n_5 = utils::input(&"Enter the first number: ")
        .parse::<usize>()
        .unwrap();
    let n_2 = utils::input(&"Enter the second number: ")
        .parse::<usize>()
        .unwrap();
    let upper_bound = utils::input(&"Enter the upper bound: ")
        .parse::<usize>()
        .unwrap()
        - 1; // under upper bound, which implies non-inclusive range
    println!(
        "THE ANSWER: {}",
        sum(n_1, upper_bound / n_1) + sum(n_2, upper_bound / n_2)
            - sum(n_2 * n_1, upper_bound / n_1 / n_2)
    );
}
fn sum(step: usize, number: usize) -> usize {
    (number * step * (number + 1)) / 2
}
