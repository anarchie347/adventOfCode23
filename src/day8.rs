use std::collections::{HashMap, hash_map};

pub fn d8(text : Vec<&str>) -> i32 {
    let path = text[0];
    //println!("{}", path);
   let hashmap : HashMap<&str, (&str, &str)> =  text[2..].iter().map(|l| {
        let (node, dirs) = l.split_once(" = ").unwrap();
        let left = &dirs[1..4];
        let right = &dirs[6..9];
        (node, (left, right))
    }).collect();
    let mut current : &str = "AAA";
    let mut count = 0;
    let mut index : usize = 0;
    let chars : Vec<char> = path.chars().collect();
    while current != "ZZZ" {
        match chars[index] {
            'L' => current = hashmap[current].0,
            'R' => current = hashmap[current].1,
            _ => unreachable!()
        }
        count += 1;
        index += 1;
        index = index % path.len();
        //println!("{}-{}-{}", current, count, index);
    }


    return count;
}

pub fn d8p2OLD(text : Vec<&str>) -> i32 {
    let path = text[0];

    let mut mapping : Vec<(usize, usize)> = vec![(18000, 18000); 17576];
    let mut current : Vec<usize> = Vec::new();
    for l in text[2..].iter() {
        let (node, dirs) = l.split_once(" = ").unwrap();
        let dchars : Vec<char> = dirs.chars().collect();
        let left = (dchars[1] as usize - 65) * 676 + (dchars[2] as usize - 65) * 26 + (dchars[3] as usize - 65);
        let right = (dchars[6] as usize - 65) * 676 + (dchars[7] as usize - 65) * 26 + (dchars[8] as usize - 65);
        let nodeNum = (node.chars().nth(0).unwrap() as usize - 65) * 676
            + (node.chars().nth(1).unwrap() as usize - 65) * 26
            + node.chars().nth(2).unwrap() as usize - 65;
        //println!("{}-{}",node, dirs);
        //println!("{}+{}+{}", (node.chars().nth(0).unwrap() as usize * 676), (node.chars().nth(1).unwrap() as usize * 26), )
        mapping[nodeNum] =  (left, right);

        if (node.chars().nth(2).unwrap() == 'A') {
            current.push(nodeNum)
        }
    }

    println!("PArse donw");
    
    //mapping.iter().for_each(|k| if k % 32 == 1 { current.push(*k)});
    //(0..17576).step_by(26).for_each(|n| if mapping[n].0 < 18000 { current.push(n)});
    println!("{} seeds", current.len());
    let mut count = 0;
    let mut index : usize = 0;
    let chars : Vec<char> = path.chars().collect();
    while current.iter().filter(|n| *n % 26 < 25).count() > 0 {
        //print!("{}", chars[index]);
        println!("{:?}", current.iter().map(|n| *n % 26).collect::<Vec<usize>>());
        println!("Current{:?}", current);
        current = current.iter().map(|n| {
            return match chars[index] {
                'L' => mapping[*n].0,
                'R' => mapping[*n].1,
                _ => unreachable!()
            };
        }).collect();
        count += 1;
        index += 1;
        
        if (index == path.len()) {
           // println!("restart");
            index = 0;
        }
        //println!("{}-{}-{}", current, count, index);
    }

    return  count;

}

pub fn d8p2(text : Vec<&str>) -> i64 {
    let path = text[0];

    let mut mapping : Vec<(usize, usize)> = vec![(18000, 18000); 17576];
    let mut starts : Vec<usize> = Vec::new();
    for l in text[2..].iter() {
        let (node, dirs) = l.split_once(" = ").unwrap();
        let dchars : Vec<char> = dirs.chars().collect();
        let left = (dchars[1] as usize - 65) * 676 + (dchars[2] as usize - 65) * 26 + (dchars[3] as usize - 65);
        let right = (dchars[6] as usize - 65) * 676 + (dchars[7] as usize - 65) * 26 + (dchars[8] as usize - 65);
        let nodeNum = (node.chars().nth(0).unwrap() as usize - 65) * 676
            + (node.chars().nth(1).unwrap() as usize - 65) * 26
            + node.chars().nth(2).unwrap() as usize - 65;
        //println!("{}-{}",node, dirs);
        //println!("{}+{}+{}", (node.chars().nth(0).unwrap() as usize * 676), (node.chars().nth(1).unwrap() as usize * 26), )
        mapping[nodeNum] =  (left, right);

        if (node.chars().nth(2).unwrap() == 'A') {
            starts.push(nodeNum)
        }
    }

    //println!("PArse donw");
    
    //mapping.iter().for_each(|k| if k % 32 == 1 { current.push(*k)});
    //(0..17576).step_by(26).for_each(|n| if mapping[n].0 < 18000 { current.push(n)});
    //println!("{} seeds", starts.len());
    
    let chars : Vec<char> = path.chars().collect();
    starts.iter().map(|node| {
        let mut count = 0;
    let mut index : usize = 0;
        let mut current = node;
        while (current % 26 < 25) {
            current = match chars[index] {
                    'L' => &mapping[*current].0,
                    'R' => &mapping[*current].1,
                    _ => unreachable!()
                };
            count += 1;
            index += 1;
            
            index = index % path.len();
        }
        //println!("{}", count);
        //println!("{}-{}-{}", current, count, index);
    count
    }).reduce(lcm).unwrap()

}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}