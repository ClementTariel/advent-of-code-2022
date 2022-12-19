use std::default;

#[path = "../../utils/mod.rs"] mod utils;

fn int_to_index(n:i32) -> usize {
    if n < 0 {
        return (-(2*n +1)) as usize;
    }else{
        return 2*n as usize;
    }
}

fn index_to_int(n:usize) -> i32 {
    if n%2 == 0 {
        return (n as i32)/2;
    }else{
        return -((n+1) as i32)/2;
    }
}

pub fn part1() {
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false]]];
    let mut faces_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(",").collect::<Vec<&str>>();
                let x: i32 = split[0].parse().unwrap();
                let y: i32 = split[1].parse().unwrap();
                let z: i32 = split[2].parse().unwrap();
                let i: usize = int_to_index(x);
                let j: usize = int_to_index(y);
                let k: usize = int_to_index(z);
                //update grid size on x axis
                while !(i < grid.len()){
                    let mut slice: Vec<Vec<bool>> = vec![];
                    for _ in 0..grid[0].len(){
                        let mut line: Vec<bool> = vec![];
                        for _ in 0..grid[0][0].len(){
                            line.push(false);
                        }
                        slice.push(line);
                    }
                    grid.push(slice);
                }
                //update grid size on y axis
                for x_i in 0..grid.len(){
                    while !(j < grid[x_i].len()){
                        let mut line: Vec<bool> = vec![];
                        for _ in 0..grid[0][0].len(){
                            line.push(false);
                        }
                        grid[x_i].push(line);
                    }
                }
                //update grid size on z axis
                for x_i in 0..grid.len(){
                    for y_j in 0..grid[x_i].len(){
                        while !(k < grid[x_i][y_j].len()){
                            grid[x_i][y_j].push(false);
                        }
                    }
                }
                // update grid value at pos x,y,z
                grid[i][j][k] = true;
                faces_count += 6;
                //top
                let next_i = i;
                let next_j = j;
                let next_k = int_to_index(z+1);
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                //bottom
                let next_i = i;
                let next_j = j;
                let next_k = int_to_index(z-1);
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                //right
                let next_i = int_to_index(x+1);
                let next_j = j;
                let next_k = k;
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                //left
                let next_i = int_to_index(x-1);
                let next_j = j;
                let next_k = k;
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                //front
                let next_i = i;
                let next_j = int_to_index(y-1);
                let next_k = k;
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                //back
                let next_i = i;
                let next_j = int_to_index(y+1);
                let next_k = k;
                if (next_i < grid.len()) 
                && (next_j < grid[0].len()) 
                && (next_k < grid[0][0].len())
                && grid[next_i][next_j][next_k]{
                    faces_count -= 2;
                }
                
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{}",faces_count);
}

pub fn part2() {
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false]]];
    let mut faces_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(",").collect::<Vec<&str>>();
                let x: i32 = split[0].parse().unwrap();
                let y: i32 = split[1].parse().unwrap();
                let z: i32 = split[2].parse().unwrap();
                let i: usize = int_to_index(x);
                let j: usize = int_to_index(y);
                let k: usize = int_to_index(z);
                //update grid size on x axis
                while !(i < grid.len()){
                    let mut slice: Vec<Vec<bool>> = vec![];
                    for _ in 0..grid[0].len(){
                        let mut line: Vec<bool> = vec![];
                        for _ in 0..grid[0][0].len(){
                            line.push(false);
                        }
                        slice.push(line);
                    }
                    grid.push(slice);
                }
                //update grid size on y axis
                for x_i in 0..grid.len(){
                    while !(j < grid[x_i].len()){
                        let mut line: Vec<bool> = vec![];
                        for _ in 0..grid[0][0].len(){
                            line.push(false);
                        }
                        grid[x_i].push(line);
                    }
                }
                //update grid size on z axis
                for x_i in 0..grid.len(){
                    for y_j in 0..grid[x_i].len(){
                        while !(k < grid[x_i][y_j].len()){
                            grid[x_i][y_j].push(false);
                        }
                    }
                }
                // update grid value at pos x,y,z
                grid[i][j][k] = true;
            }
        }
    }else{
        println!("lines not ok");
    }
    //expand grid size on x axis by 2 unit on both sides
    for _ in 0..4{
        let mut slice: Vec<Vec<bool>> = vec![];
        for _ in 0..grid[0].len(){
            let mut line: Vec<bool> = vec![];
            for _ in 0..grid[0][0].len(){
                line.push(false);
            }
            slice.push(line);
        }
        grid.push(slice);
    }
    //expand grid size on y axis by 2 unit on both sides
    for x_i in 0..grid.len(){
        for _ in 0..4{
            let mut line: Vec<bool> = vec![];
            for _ in 0..grid[0][0].len(){
                line.push(false);
            }
            grid[x_i].push(line);
        }
    }
    //expand grid size on z axis by 2 unit on both sides
    for x_i in 0..grid.len(){
        for y_j in 0..grid[x_i].len(){
            for _ in 0..4{
                grid[x_i][y_j].push(false);
            }
        }
    }
    let n_i = grid.len();
    let n_j = grid[0].len();
    let n_k = grid[0][0].len();
    let mut next_air: Vec<[i32;3]> = vec![];
    let mut air_seen: Vec<Vec<Vec<bool>>> = vec![];
    for _ in 0..n_i{
        let mut slice: Vec<Vec<bool>> = vec![];
        for _ in 0..n_j{
            let mut line: Vec<bool> = vec![];
            for _ in 0..n_k{
                line.push(false);
            }
            slice.push(line);
        }
        air_seen.push(slice);
    }
    // top and bottom of the grid
    for i in 0..n_i {
        for j in 0..n_j {
            let x = index_to_int(i);
            let y = index_to_int(j);
            let z = index_to_int(n_k-2);
            next_air.push([x,y,z]);
            let z = index_to_int(n_k-1);
            next_air.push([x,y,z]);
        }
    }
    // right and left of the grid
    for k in 0..n_k {
        for j in 0..n_j {
            let x = index_to_int(n_i-2);
            let y = index_to_int(j);
            let z = index_to_int(k);
            next_air.push([x,y,z]);
            let x = index_to_int(n_i-1);
            next_air.push([x,y,z]);
        }
    }
    // front and back of the grid
    for k in 0..n_k {
        for i in 0..n_i {
            let x = index_to_int(i);
            let y = index_to_int(n_j-2);
            let z = index_to_int(k);
            next_air.push([x,y,z]);
            let y = index_to_int(n_j-1);
            next_air.push([x,y,z]);
        }
    }
    let mut depth_count = next_air.len();
    while depth_count > 0{
        let x = next_air[0][0];
        let y = next_air[0][1];
        let z = next_air[0][2];
        next_air.remove(0);
        depth_count -= 1;
        let i = int_to_index(x);
        let j = int_to_index(y);
        let k = int_to_index(z);
        if air_seen[i][j][k]{
            if !(depth_count > 0){
                depth_count = next_air.len();
            }
            continue;
        }
        air_seen[i][j][k] = true;
        // check adjacent cubes
        //top
        let next_i = i;
        let next_j = j;
        let next_k = int_to_index(z+1);
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        //bottom
        let next_i = i;
        let next_j = j;
        let next_k = int_to_index(z-1);
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        //right
        let next_i = int_to_index(x+1);
        let next_j = j;
        let next_k = k;
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        //left
        let next_i = int_to_index(x-1);
        let next_j = j;
        let next_k = k;
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        //front
        let next_i = i;
        let next_j = int_to_index(y-1);
        let next_k = k;
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        //back
        let next_i = i;
        let next_j = int_to_index(y+1);
        let next_k = k;
        if (next_i < n_i) && (next_j < n_j) && (next_k < n_k){
            if grid[next_i][next_j][next_k]{
                faces_count += 1;
            }else if !air_seen[next_i][next_j][next_k]{
                next_air.push([index_to_int(next_i),index_to_int(next_j),index_to_int(next_k)]);
            }
        }
        if !(depth_count > 0){
            depth_count = next_air.len();
        }
    }

    println!("{}",faces_count);
}