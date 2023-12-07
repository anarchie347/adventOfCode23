use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;
mod day6;
mod day7;

fn main() {
    run(day7::d7p2, 7);
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

    let text : &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}