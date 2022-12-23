#[path = "../../utils/mod.rs"] mod utils;

pub fn resize_to_prevent_overflow(elves: &mut Vec<Vec<bool>>){
    let mut n_i = elves.len();
    let mut n_j = elves[0].len();
    //prevent right overflow
    let mut add_line = false;
    for i in 0..n_i{
        if elves[i][n_j-1]{
            add_line = true;
            break;
        }
    }
    if add_line{
        for i in 0..n_i{
            elves[i].push(false);
        }
        n_j += 1;
    }
    //prevent bottom overflow
    let mut add_line = false;
    for j in 0..n_j{
        if elves[n_i-1][j]{
            add_line = true;
            break;
        }
    }
    if add_line{
        let mut new_line: Vec<bool> = vec![];
        for _ in 0..n_j{
            new_line.push(false);
        }
        elves.push(new_line);
        n_i += 1;
    }
    //prevent top overflow
    let mut add_line = false;
    for j in 0..n_j{
        if elves[0][j]{
            add_line = true;
            break;
        }
    }
    if add_line{
        let mut new_line: Vec<bool> = vec![];
        for _ in 0..n_j{
            new_line.push(false);
        }
        elves.insert(0, new_line);
        n_i += 1;
    }
    //prevent left overflow
    let mut add_line = false;
    for i in 0..n_i{
        if elves[i][0]{
            add_line = true;
            break;
        }
    }
    if add_line{
        for i in 0..n_i{
            elves[i].insert(0, false);
        }
        //n_j += 1;
    }
    
}

pub fn possible_move(i: usize, j: usize, elves: &Vec<Vec<bool>>, dirs: &[[i32;2];4]) -> Vec<[usize;2]>{
    let mut next_moves: Vec<[usize;2]> = vec![];
    for k in 0..(dirs.len()){
        let mut move_blocked = false;
        let mut ortho_dir: [i32;2] = [0,1];
        if dirs[k][0] == 0{
            ortho_dir = [1,0];
        }
        for l in 0..3{
            if elves[((i as i32)+dirs[k][0] + (l-1)*ortho_dir[0]) as usize][((j as i32)+dirs[k][1] + (l-1)*ortho_dir[1]) as usize]{
                move_blocked = true;
                break;
            }
        }
        if !move_blocked{
            let move_i = ((i as i32)+dirs[k][0]) as usize;
            let move_j = ((j as i32)+dirs[k][1]) as usize;
            next_moves.push([move_i,move_j]);
        }
    }
    return next_moves;
}

pub fn move_elves(elves: &mut Vec<Vec<bool>>, dirs: &mut [[i32;2];4]) -> i32{
    resize_to_prevent_overflow(elves);
    let mut elves_ready = 0;
    let n_i = elves.len();
    let n_j = elves[0].len();
    let mut moves_origins: Vec<Vec<Vec<[usize;2]>>> = vec![];
    for _ in 0..n_i{
        let mut new_line: Vec<Vec<[usize;2]>> = vec![];
        for _ in 0..n_j{
            new_line.push(vec![]);
        }
        moves_origins.push(new_line);
    }
    for i in 0..n_i{
        for j in 0..n_j{
            if elves[i][j]{
                let next_moves: Vec<[usize;2]> = possible_move(i, j, &elves, &dirs);
                if next_moves.len() == 4{
                    elves_ready += 1;
                    continue;
                }
                if next_moves.len() > 0 {
                    moves_origins[next_moves[0][0]][next_moves[0][1]].push([i,j]);
                }
            }
        }
    }
    for i in 0..n_i{
        for j in 0..n_j{
            let nb_applicants = moves_origins[i][j].len();
            if nb_applicants == 1{
                elves[i][j] = true;
                let prev_i = moves_origins[i][j][0][0];
                let prev_j = moves_origins[i][j][0][1];
                elves[prev_i][prev_j] = false;
            }
        }
    }
    let first_dir = dirs[0];
    dirs[0] = dirs[1];
    dirs[1] = dirs[2];
    dirs[2] = dirs[3];
    dirs[3] = first_dir;
    return elves_ready;
}

pub fn part1() {
    let mut elves: Vec<Vec<bool>> = vec![];
    let mut n_i = 0;
    let mut n_j = 0;
    let n_rounds = 10;
    let mut elves_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                elves.push(vec![]);
                n_i += 1;
                for c in line_content.chars() {
                    elves[n_i-1].push(c == '#');
                    if c == '#' {
                        elves_count += 1;
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut dirs = [[-1,0],[1,0],[0,-1],[0,1]];

    for _ in 1..=n_rounds{
        move_elves(&mut elves, &mut dirs);
        n_i = elves.len();
        n_j = elves[0].len();
    }
    let mut empty_line_count = 0;
    let mut empty_line_found;
    for i in 0..n_i{
        empty_line_found = true;
        for j in 0..n_j{
            if elves[i][j]{
                empty_line_found = false;
                break;
            }
        }
        if empty_line_found{
            empty_line_count += 1;
        }else{
            break;
        }
    }
    for i in 0..n_i{
        empty_line_found = true;
        for j in 0..n_j{
            if elves[n_i-1-i][j]{
                empty_line_found = false;
                break;
            }
        }
        if empty_line_found{
            empty_line_count += 1;
        }else{
            break;
        }
    }
    let mut empty_column_count = 0;
    let mut empty_column_found;
    for j in 0..n_j{
        empty_column_found = true;
        for i in 0..n_i{
            if elves[i][j]{
                empty_column_found = false;
                break;
            }
        }
        if empty_column_found{
            empty_column_count += 1;
        }else{
            break;
        }
    }
    for j in 0..n_j{
        empty_column_found = true;
        for i in 0..n_i{
            if elves[i][n_j-1-j]{
                empty_column_found = false;
                break;
            }
        }
        if empty_column_found{
            empty_column_count += 1;
        }else{
            break;
        }
    }
    for i in 0..n_i{
        for j in 0..n_j{
            print!("{}",if elves[i][j] {'#'}else{'.'});
        }
        print!("\n");
    }
    println!("----");
    let area = (n_i-empty_line_count)*(n_j-empty_column_count);
    println!("{}", area - elves_count);
}


pub fn part2() {
    let mut elves: Vec<Vec<bool>> = vec![];
    let mut n_i = 0;
    let mut n_j = 0;
    let mut rounds_count = 1;
    let mut elves_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                elves.push(vec![]);
                n_i += 1;
                for c in line_content.chars() {
                    elves[n_i-1].push(c == '#');
                    if c == '#' {
                        elves_count += 1;
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut dirs = [[-1,0],[1,0],[0,-1],[0,1]];

    while move_elves(&mut elves, &mut dirs) != elves_count{
        n_i = elves.len();
        n_j = elves[0].len();
        rounds_count += 1;
    }
    for i in 0..n_i{
        for j in 0..n_j{
            print!("{}",if elves[i][j] {'#'}else{'.'});
        }
        print!("\n");
    }
    println!("----");
    println!("{}", rounds_count);
}
