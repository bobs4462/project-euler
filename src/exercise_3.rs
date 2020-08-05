use crate::utils;
pub fn run() {
    let mut number = utils::input(&"Enter the number number: ") // get's the input from stdin
        .parse::<u128>()
        .expect("Failed to parse the input");
    let mut max_factor = 3;
    let sqrt = (number as f64).sqrt() as u128;
    loop {
        while number % max_factor == 0 {
            number /= max_factor;
        }
        if max_factor + 2 > sqrt || number == 1 {
            break;
        }
        max_factor += 2;
    }
    println!("ANSWER {}", max_factor);
}
