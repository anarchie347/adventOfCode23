use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;
mod day6;

fn main() {
    run(day6::d6, 6);
    //test(day5::d5p2, 5);
}

fn run(f : fn(Vec<&str>) -> i32, day : usize) {
    let text = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("couldnt read file");
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let instant = std::time::Instant::now();
    let result = f(splitText);
    println!("Took: {:?}", instant.elapsed());
    println!("Result: {}", result);
}

fn test(f : fn(Vec<&str>) -> i32, day : usize) {

    let text : &str = "Time:         7     15      30
    Distance:     9     40     200
    ";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}