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
            
            let mut extrapolated = 0;
            diffferences.iter().for_each(|diff| {
                extrapolated += diff.last().unwrap();
            });
            
            extrapolated
            // for diffs in diffferences.iter() {
            //     print!("{:?}", diffs)
            // }
            // println!("E");
            // println!(" ");
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
            
            let mut extrapolated = 0;
            diffferences.iter().rev().for_each(|diff| {
                extrapolated =  diff[0] - extrapolated; //i am traversing the wrong way through the list to save efficiency, so the order of the subtractions needs to be swapped
            });
            //println!("{}", extrapolated);
            extrapolated
            // for diffs in diffferences.iter() {
            //     print!("{:?}", diffs)
            // }
            // println!("E");
            // println!(" ");
        })
        .sum()

}