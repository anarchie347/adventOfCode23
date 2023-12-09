use std::fmt::format;

pub fn d9(text : Vec<&str>) -> i32 {
    text.iter()
        .map(|l| {
            l.split(" ").map(|n| n.parse::<i32>().unwrap()).collect()
        })
        .map(|seq : Vec<i32>| {
            let mut diffferences : Vec<Vec<i32>> = Vec::new();
            diffferences.push(seq);
            while diffferences.last()
                .unwrap()
                .iter()
                .filter(|x| x != &&0)
                .count() > 0
                {
                    diffferences.push(
                        diffferences.last()
                        .unwrap()
                        .windows(2)
                        .map(|pair| {
                            pair[1] - pair[0]
                        })
                        .collect()
                    )
                }
            
            
            diffferences.iter()
                .map(|v| v.last().unwrap())
                .sum::<i32>()
        })
        .sum()

}

pub fn d9p2(text : Vec<&str>) -> i32 {
    text.iter()
        .map(|l| {
            l.split(" ").map(|n| n.parse::<i32>().unwrap()).collect()
        })
        .map(|seq : Vec<i32>| {
            let mut diffferences : Vec<Vec<i32>> = Vec::new();
            diffferences.push(seq);
            while diffferences.last()
                .unwrap()
                .iter()
                .filter(|x| x != &&0)
                .count() > 0
                {
                    diffferences.push(
                        diffferences.last()
                        .unwrap()
                        .windows(2)
                        .map(|pair| {
                            pair[1] - pair[0]
                        })
                        .collect()
                    )
                }
        
            diffferences.iter().rev().fold(0, |rt, y| y[0] - rt)
        })
        .sum()

}