#[path = "../../utils/mod.rs"] mod utils;


pub fn part1() {
    let mut wind_left_to_right: Vec<bool> = [].to_vec();
    const cave_width:usize = 7;
    const nb_shapes: usize = 5;
    let mut cave: Vec<[bool;cave_width]> = [[true,true,true,true,true,true,true]].to_vec();
    for _ in 0..7 {
        cave.push([false,false,false,false,false,false,false]);
    }
    let rocks: [Vec<Vec<bool>>;nb_shapes] = [
        [
            [true,true,true,true].to_vec()
        ].to_vec(),
        [
            [false,true,false].to_vec(),
            [true,true,true].to_vec(),
            [false,true,false].to_vec()
        ].to_vec(),
        [
            [false,false,true].to_vec(),
            [false,false,true].to_vec(),
            [true,true,true].to_vec()
        ].to_vec(),
        [
            [true].to_vec(),
            [true].to_vec(),
            [true].to_vec(),
            [true].to_vec()
        ].to_vec(),
        [
            [true,true].to_vec(),
            [true,true].to_vec()
        ].to_vec()
    ];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                for c in line_content.chars(){
                    wind_left_to_right.push(c == '>');
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let nb_rocks: usize = 2022;
    let mut height = 0;
    let mut wind_count: usize = 0;
    let wind_size = wind_left_to_right.len();
    for k in 0..nb_rocks{
        let rock_width = rocks[k%nb_shapes][0].len();
        let rock_height = rocks[k%nb_shapes].len();
        let mut x: usize = 2;
        let mut y: usize = height + 4;
        let mut grounded = false;
        // create enough room
        while cave.len() < height + 8{
            cave.push([false,false,false,false,false,false,false]); 
        }
        while !grounded{
            let wind_dir: i32 = if(wind_left_to_right[wind_count]){1}else{-1};
            wind_count = (wind_count + 1)%wind_size;
            // wind action
            let mut can_be_pushed = false;
            if (x > 0) && (wind_dir < 0){
                can_be_pushed = true;
                for j in 0..rock_height{
                    for i in 0..rock_width{
                        if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j][x+i-1]{
                            can_be_pushed = false;
                            break;
                        }
                    }
                }
            }
            if (x + rock_width < cave_width) && (wind_dir > 0){
                can_be_pushed = true;
                for j in 0..rock_height{
                    for i in 0..rock_width{
                        if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j][x+i+1]{
                            can_be_pushed = false;
                            break;
                        }
                    }
                }
            }
            if can_be_pushed{
                x = ((x as i32) + wind_dir) as usize;
            }
            //gravity action
            for i in 0..rock_width{
                for j in 0..rock_height{
                    if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j-1][x+i]{
                        grounded = true;
                        break;
                    }
                }
                if grounded { break; }
            }
            if !grounded {
                y -= 1;
            }
        }
        //update height
        if y + rock_height - 1 > height{
            height = y + rock_height - 1;
        }
        // add blocks
        for i in 0..rock_width{
            for j in 0..rock_height{
                if rocks[k%nb_shapes][rock_height-1-j][i]{
                    cave[y+j][x+i] = true;
                }
            }
        }
        if k < 10 {
            for j in 0..cave.len(){
                for i in 0..cave_width{
                    print!("{}",if(cave[cave.len()-1-j][i]){'#'}else{'.'});
                }
                print!("\n");
            }
        }
    }
    println!("{}", height);
}

pub fn part2() {
    let mut wind_left_to_right: Vec<bool> = [].to_vec();
    const cave_width:usize = 7;
    const nb_shapes: usize = 5;
    let mut cave: Vec<[bool;cave_width]> = [[true,true,true,true,true,true,true]].to_vec();
    for _ in 0..7 {
        cave.push([false,false,false,false,false,false,false]);
    }
    let mut rocks: [Vec<Vec<bool>>;nb_shapes] = [
        [
            [true,true,true,true].to_vec()
        ].to_vec(),
        [
            [false,true,false].to_vec(),
            [true,true,true].to_vec(),
            [false,true,false].to_vec()
        ].to_vec(),
        [
            [false,false,true].to_vec(),
            [false,false,true].to_vec(),
            [true,true,true].to_vec()
        ].to_vec(),
        [
            [true].to_vec(),
            [true].to_vec(),
            [true].to_vec(),
            [true].to_vec()
        ].to_vec(),
        [
            [true,true].to_vec(),
            [true,true].to_vec()
        ].to_vec()
    ];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                for c in line_content.chars(){
                    wind_left_to_right.push(c == '>');
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut height = 0;
    let mut wind_count: usize = 0;
    let wind_size = wind_left_to_right.len();
    let mut pattern_found = false;
    let mut cave_patterns: Vec<[i32;9]> = [].to_vec();
    let mut pattern_start: usize = 0;
    let mut pattern_period: usize = 0;
    let mut pattern_height: i64 = 0;
    let mut k: usize = 0;
    cave_patterns.push([-1,0,0,0,0,0,0,0,0]);
    while !pattern_found{
        let rock_width = rocks[k%nb_shapes][0].len();
        let rock_height = rocks[k%nb_shapes].len();
        let mut x: usize = 2;
        let mut y: usize = height + 4;
        let mut grounded = false;
        // create enough room
        while cave.len() < height + 8{
            cave.push([false,false,false,false,false,false,false]); 
        }
        // compute the current "pattern"
        let mut new_pattern: [i32;cave_width+2] = [0,0,0,0,0,0,0,height as i32,k as i32];
        let mut y_ref = 0;
        while !cave[cave.len()-1-y_ref][0]{
            y_ref += 1;
        }
        for i in 1..cave_width {
            let mut y_i = 0;
            while !cave[cave.len()-1-y_i][i]{
                y_i += 1;
            }
            new_pattern[i] = (y_i as i32) - (y_ref as i32);
        }
        cave_patterns.remove(cave_patterns.len()-1);
        cave_patterns.push(new_pattern);
        // check similarities with previous one
        for i in 0..(wind_count/wind_size){
            if (i*wind_size+(wind_count%wind_size) != wind_count) && (cave_patterns[i*wind_size+(wind_count%wind_size)][cave_width+1]%(nb_shapes as i32) == cave_patterns[wind_count][cave_width+1]%(nb_shapes as i32)){
                pattern_found = true;
                for j in 0..cave_width{
                    if cave_patterns[i*wind_size+(wind_count%wind_size)][j] != cave_patterns[wind_count][j]{
                        pattern_found = false;
                    }
                }
                if pattern_found{
                    println!("pattern found !!! from {} to {}",i*wind_size+(wind_count%wind_size),wind_count);
                    println!("{}",cave_patterns.len());
                    for j in 0..cave_width+2{
                        print!("{},",cave_patterns[i*wind_size+(wind_count%wind_size)][j]);
                    }
                    print!("\n");
                    for j in 0..cave_width+2{
                        print!("{},",cave_patterns[wind_count][j]);
                    }
                    print!("\n");
                    pattern_start = cave_patterns[i*wind_size+(wind_count%wind_size)][cave_width+1] as usize;
                    pattern_period = (cave_patterns[wind_count][cave_width+1] - cave_patterns[i*wind_size+(wind_count%wind_size)][cave_width+1]) as usize;
                    pattern_height = (cave_patterns[wind_count][cave_width] - cave_patterns[i*wind_size+(wind_count%wind_size)][cave_width]) as i64;
                    break;
                }
            }
            if pattern_found{
                break;
            }
        }
        if pattern_found{
            break;
        }
        while !grounded{
            let wind_dir: i32 = if(wind_left_to_right[wind_count%wind_size]){1}else{-1};
            wind_count = wind_count + 1;
            // to make it easier to count
            let mut new_pattern: [i32;9] = [-1,0,0,0,0,0,0,0,0];
            cave_patterns.push(new_pattern);
            // wind action
            let mut can_be_pushed = false;
            if (x > 0) && (wind_dir < 0){
                can_be_pushed = true;
                for j in 0..rock_height{
                    for i in 0..rock_width{
                        if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j][x+i-1]{
                            can_be_pushed = false;
                            break;
                        }
                    }
                }
            }
            if (x + rock_width < cave_width) && (wind_dir > 0){
                can_be_pushed = true;
                for j in 0..rock_height{
                    for i in 0..rock_width{
                        if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j][x+i+1]{
                            can_be_pushed = false;
                            break;
                        }
                    }
                }
            }
            if can_be_pushed{
                x = ((x as i32) + wind_dir) as usize;
            }
            //gravity action
            for i in 0..rock_width{
                for j in 0..rock_height{
                    if rocks[k%nb_shapes][rock_height-1-j][i] && cave[y+j-1][x+i]{
                        grounded = true;
                        break;
                    }
                }
                if grounded { break; }
            }
            if !grounded {
                y -= 1;
            }
        }
        //update height
        if y + rock_height - 1 > height{
            height = y + rock_height - 1;
        }
        // add blocks
        for i in 0..rock_width{
            for j in 0..rock_height{
                if rocks[k%nb_shapes][rock_height-1-j][i]{
                    cave[y+j][x+i] = true;
                }
            }
        }
        k+=1;
    }
    let goal: i64 = 1000000000000;
    let pattern_count = (goal-(pattern_start as i64))/(pattern_period as i64);
    println!("pattern_count : {}", pattern_count);
    println!("pattern_height : {}", pattern_height);
    let nb_of_missing_block = (goal-(pattern_count*(pattern_period as i64))) as i32;
    println!("nb_of_missing_block : {}", nb_of_missing_block);
    let mut i=0;
    while cave_patterns[i][cave_width+1] != nb_of_missing_block{
        i+=1;
    }
    let mut final_height = (cave_patterns[i][cave_width] as i64) ;
    println!("height of repeated pattern : {}", pattern_height*pattern_count);
    println!("basic height : {}", final_height);
    final_height += pattern_height*pattern_count;
    println!("final height : {}", final_height);
}