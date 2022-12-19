#[path = "../../utils/mod.rs"] mod utils;


fn update_next_position_and_distance(next_position: &mut Vec<[i32;2]>, distance: &mut Vec<Vec<i32>>, next_i: usize, i:usize, next_j:usize, j:usize){
    let d = distance[i][j];
    if distance[next_i][next_j] == -1 {
        let mut k:usize = 0;
        while (k < next_position.len()) && (distance[next_position[k][0] as usize][next_position[k][1] as usize] < d+1) {
            k +=1;
        }
        next_position.insert(k, [next_i as i32,next_j as i32]);
        distance[next_i][next_j] = d+1;
    }else if distance[next_i][next_j] > d+1 {
        let mut k:usize = 0;
        while k < next_position.len() {
            if (next_position[k][0] == (i as i32)) && (next_position[k][1] == (j as i32)) {
                next_position.remove(k);
                k = next_position.len();
            }
            k +=1;
        }
        k = 0;
        while (k < next_position.len()) && (distance[next_position[k][0] as usize][next_position[k][1] as usize] < d+1) {
            k +=1;
        }
        next_position.insert(k, [i as i32,j as i32]);
        distance[next_i][next_j] = d+1;
    }
}

pub fn part1() {
    let mut n = 0;
    let mut height_map: Vec<Vec<i32>> = [].to_vec();
    let mut distance: Vec<Vec<i32>> = [].to_vec();
    let mut next_position: Vec<[i32;2]> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                height_map.push([].to_vec());
                distance.push([].to_vec());
                n += 1;
                for (i, c) in line_content.chars().enumerate() {
                    distance[n-1].push(-1);
                    if c == 'E' {
                        height_map[n-1].push(26);
                    }else if c == 'S' {
                        height_map[n-1].push(-1);
                        next_position.push([(n as i32) - 1 ,i as i32]);
                        distance[n-1][i] = 0;
                    }else{
                        height_map[n-1].push((c as i32) - ('a' as i32));
                    }
                    
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_i = height_map.len();
    let n_j = height_map[0].len();
    let mut path_found: bool = false;
    while !path_found && next_position.len() > 0 {
        let i = next_position[0][0] as usize;
        let j = next_position[0][1] as usize;

        for i2 in 0..n_i {
            for j2 in 0.. n_j {
                if (i2 == i) && (j2 == j){
                    print!("X");
                }else{
                    print!("_",);
                }
            }
            print!("\n");
        }

        let h = height_map[i][j];
        //println!("{h:?}");
        next_position.remove(0);
        let next_i: i32 = (i as i32) - 1;
        let next_j: i32 = j as i32;
        if (next_i >= 0) && (h+1 >= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 26 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = (i as i32) + 1;
        let next_j: i32 = j as i32;
        if ((next_i as usize) < height_map.len()) && (h+1 >= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 26 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = i as i32;
        let next_j: i32 = (j as i32) - 1;
        if (next_j >= 0) && (h+1 >= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 26 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = i as i32;
        let next_j: i32 = (j as i32) + 1;
        if ((next_j as usize) < height_map[i].len()) && (h+1 >= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 26 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
    }
    
    
}

pub fn part2() {
    let mut n = 0;
    let mut height_map: Vec<Vec<i32>> = [].to_vec();
    let mut distance: Vec<Vec<i32>> = [].to_vec();
    let mut next_position: Vec<[i32;2]> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                height_map.push([].to_vec());
                distance.push([].to_vec());
                n += 1;
                for (i, c) in line_content.chars().enumerate() {
                    distance[n-1].push(-1);
                    if c == 'E' {
                        height_map[n-1].push(26);
                        next_position.push([(n as i32) - 1 ,i as i32]);
                        distance[n-1][i] = 0;
                    }else if c == 'S' {
                        height_map[n-1].push(0);
                    }else{
                        height_map[n-1].push((c as i32) - ('a' as i32));
                    }
                    
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_i = height_map.len();
    let n_j = height_map[0].len();
    let mut path_found: bool = false;
    while !path_found && next_position.len() > 0 {
        let i = next_position[0][0] as usize;
        let j = next_position[0][1] as usize;

        for i2 in 0..n_i {
            for j2 in 0.. n_j {
                if (i2 == i) && (j2 == j){
                    print!("X");
                }else{
                    print!("_",);
                }
            }
            print!("\n");
        }

        let h = height_map[i][j];
        //println!("{h:?}");
        next_position.remove(0);
        let next_i: i32 = (i as i32) - 1;
        let next_j: i32 = j as i32;
        if (next_i >= 0) && (h-1 <= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 0 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = (i as i32) + 1;
        let next_j: i32 = j as i32;
        if ((next_i as usize) < height_map.len()) && (h-1 <= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 0 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = i as i32;
        let next_j: i32 = (j as i32) - 1;
        if (next_j >= 0) && (h-1 <= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 0 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
        let next_i: i32 = i as i32;
        let next_j: i32 = (j as i32) + 1;
        if ((next_j as usize) < height_map[i].len()) && (h-1 <= height_map[next_i as usize][next_j as usize]){
            let next_i = next_i as usize;
            let next_j = next_j as usize;
            update_next_position_and_distance(&mut next_position, &mut distance, next_i, i, next_j, j);
            if height_map[next_i][next_j] == 0 {
                println!("{}",distance[next_i][next_j]);
                path_found = true;
            }
        }
    }
}
