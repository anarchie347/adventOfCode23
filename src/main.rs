use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    run(day9::d9, 9);
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

    let text : &str = "LR

    PPA = (PPB, XXX)
    PPB = (XXX, PPZ)
    PPZ = (PPB, XXX)
    QQA = (QQB, XXX)
    QQB = (QQC, QQC)
    QQC = (QQZ, QQZ)
    QQZ = (QQB, QQB)
    XXX = (XXX, XXX)";
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