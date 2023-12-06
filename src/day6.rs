

pub fn d6(text : Vec<&str>) -> i32 {
    //0: time
    //1: recordDist
    let data : Vec<(usize, usize)> = vec![
        ((&text[0][11..15].trim()).parse().unwrap(), (&text[1][11..15].trim()).parse().unwrap()),
        ((&text[0][18..22].trim()).parse().unwrap(), (&text[1][18..22].trim()).parse().unwrap()),
        ((&text[0][25..29].trim()).parse().unwrap(), (&text[1][25..29].trim()).parse().unwrap()),
        ((&text[0][32..36].trim()).parse().unwrap(), (&text[1][32..36].trim()).parse().unwrap())
    ];
    // let data : Vec<(usize, usize)> = vec![
    //     (7,9),
    //     (15,40),
    //     (30,200)
    // ];

    data.iter().map(|d| {
        let sqrt_discrim = ((d.0 * d.0 - 4 * d.1) as f32).sqrt();
        let sol1 = (d.0 as f32 + sqrt_discrim) / 2f32;
        let sol2 = (d.0 as f32 - sqrt_discrim) / 2f32;
        ((sol1 - 1f32).ceil() - (sol2 +  1f32).floor()) as i32 + 1
        //println!("{}", recordTime);
        //(bestTime - recordTime) * .2
    }).product()

    // return data.iter().map(|d| {
    //     let maxDist = ((d.0 * d.0) as f32 * 0.25f32) as usize;
    //     2 * (maxDist - d.1)
    // }).product::<usize>() as i32;
}
pub fn d6p2(text : Vec<&str>) -> i32 {
    //0: time
    //1: recordDist
    let time : usize = text[0][11..].replace(" ", "").parse().unwrap();
    let dist : usize = text[1][11..].replace(" ", "").parse().unwrap();
    // let data : Vec<(usize, usize)> = vec![
    //     (7,9),
    //     (15,40),
    //     (30,200)
    // ];
        let sqrt_discrim = ((time * time - 4 * dist) as f64).sqrt();
        let sol1 = (time as f64 + sqrt_discrim) / 2f64;
        let sol2 = (time as f64 - sqrt_discrim) / 2f64;
        ((sol1 - 1f64).ceil() - (sol2 +  1f64).floor()) as i32 + 1
        //println!("{}", recordTime);
        //(bestTime - recordTime) * .2
    

    // return data.iter().map(|d| {
    //     let maxDist = ((d.0 * d.0) as f32 * 0.25f32) as usize;
    //     2 * (maxDist - d.1)
    // }).product::<usize>() as i32;
}