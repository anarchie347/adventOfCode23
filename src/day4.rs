
use std::{num::ParseIntError, usize};

pub fn d4(text : Vec<&str>) -> i32{
    let mut total = 0;
    text.iter().for_each(|line : &&str| {
        let (leftFull, right) = line.split_once(" |").unwrap();
        let (_, left) = leftFull.split_once(":").unwrap();

        let winningNums : Vec<i32> = left.chars()
            .collect::<Vec<char>>()[1..]
            .chunks(3)
            .map(|a| a.iter().collect::<String>().trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        let scratchNums : Vec<i32> = right.chars()
            .collect::<Vec<char>>()[1..]
            .chunks(3)
            .map(|a| a.iter().collect::<String>().trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        let count : usize = winningNums.iter().filter(|w| scratchNums.contains(w)).count();
        if (count > 0) {
            total += 2usize.pow(count as u32 - 1);
        }
        
        //println!("Winning: {:?} --- {:?}", winningNums, scratchNums);
        //let Vec<nums>> = 10;
    });
    return total as i32;
}

pub fn d4p2(text : Vec<&str>) -> i32{
    let mut total = 0;
    let resultCards = text.iter().enumerate().map(|(index, line)| {
        let (leftFull, right) = line.split_once(" |").unwrap();
        let (_, left) = leftFull.split_once(":").unwrap();

        let winningNums : Vec<i32> = left.chars()
            .collect::<Vec<char>>()[1..]
            .chunks(3)
            .map(|a| a.iter().collect::<String>().trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        let scratchNums : Vec<i32> = right.chars()
            .collect::<Vec<char>>()[1..]
            .chunks(3)
            .map(|a| a.iter().collect::<String>().trim().parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        let count : usize = winningNums.iter().filter(|w| scratchNums.contains(w)).count();
        
        let mut toReturn : Vec<i32> = Vec::new();
        for i in 0..count {
            toReturn.push((index + 2 + i) as i32);
        }
        return toReturn;
    }).collect::<Vec<Vec<i32>>>();

    let mut cardQuants : Vec<i32> = vec![1; text.len()];
    for i in 0..cardQuants.len() {
        let gained = resultCards[i].clone();
        gained.iter().for_each(|n| {
            cardQuants[*n as usize - 1] += cardQuants[i];
        });
    }
    //let afterRound0 : Vec<i32> = (1..=6).collect();
    //let  mut afterRound1 : Vec<i32> = Vec::new();
    //afterRound0.iter().for_each(|c| afterRound1.extend(resultCards[(*c - 1) as usize].clone()));
    //println!("After1Pass : {:?}", afterRound1);
    //let mut afterRound2 : Vec<i32> = Vec::new();

    //afterRound1.iter().for_each(|c| afterRound2.extend(resultCards[(*c - 1) as usize].clone()));
    //return (resultCards.len() + afterRound1.len() + afterRound2.len()) as i32;
    return cardQuants.iter().fold(0, |x,y| x + y);
}