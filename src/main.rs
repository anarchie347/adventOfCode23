use day5::{NumRange, mapRange};

mod day4;
mod day3;
mod day5;

fn main() {
    run(day5::newd5p2, 5);
    //test(day5::d5p2, 5);
}

fn t(text : Vec<&str>) -> i32 {
    let mut mappingType = 0;
    let mut maps : Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 2..text.len() {
        let trimmed = text[i].trim();
        if (trimmed == "") {
            continue;
        }
        if (!trimmed.chars().next().unwrap().is_ascii_digit()) {
            mappingType += 1;
            continue;
        }
        while (maps.len() <= mappingType) {
            maps.push(Vec::new())
        }
        //println!("E{:?}E", trimmed.split(" ").collect::<Vec<&str>>());
        maps[mappingType].push(trimmed.split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    maps = maps[1..].to_vec(); //not sure why but ok

    let nr = NumRange{lower: 48, range: 5, offset: 1};
    let mut nrs : Vec<NumRange> = Vec::new();
    day5::newMapRange(&maps[0], nr, &mut nrs);
    println!("{:?}", nrs);

    return 4;
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

    let text : &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";
    let splitText = text.split("\n").map(|s| {s.trim()}).collect();
    let result = f(splitText);
    println!("TestResult: {}", result);
}