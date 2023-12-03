mod day3;
use std::time::Instant;

use day3::d3p2;
fn main() {
    run(day3::d3p2, 3);
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

    let text : &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}