use crate::utils;
pub fn run() {
    let limit = utils::input(&"Enter the upper number: ") // get's the input from stdin
        .parse::<usize>()
        .expect("Failed to parse the input")
        - 1;
    let mut acc = 2;
    let mut last_two = (1, 2);
    let mut t: usize;
    loop {
        t = last_two.0 + last_two.1;
        if t > limit {
            break;
        }
        last_two.0 = last_two.1;
        last_two.1 = t;
        acc += t * (!t & 1);
    }
    println!("ANSWER {}", acc);
}
