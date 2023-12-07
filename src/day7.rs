use std::{ops::Index, arch::x86_64::__get_cpuid_max};

pub fn d7(text : Vec<&str>) -> i32 {
    let hands = text.iter().map(|l| l.split_once(" ").unwrap());
    let mut valuedHands = hands.map(|(h, b)| (handScore(h), b.parse::<usize>().unwrap())).collect::<Vec<(usize, usize)>>();

    valuedHands.sort_by(|a,b| a.0.cmp(&b.0));
    valuedHands.iter().enumerate().fold(0, |t, n| t + ((n.0 + 1) * n.1.1)) as i32
}

fn handScore(hand : &str) -> usize {
    comboScore(hand)  * 16usize.pow(5) + asHexString(hand)
}

fn comboScore(hand : &str) -> usize {
    let mut occurences = vec![0; 13];
    hand.chars().for_each(|c| occurences[cardNum(c)] += 1);
    let max = occurences.iter().max().unwrap();
    if max == &3 && occurences.contains(&2) {
        return 4
    }
    if max == &2 && occurences.iter().filter(|x| x == &&2).count() >= 2 {
        return 2;
    }
    (match max {
        5 => 6,
        4 => 5,
        3 => 3,
        2 => 1,
        _ => 0
    })
}

fn asHexString(hand : &str) -> usize {
    hand.chars().enumerate().map(|(i, c)| cardNum(c) * (16usize.pow( 4 - i as u32))).sum()
}

fn cardNum(card : char) -> usize {
    (match  card {
        'A' => 0xE,
        'K' => 0xD,
        'Q' => 0xC,
        'J' => 0xB,
        'T' => 0xA,
        _ => card as usize - 48
    }) - 2
}
fn cardNumP2(card : char) -> usize {
    (match  card {
        'A' => 0xE,
        'K' => 0xD,
        'Q' => 0xC,
        'J' => 0x2,
        'T' => 0xB,
        _ => card as usize - 47
    }) - 2
}

fn handScoreP2(hand : &str) -> usize {
    asHexStringP2(hand) + comboScoreP2(hand)  * 16usize.pow(5)
}

fn comboScoreP2(hand : &str) -> usize {
    let mut occurences = vec![0; 13];
    
    hand.chars().for_each(|c| occurences[cardNum(c)] += 1);
    placeJokers(&mut occurences);
    let max = occurences.iter().max().unwrap();
    if max == &3 && occurences.contains(&2) {
        return 4
    }
    if max == &2 && occurences.iter().filter(|x| x == &&2).count() >= 2 {
        return 2;
    }
    (match max {
        5 => 6,
        4 => 5,
        3 => 3,
        2 => 1,
        _ => 0
    })
}

fn placeJokers(occurences : &mut Vec<i32>) {
    let jokers = occurences[cardNum('J')];
    occurences[cardNum('J')] = 0;
    let max = occurences.iter().max().unwrap();
    let maxIndex = occurences.iter().position(|x| x == max).unwrap();
    occurences[maxIndex] += jokers;
    


}

fn asHexStringP2(hand : &str) -> usize {
    hand.chars().enumerate().map(|(i, c)| cardNumP2(c) * (16usize.pow( 4 - i as u32))).sum()
}

pub fn d7p2(text : Vec<&str>) -> i32 {
    let hands = text.iter().map(|l| l.split_once(" ").unwrap());
    let mut valuedHands = hands.map(|(h, b)| (handScoreP2(h), b.parse::<usize>().unwrap())).collect::<Vec<(usize, usize)>>();
    valuedHands.sort_by(|a,b| a.0.cmp(&b.0));
    valuedHands.iter().enumerate().fold(0, |t, n| t + ((n.0 + 1) * n.1.1)) as i32
}