#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut min_x: usize = 500;
    let mut max_x: usize = 500;
    let mut max_depth: usize = 0;
    let mut sand_map: Vec<Vec<bool>> = [[false].to_vec()].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" -> ").collect::<Vec<&str>>();
                for k in 0..(split.len()-1) {
                    let start_coord = split[k].split(",").collect::<Vec<&str>>();
                    let end_coord = split[k+1].split(",").collect::<Vec<&str>>();
                    let mut x1: usize = start_coord[0].parse().unwrap();
                    let mut y1: usize = start_coord[1].parse().unwrap();
                    let mut x2: usize = end_coord[0].parse().unwrap();
                    let mut y2: usize = end_coord[1].parse().unwrap();
                    if max_depth < y1+2 {max_depth = y1+2;}
                    if max_depth < y2+2 {max_depth = y2+2;}
                    let mut n_i = sand_map.len();
                    let mut n_j = sand_map[0].len();
                    for i in 0..n_i {
                        for j in n_j..max_depth {
                            sand_map[i].push(false);
                        }
                    }
                    n_j = sand_map[0].len();
                    
                    if min_x > x1 {min_x = x1;}
                    if min_x > x2 {min_x = x2;}
                    for i in n_i..=(max_x-min_x) {
                        sand_map.insert(0,[].to_vec());
                        for j in 0..max_depth {
                            sand_map[0].push(false);
                        }
                    }
                    n_i = sand_map.len();

                    if max_x < x1 {max_x = x1;}
                    if max_x < x2 {max_x = x2;}
                    for i in n_i..=(max_x-min_x) {
                        sand_map.push([].to_vec());
                        for j in 0..max_depth {
                            sand_map[n_i].push(false);
                        }
                        n_i += 1;
                    }
                    if x1> x2 {
                        (x1,x2) = (x2,x1);
                    }
                    if y1> y2 {
                        (y1,y2) = (y2,y1);
                    }
                    //println!("x1:{}, y1:{}, x2:{}, y2{}",x1,y1,x2,y2);
                    for i in x1..=x2 {
                        //println!("  i:{}",i);
                        for j in y1..=y2 {
                            sand_map[i-min_x][j] = true;
                            //println!("      j:{}",j);
                        }
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    for j in 0..max_depth {
        for i in min_x..=max_x {
            if sand_map[i-min_x][j]{
                print!("#");
            }else{
                print!("_");
            }
        }
        print!("\n");
    }
    let mut units_of_sand = 0;
    let mut out_of_bound = false;
    while !out_of_bound {
        let mut sand_x = 500;
        let mut sand_y = 0;
        let mut grounded = false;
        while !grounded{
            if sand_y+1 == max_depth{
                out_of_bound = true;
                break;
            }
            if !sand_map[sand_x-min_x][sand_y+1] {
                sand_y += 1;
                continue;
            }
            if sand_x == min_x {
                out_of_bound = true;
                break;
            }
            if !sand_map[sand_x-min_x-1][sand_y+1] {
                sand_y += 1;
                sand_x -= 1;
                continue;
            }
            if sand_x == max_x {
                out_of_bound = true;
                break;
            }
            if !sand_map[sand_x-min_x+1][sand_y+1] {
                sand_y += 1;
                sand_x += 1;
                continue;
            }
            sand_map[sand_x-min_x][sand_y] = true;
            grounded = true;
            units_of_sand += 1;
        }
    }
    for j in 0..max_depth {
        for i in min_x..=max_x {
            if sand_map[i-min_x][j]{
                print!("#");
            }else{
                print!("_");
            }
        }
        print!("\n");
    }
    println!("result : {}",units_of_sand);
    
}

pub fn part2() {
    let mut min_x: usize = 500;
    let mut max_x: usize = 500;
    let mut max_depth: usize = 0;
    let mut sand_map: Vec<Vec<bool>> = [[false].to_vec()].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" -> ").collect::<Vec<&str>>();
                for k in 0..(split.len()-1) {
                    let start_coord = split[k].split(",").collect::<Vec<&str>>();
                    let end_coord = split[k+1].split(",").collect::<Vec<&str>>();
                    let mut x1: usize = start_coord[0].parse().unwrap();
                    let mut y1: usize = start_coord[1].parse().unwrap();
                    let mut x2: usize = end_coord[0].parse().unwrap();
                    let mut y2: usize = end_coord[1].parse().unwrap();
                    if max_depth < y1+2 {max_depth = y1+2;}
                    if max_depth < y2+2 {max_depth = y2+2;}
                    let mut n_i = sand_map.len();
                    let mut n_j = sand_map[0].len();
                    for i in 0..n_i {
                        for j in n_j..max_depth {
                            sand_map[i].push(false);
                        }
                    }
                    n_j = sand_map[0].len();
                    
                    if min_x > x1 {min_x = x1;}
                    if min_x > x2 {min_x = x2;}
                    for i in n_i..=(max_x-min_x) {
                        sand_map.insert(0,[].to_vec());
                        for j in 0..max_depth {
                            sand_map[0].push(false);
                        }
                    }
                    n_i = sand_map.len();

                    if max_x < x1 {max_x = x1;}
                    if max_x < x2 {max_x = x2;}
                    for i in n_i..=(max_x-min_x) {
                        sand_map.push([].to_vec());
                        for j in 0..max_depth {
                            sand_map[n_i].push(false);
                        }
                        n_i += 1;
                    }
                    if x1> x2 {
                        (x1,x2) = (x2,x1);
                    }
                    if y1> y2 {
                        (y1,y2) = (y2,y1);
                    }
                    //println!("x1:{}, y1:{}, x2:{}, y2{}",x1,y1,x2,y2);
                    for i in x1..=x2 {
                        //println!("  i:{}",i);
                        for j in y1..=y2 {
                            sand_map[i-min_x][j] = true;
                            //println!("      j:{}",j);
                        }
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }

    let mut n_i = sand_map.len();
    if min_x+max_depth+1 > min_x {min_x = min_x-max_depth-1;}
    for i in n_i..=(max_x-min_x) {
        sand_map.insert(0,[].to_vec());
        for j in 0..max_depth {
            sand_map[0].push(false);
        }
    }
    n_i = sand_map.len();

    if max_x < max_x+max_depth+1 {max_x = max_x+max_depth+1;}
    for i in n_i..=(max_x-min_x) {
        sand_map.push([].to_vec());
        for j in 0..max_depth {
            sand_map[n_i].push(false);
        }
        n_i += 1;
    }

    for j in 0..max_depth {
        for i in min_x..=max_x {
            if sand_map[i-min_x][j]{
                print!("#");
            }else{
                print!("_");
            }
        }
        print!("\n");
    }
    print!("\n");
    let mut units_of_sand = 0;
    let mut out_of_bound = false;
    while !sand_map[500-min_x][0] {
        let mut sand_x = 500;
        let mut sand_y = 0;
        let mut grounded = false;
        while !grounded{
            if sand_y+1 == max_depth{
                sand_map[sand_x-min_x][sand_y] = true;
                grounded = true;
                units_of_sand += 1;
                continue;
            }
            if !sand_map[sand_x-min_x][sand_y+1] {
                sand_y += 1;
                continue;
            }
            if !sand_map[sand_x-min_x-1][sand_y+1] {
                sand_y += 1;
                sand_x -= 1;
                continue;
            }
            if !sand_map[sand_x-min_x+1][sand_y+1] {
                sand_y += 1;
                sand_x += 1;
                continue;
            }
            sand_map[sand_x-min_x][sand_y] = true;
            grounded = true;
            units_of_sand += 1;
        }
    }
    for j in 0..max_depth {
        for i in min_x..=max_x {
            if sand_map[i-min_x][j]{
                print!("#");
            }else{
                print!("_");
            }
        }
        print!("\n");
    }
    print!("\n");
    println!("result : {}",units_of_sand);
    
}