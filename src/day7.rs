use std::{ops::Index, arch::x86_64::__get_cpuid_max};

pub fn d7(text : Vec<&str>) -> i32 {
    calculate(text, false)
}

fn handScore(hand : &str, jokers : bool) -> usize {
    comboScore(hand, jokers)  * 16usize.pow(5) + asHexNUm(hand, jokers)
}

fn comboScore(hand : &str, jokers : bool) -> usize {
    let mut occurences = vec![0; 13];
    hand.chars().for_each(|c| occurences[cardNum(c, jokers)] += 1);
    if (jokers) {
        placeJokers(&mut occurences)
    }
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

fn asHexNUm(hand : &str, jokers : bool) -> usize {
    hand.chars().enumerate().map(|(i, c)| cardNum(c, jokers) * (16usize.pow( 4 - i as u32))).sum()
}

fn cardNum(card : char, jokers : bool) -> usize {
    if jokers {
        return (match  card {
            'A' => 0xE,
            'K' => 0xD,
            'Q' => 0xC,
            'J' => 0x2,
            'T' => 0xB,
            _ => card as usize - 47
        }) - 2;
    }

    (match  card {
        'A' => 0xE,
        'K' => 0xD,
        'Q' => 0xC,
        'J' => 0xB,
        'T' => 0xA,
        _ => card as usize - 48
    }) - 2
}


fn placeJokers(occurences : &mut Vec<i32>) {
    let jokers = occurences[cardNum('J', true)];
    occurences[cardNum('J', true)] = 0;
    let max = occurences.iter().max().unwrap();
    let maxIndex = occurences.iter().position(|x| x == max).unwrap();
    occurences[maxIndex] += jokers;
    


}


pub fn d7p2(text : Vec<&str>) -> i32 {
    calculate(text, true)
}

fn calculate(text : Vec<&str>, jokers : bool) -> i32 {
    let hands = text.iter().map(|l| l.split_once(" ").unwrap());
    let mut valuedHands = hands.map(|(h, b)| (handScore(h, jokers), b.parse::<usize>().unwrap())).collect::<Vec<(usize, usize)>>();
    valuedHands.sort_by(|a,b| a.0.cmp(&b.0));
    valuedHands.iter().enumerate().fold(0, |t, n| t + ((n.0 + 1) * n.1.1)) as i32
}