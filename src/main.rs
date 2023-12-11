#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(dead_code)]
use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    run(day11::d11, 11);
    //test(day5::d5p2, 5);
}

fn run<F,T>(f :  F, day : usize)
where
    F: Fn(Vec<&str>) -> T,
    T : std::fmt::Display
{
    let text = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("couldnt read file");
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let instant = std::time::Instant::now();
    let result = f(splitText);
    println!("Took: {:?}", instant.elapsed());
    println!("Result: {}", result);
}

fn test<F,T>(f :  F, _day : usize)
where
    F: Fn(Vec<&str>) -> T,
    T : std::fmt::Display
{

    let text : &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}

fn bench<F,T>(f :  F, day : usize)
where
    F: Fn(Vec<&str>) -> T,
    T : std::fmt::Display
{
    let text = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("couldnt read file");
    let splitText : Vec<&str> = text.split("\n").map(|s| {s.trim()}).collect();
    const repetitions : u32= 1000;
    let instant = std::time::Instant::now();
    for i in 0..repetitions {
        f(splitText.clone());
    }
    
    println!("Average time: {:?}", instant.elapsed() / repetitions);
}