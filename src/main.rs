use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    bench(day9::d9p2, 9);
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

fn test(f : fn(Vec<&str>) -> i32, _day : usize) {

    let text : &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}

fn bench(f : fn(Vec<&str>) -> i32, day : usize) {
    let text = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("couldnt read file");
    let splitText : Vec<&str> = text.split("\n").map(|s| {s.trim()}).collect();
    const repetitions : u32= 1000;
    let instant = std::time::Instant::now();
    for i in 0..repetitions {
        f(splitText.clone());
    }
    
    println!("Average time: {:?}", instant.elapsed() / repetitions);
}