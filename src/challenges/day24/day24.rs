#[path = "../../utils/mod.rs"] mod utils;

pub fn gcd(a: usize, b:usize) -> usize {
    let mut q = a;
    let mut r = b;
    while r != 0 {
        let tmp = r;
        r = q%r;
        q = tmp;
    }
    return q;
}

pub fn lcm(a: usize, b:usize) -> usize {
    return a*b/gcd(a,b);
}

pub fn diff(a: usize, b: usize) -> usize{
    if a > b{
        return a-b;
    }
    return b-a;
}

pub fn part1() {
    let mut periodic_maps: Vec<Vec<Vec<Vec<char>>>> = vec![vec![]];
    let mut n_i = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                periodic_maps[0].push(vec![]);
                n_i += 1;
                for c in line_content.chars(){
                    if c == '.'{
                        periodic_maps[0][n_i-1].push(vec![]);
                    }else{
                        periodic_maps[0][n_i-1].push(vec![c]);
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_j = periodic_maps[0][0].len();
    let n = lcm(n_i-2,n_j-2);
    for k in 1..n{
        periodic_maps.push(vec![vec![]]);
        periodic_maps[k][0].push(vec!['#']);
        periodic_maps[k][0].push(vec![]);
        for _ in 0..n_j-2{
            periodic_maps[k][0].push(vec!['#']);
        }
        for i in 1..n_i-1{
            periodic_maps[k].push(vec![]);
            periodic_maps[k][i].push(vec!['#']);
            for j in 1..n_j-1{
                periodic_maps[k][i].push(vec![]);
                let prev_i = (n_i-3 + i-1)%(n_i-2) + 1;
                let prev_j = j;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == 'v'{
                        periodic_maps[k][i][j].push('v');
                    }
                }
                let prev_i = (n_i-3 + i+1)%(n_i-2) + 1;
                let prev_j = j;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '^'{
                        periodic_maps[k][i][j].push('^');
                    }
                }
                let prev_i = i;
                let prev_j = (n_j-3 + j-1)%(n_j-2) + 1;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '>'{
                        periodic_maps[k][i][j].push('>');
                    }
                }
                let prev_i = i;
                let prev_j = (n_j-3 + j+1)%(n_j-2) + 1;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '<'{
                        periodic_maps[k][i][j].push('<');
                    }
                }
            }
            periodic_maps[k][i].push(vec!['#']);
        }
        periodic_maps[k].push(vec![]);
        for _ in 0..n_j-2{
            periodic_maps[k][n_i-1].push(vec!['#']);
        }
        periodic_maps[k][n_i-1].push(vec![]);
        periodic_maps[k][n_i-1].push(vec!['#']);
    }
    // implement A*
    let mut closed: Vec<Vec<Vec<Option<[usize;3]>>>> = vec![];
    for k in 0..n{
        closed.push(vec![]);
        for i in 0..n_i{
            closed[k].push(vec![]);
            for _ in 0..n_j{
                closed[k][i].push(None);
            }
        }
    }
    let mut open: Vec<[usize;5]> = vec![[0,0,1,0,1]];
    let mut path_len: Option<usize> = None;
    while !path_len.is_some(){
        let [k,i,j,prev_i,prev_j] = open[0];
        open.remove(0);
        let next_moves: Vec<[usize;2]>;
        if (i == 0) && (j == 1){
            next_moves = vec![[i+1,j],[i,j]];
        }else{
            next_moves = vec![[i+1,j],[i-1,j],[i,j-1],[i,j+1],[i,j]];
        }
        for [next_i,next_j] in next_moves{
            if (next_i == n_i - 1) && (next_j == n_j - 2){
                path_len = Some(k+1);
                break;
            } else if (periodic_maps[(k+1)%n][next_i][next_j].len() == 0)
            && (!closed[(k+1)%n][next_i][next_j].is_some() || closed[(k+1)%n][next_i][next_j].unwrap()[0] > k+1){
                closed[(k+1)%n][next_i][next_j] = None;
                //manathan distance to the goal
                let h = k + 1 + (n_i - 1) - next_i + (n_j - 2) - next_j;
                let mut l = 0;
                let mut already_open = false;
                while l < open.len(){
                    let h_l = open[l][0] + (n_i - 1) - open[l][1] + (n_j - 2) - open[l][2];
                    if (h > h_l) || ((h == h_l) && (k+1 <= open[l][0])){
                        already_open = (open[l][0]%n == (k+1)%n) && (open[l][1] == next_i) && (open[l][2] == next_j);
                        if already_open{
                            break;
                        }
                        l += 1;
                    }else{
                        break;
                    }
                }
                if ! already_open{
                    open.insert(l,[k+1,next_i,next_j,i,j]);
                    l += 1;
                    while l < open.len(){
                        if (open[l][0]%n == (k+1)%n) && (open[l][1] == next_i) && (open[l][2] == next_j){
                            open.remove(l);
                            continue;
                        }
                        l += 1;
                    }
                }

            }
        }
        closed[k%n][i][j] = Some([k,prev_i,prev_j]);
    }
    println!("{}",path_len.unwrap());
}


pub fn part2() {
    let mut periodic_maps: Vec<Vec<Vec<Vec<char>>>> = vec![vec![]];
    let mut n_i = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                periodic_maps[0].push(vec![]);
                n_i += 1;
                for c in line_content.chars(){
                    if c == '.'{
                        periodic_maps[0][n_i-1].push(vec![]);
                    }else{
                        periodic_maps[0][n_i-1].push(vec![c]);
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    let n_j = periodic_maps[0][0].len();
    let n = lcm(n_i-2,n_j-2);
    for k in 1..n{
        periodic_maps.push(vec![vec![]]);
        periodic_maps[k][0].push(vec!['#']);
        periodic_maps[k][0].push(vec![]);
        for _ in 0..n_j-2{
            periodic_maps[k][0].push(vec!['#']);
        }
        for i in 1..n_i-1{
            periodic_maps[k].push(vec![]);
            periodic_maps[k][i].push(vec!['#']);
            for j in 1..n_j-1{
                periodic_maps[k][i].push(vec![]);
                let prev_i = (n_i-3 + i-1)%(n_i-2) + 1;
                let prev_j = j;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == 'v'{
                        periodic_maps[k][i][j].push('v');
                    }
                }
                let prev_i = (n_i-3 + i+1)%(n_i-2) + 1;
                let prev_j = j;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '^'{
                        periodic_maps[k][i][j].push('^');
                    }
                }
                let prev_i = i;
                let prev_j = (n_j-3 + j-1)%(n_j-2) + 1;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '>'{
                        periodic_maps[k][i][j].push('>');
                    }
                }
                let prev_i = i;
                let prev_j = (n_j-3 + j+1)%(n_j-2) + 1;
                let elem_count = periodic_maps[k-1][prev_i][prev_j].len();
                for l in 0..elem_count{
                    if periodic_maps[k-1][prev_i][prev_j][l] == '<'{
                        periodic_maps[k][i][j].push('<');
                    }
                }
            }
            periodic_maps[k][i].push(vec!['#']);
        }
        periodic_maps[k].push(vec![]);
        for _ in 0..n_j-2{
            periodic_maps[k][n_i-1].push(vec!['#']);
        }
        periodic_maps[k][n_i-1].push(vec![]);
        periodic_maps[k][n_i-1].push(vec!['#']);
    }
    let mut chronos: Vec<usize> = vec![0];
    let open_start = [[0,0,1,0,1],[0,n_i-1, n_j-2,n_i-1, n_j-2],[0,0,1,0,1]];
    let goals = [[n_i-1, n_j-2],[0, 1],[n_i-1, n_j-2]];
    for attempt in 0..3{
        // implement A* 3 times
        let mut closed: Vec<Vec<Vec<Option<[usize;3]>>>> = vec![];
        for k in 0..n{
            closed.push(vec![]);
            for i in 0..n_i{
                closed[k].push(vec![]);
                for _ in 0..n_j{
                    closed[k][i].push(None);
                }
            }
        }
        let mut open: Vec<[usize;5]> = vec![open_start[attempt]];
        open[0][0] = chronos[chronos.len()-1];
        let goal = goals[attempt];
        let mut path_len: Option<usize> = None;
        while !path_len.is_some(){
            let [k,i,j,prev_i,prev_j] = open[0];
            open.remove(0);
            let next_moves: Vec<[usize;2]>;
            if (i == 0) && (j == 1){
                next_moves = vec![[i+1,j],[i,j]];
            }else if (i == n_i - 1) && (j == n_j - 2){
                next_moves = vec![[i-1,j],[i,j]];
            }else{
                next_moves = vec![[i,j+1],[i+1,j],[i-1,j],[i,j-1],[i,j]];
            }
            for [next_i,next_j] in next_moves{
                if (next_i == goal[0]) && (next_j == goal[1]){
                    path_len = Some(k+1);
                    break;
                } else if ((next_i>0) || next_j==1) && (next_j>0) && ((next_i<n_i-1) || next_j==n_j-2) && (next_j<n_j-1)
                && (periodic_maps[(k+1)%n][next_i][next_j].len() == 0)
                && (!closed[(k+1)%n][next_i][next_j].is_some() || closed[(k+1)%n][next_i][next_j].unwrap()[0] > k+1){
                    closed[(k+1)%n][next_i][next_j] = None;
                    //manathan distance to the goal
                    let h = k + 1 + diff(goal[0], next_i) + diff(goal[1], next_j) ;
                    let mut l = 0;
                    let mut already_open = false;
                    while l < open.len(){
                        let h_l = open[l][0] + diff(goal[0], open[l][1]) + diff(goal[1], open[l][2]);
                        if (h > h_l) || ((h == h_l) && (k+1 <= open[l][0])){
                            already_open = (open[l][0]%n == (k+1)%n) && (open[l][1] == next_i) && (open[l][2] == next_j);
                            if already_open{
                                break;
                            }
                            l += 1;
                        }else{
                            break;
                        }
                    }
                    if ! already_open{
                        open.insert(l,[k+1,next_i,next_j,i,j]);
                        l += 1;
                        while l < open.len(){
                            if (open[l][0]%n == (k+1)%n) && (open[l][1] == next_i) && (open[l][2] == next_j){
                                open.remove(l);
                                continue;
                            }
                            l += 1;
                        }
                    }

                }
            }
            closed[k%n][i][j] = Some([k,prev_i,prev_j]);
        }
        let chrono = path_len.unwrap();
        println!("chrono for run {} : {}",attempt+1,chrono-chronos[chronos.len()-1]);
        chronos.push(chrono);
    }
    println!("{}",chronos[chronos.len()-1]);
}
