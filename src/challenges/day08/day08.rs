#[path = "../../utils/mod.rs"] mod utils;

fn score(forest:&Vec<Vec<i32>>, i:usize, j:usize, n:usize) -> i32 {
    if (i == 0) || (i == n-1) || (j == 0) || (j == n-1) {
        return 0;
    }
    let mut total_score = 1;
    let mut max_height = -1;
    let mut local_score = 0;
    let h = forest[i][j];
    for k in 0..i {
        local_score += 1;
        if forest[i-1-k][j] > max_height {
            max_height = forest[i-1-k][j];
        }
        if max_height >= h {
            break;
        }
    }
    total_score *= local_score;
    local_score = 0;
    max_height = -1;
    for k in (i+1)..n {
        local_score += 1;
        if forest[k][j] > max_height {
            max_height = forest[k][j];
        }
        if max_height >= h {
            break;
        }
    }
    total_score *= local_score;
    local_score = 0;
    max_height = -1;
    for k in 0..j {
        local_score += 1;
        if forest[i][j-1-k] > max_height {
            max_height = forest[i][j-1-k];
        }
        if max_height >= h {
            break;
        }
    }
    total_score *= local_score;
    local_score = 0;
    max_height = -1;
    for k in (j+1)..n {
        local_score += 1;
        if forest[i][k] > max_height {
            max_height = forest[i][k];
        }
        if max_height >= h {
            break;
        }
    }
    total_score *= local_score;
    return total_score;
}

pub fn part1() {
    let mut forest:Vec<Vec<i32>> = [].to_vec();
    let mut tree_visibility:Vec<Vec<bool>> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut line_of_trees:Vec<i32> = [].to_vec();
                let mut line_of_tree_visibilities:Vec<bool> = [].to_vec();
                let mut max_height = -1;
                for (i, c) in line_content.chars().enumerate() {
                    let h:i32 = (c as i32) - ('0' as i32);
                    line_of_trees.push(h);
                    line_of_tree_visibilities.push(false);
                    if h > max_height {
                        line_of_tree_visibilities[i] = true;
                        max_height = h;
                    }

                }
                max_height = -1;
                let n = line_of_trees.len();
                for i in 0..n {
                    let h = line_of_trees[n-1-i];
                    if h > max_height {
                        line_of_tree_visibilities[n-1-i] = true;
                        max_height = h;
                    }

                }
                forest.push(line_of_trees);
                tree_visibility.push(line_of_tree_visibilities);
                
            }
        }
    }else{
        println!("lines not ok");
    }
    let n = forest.len();
    for j in 0..n {
        let mut max_height = -1;
        for i in 0..n {
            let h = forest[i][j];
            if h > max_height {
                tree_visibility[i][j] = true;
                max_height = h;
            }
        }
        max_height = -1;
        for i in 0..n {
            let h = forest[n-1-i][j];
            if h > max_height {
                tree_visibility[n-1-i][j] = true;
                max_height = h;
            }
        }
    }
    let mut visible_trees_nb = 0;
    for i in 0..n {
        for j in 0..n {
            if tree_visibility[i][j]{
                visible_trees_nb += 1;
            }
        }
    }
    println!("{}",visible_trees_nb);

}

pub fn part2() {
    let mut forest:Vec<Vec<i32>> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut line_of_trees:Vec<i32> = [].to_vec();
                for (i, c) in line_content.chars().enumerate() {
                    let h:i32 = (c as i32) - ('0' as i32);
                    line_of_trees.push(h);
                }
                forest.push(line_of_trees);
            }
        }
    }else{
        println!("lines not ok");
    }
    let n = forest.len();
    let mut best_score = 0;
    for i in 0..n {
        for j in 0..n {
            let mut local_score = score(&forest, i, j, n);
            //print!("{local_score:?}");
            if local_score > best_score {
                best_score = local_score;
            }
        }
        //print!("\n");
    }
    
    println!("{}",best_score);
}
