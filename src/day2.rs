use std;
pub fn day2e(text : Vec<&str>) {
    let mut count : i32 = 0;
    text.iter().for_each(|line : &&str| {
        let (left, right) = line.split_once(":").unwrap();
        let id : i32 = left.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let grabs : Vec<&str> = right.split(";").collect();
        const MAX_RED : i32 = 12;
        const MAX_GREEN : i32 = 13;
        const MAX_BLUE : i32 = 14;
        let mut valid = true;
        grabs.iter().for_each(|s : &&str| {
            let cubeInfos : Vec<String> = s.split(",").map(|x : &str| {x.get(1..).unwrap().to_string()}).collect();
            cubeInfos.iter().for_each(|cI : &String| {
                println!("{}", cI);
                let (amount, colour) = cI.split_once(" ").unwrap();
                let amountInt : i32 = amount.parse::<i32>().unwrap();
                println!("{}, {}", colour, amount);
                if colour == "red" && amount.parse::<i32>().unwrap() > MAX_RED {
                    valid = false;
                }
                if colour == "green" && amount.parse::<i32>().unwrap() > MAX_GREEN {
                    valid = false;
                }
                if colour == "blue" && amount.parse::<i32>().unwrap() > MAX_BLUE {
                    valid = false;
                }
            });
        });
        if valid {
            count += id;
            println!("W          W{}", id)
        }
    });
    print!("e{}", count);
}

pub fn day2p2(text : Vec<&str>) {
    let mut count : i32 = 0;
    text.iter().for_each(|line : &&str| {
        let (left, right) = line.split_once(":").unwrap();
        let id : i32 = left.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let grabs : Vec<&str> = right.split(";").collect();
        let mut maxRed = 0;
        let mut maxBlue = 0;
        let mut maxGreen = 0;
        grabs.iter().for_each(|s : &&str| {
            let cubeInfos : Vec<String> = s.split(",").map(|x : &str| {x.get(1..).unwrap().to_string()}).collect();
            cubeInfos.iter().for_each(|cI : &String| {
                println!("{}", cI);
                let (amount, colour) = cI.split_once(" ").unwrap();
                let amountInt : i32 = amount.parse::<i32>().unwrap();
                println!("{}, {}", colour, amount);
                if colour == "red" && amountInt > maxRed {
                    maxRed = amountInt
                }
                if colour == "green" && amountInt > maxGreen {
                    maxGreen = amountInt
                }
                if colour == "blue" && amountInt > maxBlue {
                    maxBlue = amountInt
                }
            });
        });
            count += (maxRed * maxBlue * maxGreen);
    });
    print!("e{}", count);
}