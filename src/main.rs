use euler::exercises::Exercises;
fn main() {
    let ex = Exercises::new();
    ex.get("exercise_1")();
    ex.get("exercise_2")();
    ex.get("exercise_3")();
}
