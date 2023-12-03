use core::num;

pub fn d3(text : Vec<&str>) -> i32 {
    let chars : Vec<Vec<char>> = text.iter().map(|l| {
        return l.chars().collect::<Vec<char>>();
}).collect::<Vec<Vec<char>>>();
    let mut y : usize = 0;
    let mut x: usize = 0;
    let mut total = 0;
    while y < chars.len() {
        while x < chars[y].len() {
            if !(chars[y][x] == '.' || chars[y][x].is_ascii_digit()) {
                // special char
                total += getNum(&chars, y -1,x - 1, x).unwrap_or(0);
                total += getNum(&chars, y -1, x, 5000).unwrap_or(0);
                total += getNum(&chars, y - 1, x + 1, x).unwrap_or(0);
                total += getNum(&chars, y, x - 1, 5000).unwrap_or(0);
                total += getNum(&chars, y, x + 1, 5000).unwrap_or(0);
                total += getNum(&chars, y + 1, x - 1, x).unwrap_or(0);
                total += getNum(&chars, y + 1, x, 5000).unwrap_or(0);
                total += getNum(&chars, y + 1, x + 1, x).unwrap_or(0);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    return total;
}

pub fn d3p2(text : Vec<&str>) -> i32 {
    let chars : Vec<Vec<char>> = text.iter().map(|l| {
        return l.chars().collect::<Vec<char>>();
}).collect::<Vec<Vec<char>>>();
    let mut y : usize = 0;
    let mut x: usize = 0;
    let mut total = 0;
    while y < chars.len() {
        while x < chars[y].len() {
            if chars[y][x] == '*' {
                // special char
                //println!("{}-{}", y, x);
                let mut nums : [Option<i32>; 8] = [None; 8];
                nums[0] = getNum(&chars, y -1,x - 1, x);
                nums[1] = getNum(&chars, y -1, x, 5000);
                nums[2] = getNum(&chars, y - 1, x + 1, x);
                nums[3] = getNum(&chars, y, x - 1, 5000);
                nums[4] = getNum(&chars, y, x + 1, 5000);
                nums[5] = getNum(&chars, y + 1, x - 1, x);
                nums[6] = getNum(&chars, y + 1, x, 5000);
                nums[7] = getNum(&chars, y + 1, x + 1, x);

                let collapsed : Vec<i32> = nums.iter().filter_map(|&opt| opt).collect();
                if (collapsed.len() == 2) {
                    total += collapsed[0] * collapsed[1];
                }
            }

            x += 1;
        }
        y += 1;
        x = 0;
    }
    return total;
}

fn getNum(chars : &Vec<Vec<char>>, y : usize, x : usize, forbiddenX : usize) -> Option<i32> {
    if !chars[y][x].is_ascii_digit() {
        return None;
    }
    let mut startX : i32 = x.try_into().unwrap();
    while (startX >= 0 && chars[y][startX as usize].is_ascii_digit()) {
        startX -= 1;
    }
    startX += 1;

    let mut endX : i32 = x.try_into().unwrap();
    while (endX < chars[y].len().try_into().unwrap() && chars[y][endX as usize].is_ascii_digit()) {
        endX += 1;
    }
    endX -= 1;

    let startXusize = startX.try_into().unwrap();
    let endXusize = endX.try_into().unwrap();

    if (forbiddenX >= startXusize && forbiddenX <= endXusize) {
        return None;
    }

    let numStr = chars[y][startXusize..=endXusize].iter().collect::<String>();
    return Some(numStr.parse::<i32>().unwrap())
}