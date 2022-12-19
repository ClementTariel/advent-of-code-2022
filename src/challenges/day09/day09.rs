#[path = "../../utils/mod.rs"] mod utils;

fn letter_to_move(c:char) -> [i32;2]{
    if c == 'R' {
        return [1,0];
    }else if c == 'L' {
        return [-1,0];
    }else if c == 'U' {
        return [0,1];
    }else if c == 'D' {
        return [0,-1];
    }
    println!("error when parsing, could not recognize char {c:?}");
    return [0,0];
}

fn abs(n:i32) -> i32{
    if n < 0 {
        return -n;
    }
    return n;
}

fn int_to_index(n:i32) -> usize {
    if n < 0 {
        return (-(2*n +1)) as usize;
    }else{
        return 2*n as usize;
    }
}

pub fn part1() {
    let mut visited:Vec<Vec<bool>> = [[false].to_vec()].to_vec();
    let mut total_sum = 0;
    let mut h_pos = [0,0];
    let mut t_pos = [0,0];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let letter:char = split[0].parse().unwrap();
                let n:i32 = split[1].parse().unwrap();
                let current_move = letter_to_move(letter);
                let mut n_i = visited.len();
                let mut n_j = visited[0].len();
                if n_j <= int_to_index(h_pos[1]+(n*current_move[1])){
                    for i in 0..n_i{
                        for j in 0..=(int_to_index(h_pos[1]+(n*current_move[1])) - n_j){
                            visited[i].push(false);
                        }
                    }
                }
                n_j = visited[0].len();
                if n_i <= int_to_index(h_pos[0]+(n*current_move[0])){
                    for i in n_i..=int_to_index(h_pos[0]+(n*current_move[0])){
                        visited.push([].to_vec());
                        for j in 0..n_j{
                            visited[i].push(false);
                        }
                    }
                }
                n_i = visited.len();
                for k in 0..n {
                    h_pos[0] += current_move[0];
                    h_pos[1] += current_move[1];
                    if (abs(h_pos[0] - t_pos[0]) > 1) || (abs(h_pos[1] - t_pos[1]) > 1){
                        if h_pos[0] > t_pos[0] {
                            t_pos[0] += 1;
                        }else if h_pos[0] < t_pos[0] {
                            t_pos[0] -= 1;
                        }
                        if h_pos[1] > t_pos[1] {
                            t_pos[1] += 1;
                        }else if h_pos[1] < t_pos[1] {
                            t_pos[1] -= 1;
                        }
                    }
                    visited[int_to_index(t_pos[0])][int_to_index(t_pos[1])] = true;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_i = visited.len();
    let n_j = visited[0].len();
    for i in 0..n_i{
        for j in 0..n_j{
            if visited[i][j]{
                total_sum += 1;
            }
        }
    }
    println!("{}",total_sum);

}


pub fn part2() {
    let mut visited:Vec<Vec<bool>> = [[false].to_vec()].to_vec();
    let mut total_sum = 0;
    let mut knots_pos:[[i32;2];10] = [[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0],[0,0]];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let letter:char = split[0].parse().unwrap();
                let n:i32 = split[1].parse().unwrap();
                let current_move = letter_to_move(letter);
                let mut n_i = visited.len();
                let mut n_j = visited[0].len();
                if n_j <= int_to_index(knots_pos[0][1]+(n*current_move[1])){
                    for i in 0..n_i{
                        for j in 0..=(int_to_index(knots_pos[0][1]+(n*current_move[1])) - n_j){
                            visited[i].push(false);
                        }
                    }
                }
                n_j = visited[0].len();
                if n_i <= int_to_index(knots_pos[0][0]+(n*current_move[0])){
                    for i in n_i..=int_to_index(knots_pos[0][0]+(n*current_move[0])){
                        visited.push([].to_vec());
                        for j in 0..n_j{
                            visited[i].push(false);
                        }
                    }
                }
                n_i = visited.len();
                for k in 0..n {
                    knots_pos[0][0] += current_move[0];
                    knots_pos[0][1] += current_move[1];
                    for l in 0..9{
                        if (abs(knots_pos[l][0] - knots_pos[l+1][0]) > 1) || (abs(knots_pos[l][1] - knots_pos[l+1][1]) > 1){
                            if knots_pos[l][0] > knots_pos[l+1][0] {
                                knots_pos[l+1][0] += 1;
                            }else if knots_pos[l][0] < knots_pos[l+1][0] {
                                knots_pos[l+1][0] -= 1;
                            }
                            if knots_pos[l][1] > knots_pos[l+1][1] {
                                knots_pos[l+1][1] += 1;
                            }else if knots_pos[l][1] < knots_pos[l+1][1] {
                                knots_pos[l+1][1] -= 1;
                            }
                        }
                    }
                    visited[int_to_index(knots_pos[9][0])][int_to_index(knots_pos[9][1])] = true;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_i = visited.len();
    let n_j = visited[0].len();
    for i in 0..n_i{
        for j in 0..n_j{
            if visited[i][j]{
                total_sum += 1;
            }
        }
    }
    println!("{}",total_sum);
}
