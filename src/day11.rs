
pub fn d11(text : Vec<&str>) -> i32 {
    let mut exta_x : Vec<usize> = Vec::new();
    for x in 0..text[0].len() {

        let mut empty = true;
        for y in 0..text.len() {
            if text[y].chars().nth(x).unwrap() == '#' {
                empty = false;
            }
        }
        exta_x.push(
            *(exta_x.last().unwrap_or(&0)) + empty as usize
        );
    }

    let mut extra_y = 0;
    let mut galaxies : Vec<(usize, usize)> = Vec::new();
    text.iter().enumerate().for_each(|(y, row)| {
        if row.contains('#') {
            galaxies.extend(row.chars()
            .enumerate()
            .filter_map(|(i, elem)| if elem == '#' {Some((i+ exta_x[i], y + extra_y))} else {None})

        );
        } else {
            extra_y += 1;
        }    
    });

    let mut total_dist = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            total_dist += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }


    total_dist as i32
}


pub fn d11p2(text : Vec<&str>) -> i64 {
    let mut exta_x : Vec<usize> = Vec::new();
    for x in 0..text[0].len() {

        let mut empty = true;
        for y in 0..text.len() {
            if text[y].chars().nth(x).unwrap() == '#' {
                empty = false;
            }
        }
        exta_x.push(
            *(exta_x.last().unwrap_or(&0)) + (empty as usize * 999_999)
        );
    }

    let mut extra_y = 0;
    let mut galaxies : Vec<(usize, usize)> = Vec::new();
    text.iter().enumerate().for_each(|(y, row)| {
        if row.contains('#') {
            galaxies.extend(row.chars()
            .enumerate()
            .filter_map(|(i, elem)| if elem == '#' {Some((i+ exta_x[i], y + extra_y))} else {None})
        );
        } else {
            extra_y += 999_999;
        }    
    });

    let mut total_dist = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            total_dist += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }

    println!("{}", total_dist);
    total_dist as i64
}