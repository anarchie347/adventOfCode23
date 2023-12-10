use colored::*;

pub fn d10(text : Vec<&str>) -> i32 {
    let mut coord = (55000,5000);
    let map = text.iter().enumerate().map(|(i, s)| {
        if let Some(index) = s.find('S') {
            coord = (index, i);
            //println!("{}", s);
            //println!("{}-{},{}", s.chars().nth(index).unwrap(), index, i);
        }
        s.chars().collect::<Vec<char>>()      
    }).collect::<Vec<Vec<char>>>();
    //println!("E{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    let start: (usize, usize) = coord;
    
    let mut past_coord = coord;
    coord = (coord.0, coord.1 - 1); //up

    let mut symbol = *map.get(coord.1).and_then(|v| v.get(coord.0)).unwrap_or(&'X');
    let mut found = false;
    
    if symbol == '|' || symbol == '7' || symbol == 'F' {
        found = true
    }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    if !found {
        coord = (coord.0 + 1, coord.1 + 1); //right
        symbol = *map.get(coord.1).and_then(|v| v.get(coord.0)).unwrap_or(&'X');
        if symbol == '-' || symbol == 'J' || symbol == '7' {
            found = true
        }
    }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    if !found {
        coord = (coord.0 - 1, coord.1 + 1);//down
        symbol= *map.get(coord.1).and_then(|v| v.get(coord.0)).unwrap_or(&'X');
        if symbol == '|' || symbol == 'L' || symbol == 'J' {
            found = true
        }
    }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    if !found {
        coord = (coord.0 - 1, coord.1 - 1); //left
    }

    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    let mut path_length = 1; //include start
    while coord != start {
        (coord, past_coord) = traverse(&map, coord, past_coord);
        path_length += 1;
    }
    (path_length) / 2

}

fn traverse(map : &Vec<Vec<char>>, (x, y) : (usize, usize), (prev_x, prev_y) : (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let new_coord = match map[y][x] {
        '|' => if prev_y > y {(x, y - 1)} else {(x, y + 1)},
        '-' => if prev_x > x {(x - 1, y)} else {(x + 1, y)},
        'L' => if prev_y < y {(x + 1, y)} else {(x, y - 1)},
        'J' => if prev_y < y {(x - 1, y)} else {(x, y - 1)},
        '7' => if prev_y > y {(x - 1, y)} else {(x, y + 1)},
        'F' => if prev_y > y {(x + 1, y)} else {(x, y + 1)},
        _ => {
            println!("{}", map[y][x]);
            (0,0)
        }
    };
    (new_coord, (x, y))
}

pub fn d10p2(text : Vec<&str>) -> i32 {
    let mut coord = (55000,5000);
    let mut map = text.iter().enumerate().map(|(i, s)| {
        if let Some(index) = s.find('S') {
            coord = (index, i);
            //println!("{}", s);
            //println!("{}-{},{}", s.chars().nth(index).unwrap(), index, i);
        }
        s.chars().collect::<Vec<char>>()     
    }).collect::<Vec<Vec<char>>>();
    //println!("E{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    let start: (usize, usize) = coord;
    
    
    let mut past_coord = coord;

    let mut symbol = *map.get(coord.1).and_then(|v| v.get(coord.0)).unwrap_or(&'X');
    let mut connectionUp = false;
    let mut connectionRight = false;
    let mut connectionDown = false;
    let mut connectionLeft = false;
    let mut tryCoord = (start.0, start.1 - 1); //up
    
    if symbol == '|' || symbol == '7' || symbol == 'F' {
        coord = tryCoord;
        connectionUp = true;
    }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
        tryCoord = (start.0 + 1, start.1); //right
        symbol = *map.get(tryCoord.1).and_then(|v| v.get(tryCoord.0)).unwrap_or(&'X');
        if symbol == '-' || symbol == 'J' || symbol == '7' {
            coord = tryCoord;
            connectionRight = true;
        }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
        tryCoord = (start.0, start.1 + 1);//down
        symbol= *map.get(tryCoord.1).and_then(|v| v.get(tryCoord.0)).unwrap_or(&'X');
        if symbol == '|' || symbol == 'L' || symbol == 'J' {
            coord = tryCoord;
            connectionDown = true;
        }
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
        tryCoord = (start.0 - 1, start.1); //left
        symbol= *map.get(tryCoord.1).and_then(|v| v.get(tryCoord.0)).unwrap_or(&'X');
        if symbol == 'F' || symbol == 'L' || symbol == '-' {
            coord = tryCoord;
            connectionLeft = true;
        }
    map[start.1][start.0] = determine_start_char(connectionUp, connectionRight, connectionDown, connectionLeft);
    //println!("{}", determine_start_char(connectionUp, connectionRight, connectionDown, connectionLeft));
    //println!("{},{},{}", coord.0, coord.1, map[coord.1][coord.0]);
    let mut path_length = 1; //inculde start
    let mut path : Vec<(usize, usize)> = Vec::new();
    path.push(start);
    
    let mut cleared_map = vec![vec!['.'; map[0].len()]; map.len()];
    cleared_map[start.1][start.0] = map[start.1][start.0];
    while coord != start {
        path.push(coord);
        cleared_map[coord.1][coord.0] = map[coord.1][coord.0];
        (coord, past_coord) = traverse(&map, coord, past_coord);
        
        path_length += 1;
    }

    let mut cleared_expanded_walls = vec![vec![false; map[0].len() * 2 - 1]; map.len() * 2 - 1];
    //true if wall

    for y in 0..cleared_expanded_walls.len() {
        for x in 0..cleared_expanded_walls[y].len() {
            //consider the small section
            //0,0 1,0
            //0,1 1,1
            //each if section deals with a certain part
            if (y % 2) + (x % 2) == 0 { //0,0
                cleared_expanded_walls[y][x] = cleared_map[y / 2][x / 2] != '.';
            } else if (y + x) % 2 == 0 { //1,1
                cleared_expanded_walls[y][x] = false;
            } else {
                let adj_x = x / 2;
                let adj_y = y / 2;
                let adj_symbol = cleared_map[adj_y][adj_x]; //0,0
                if y % 2 == 0 { //1,0
                    cleared_expanded_walls[y][x] = "-LF".contains(adj_symbol);
                } else { //0,1
                    cleared_expanded_walls[y][x] = "|7F".contains(adj_symbol);
                }
                

            }
        }
    }
    //opens up between pipe gaps

    let width = cleared_expanded_walls[0].len();
    cleared_expanded_walls.insert(0, vec![false; width]);
    cleared_expanded_walls.push(vec![false; width]);
    for i in 0..cleared_expanded_walls.len() {
        cleared_expanded_walls[i].insert(0, false);
        cleared_expanded_walls[i].push(false);
    }
    //wraps the cleared exapnded walls in container of empty
    
    let mut explored : Vec<(usize, usize)> = Vec::new();
    let expanded_width = width + 2;
    let expanded_height = cleared_expanded_walls.len();
    explored.extend((0..expanded_width).map(|x| (x, 0)));
    explored.extend((0..expanded_width).map(|x| (x, expanded_height - 1)));
    explored.extend((1..(expanded_height - 1)).map(|y| (0, y)));
    explored.extend((1..(expanded_height - 1)).map(|y| (expanded_width - 1, y)));
    
    let size = (expanded_width, expanded_height);
    let mut og_map_explored = 0;
    let mut new_discovered = true;
    while new_discovered {
        new_discovered = false;
        let temp_explored = explored.clone();
        for coord in temp_explored{
            let up = (coord.0, coord.1 - 1);
            let right = (coord.0 + 1, coord.1);
            let down = (coord.0, coord.1 + 1);
            let left = (coord.0 - 1, coord.1); 
            if is_valid_coord(up, size) && !cleared_expanded_walls[up.1][up.0] && !explored.contains(&up)  {
                explored.push(up);
                if (is_og_map(up)) {
                    og_map_explored += 1;
                }
                new_discovered = true;
            }
            if is_valid_coord(right, size)&& !cleared_expanded_walls[right.1][right.0] && !explored.contains(&right) {
                explored.push(right);
                if (is_og_map(right)) {
                    og_map_explored += 1;
                }
                new_discovered = true;
            }
            if is_valid_coord(down, size)&& !cleared_expanded_walls[down.1][down.0] && !explored.contains(&down) {
                explored.push(down);
                if (is_og_map(down)) {
                    og_map_explored += 1;
                }
                new_discovered = true;
            }
            if is_valid_coord(left, size)&& !cleared_expanded_walls[left.1][left.0] && !explored.contains(&left) {
                explored.push(left);
                if (is_og_map(left)) {
                    og_map_explored += 1;
                }
                new_discovered = true;
            }
        }

        // for y in 0..cleared_expanded_walls.len() {
        //     for x in 0..cleared_expanded_walls[0].len() {
        //         if cleared_expanded_walls[y][x] {
        //             print!("{}", "X".on_red());
        //         }
        //         else if explored.contains(&(x,y)) {
        //             print!("{}", ".".on_green())
        //         } else {
        //             print!("{}", ".".on_purple())
        //         }
        //     }
        //     println!();
        // }
        
        // std::thread::sleep(std::time::Duration::from_secs(2));
        // println!("\n\n\n\n");
    }


// for l in cleared_expanded_walls {
//         println!("{}", l.iter().map(|w| if *w {'X'} else {'.'}).collect::<String>());
//     }

for y in 0..cleared_expanded_walls.len() {
    for x in 0..cleared_expanded_walls[0].len() {
        if cleared_expanded_walls[y][x] {
            print!("{}", "X".on_red());
        }
        else if explored.contains(&(x,y)) {
            print!("{}", ".")
        } else {
            print!("{}", ".".on_green())
        }
    }
    println!();
}

    let og_map_size = map.len() * map[0].len();
    return (og_map_size - path_length - og_map_explored) as i32;



    
    // for l in cleared_map {
    //     println!("{}", l.iter().collect::<String>());
    // }
        
}

fn is_valid_coord((x, y) : (usize, usize), (len_x, len_y) : (usize, usize)) -> bool {
    x > 0
    && x < len_x
    && y > 0
    && y < len_y
}
fn is_og_map((x,y) : (usize, usize)) -> bool {
    (x - 1) % 2 == 0 && (y - 1) % 2 == 0
}

fn determine_start_char(u : bool, r : bool, d : bool, l : bool) -> char {
    println!("{},{},{},{}", u, r, d, l);
    if u && d {
        '|'
    } else if l && r {
        '-'
    } else if u && r {
        'L'
    } else if u && l {
        'J'
    } else if l && d {
        '7'
    } else if r && d {
        'F'
    } else {
        '.'
    }
}