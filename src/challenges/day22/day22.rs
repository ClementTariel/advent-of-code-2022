#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut map: Vec<Vec<char>> = vec![];
    let mut drawing_map = true;
    let mut n_i = 0;
    let mut n_j = 0;
    let mut x = 0;
    let mut y = 0;
    let mut dir=0;
    let mut steps=0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == ""{
                    drawing_map = false;
                    continue;
                }
                if drawing_map{
                    map.push(vec![]);
                    n_i += 1;
                    for c in line_content.chars() {
                        map[n_i-1].push(c);
                        if (x==0) && (map[x][y]==' '){
                            y += 1;
                        }
                    }
                    if map[n_i-1].len() > n_j {
                        for i in 0..(n_i-1){
                            for _ in n_j..map[n_i-1].len(){
                                map[i].push(' ');
                            }
                        }
                    }
                    while map[n_i-1].len() < n_j{
                        map[n_i-1].push(' ');
                    }
                    n_j = map[n_i-1].len();

                }else{
                    let dirs = [[0,1],[1,0],[0,n_j-1],[n_i-1,0]];
                    for c in line_content.chars() {
                        let make_a_turn = (c == 'L') || (c == 'R');
                        if make_a_turn{
                            for _ in 0..steps{
                                x = (x + dirs[dir][0])%n_i;
                                y = (y + dirs[dir][1])%n_j;
                                while map[x][y] == ' '{
                                    x = (x + dirs[dir][0])%n_i;
                                    y = (y + dirs[dir][1])%n_j;
                                }
                                if map[x][y] == '#'{
                                    x = (x + n_i - dirs[dir][0])%n_i;
                                    y = (y + n_j - dirs[dir][1])%n_j;
                                    while map[x][y] == ' '{
                                        x = (x + n_i - dirs[dir][0])%n_i;
                                        y = (y + n_j - dirs[dir][1])%n_j;
                                    }
                                    break;
                                }
                            }
                            if c == 'L'{
                                dir = (dir + 3)%4;
                            }
                            if c == 'R' {
                                dir = (dir + 1)%4;
                            }
                            steps = 0;
                        }else{
                            steps = 10*steps + (c as i32) - ('0' as i32);
                        }
                    }
                    
                }

            }
        }
    }else{
        println!("lines not ok");
    }
    let dirs = [[0,1],[1,0],[0,n_j-1],[n_i-1,0]];
    for _ in 0..steps{
        x = (x + dirs[dir][0])%n_i;
        y = (y + dirs[dir][1])%n_j;
        while map[x][y] == ' '{
            x = (x + dirs[dir][0])%n_i;
            y = (y + dirs[dir][1])%n_j;
        }
        if map[x][y] == '#'{
            x = (x + n_i - dirs[dir][0])%n_i;
            y = (y + n_j - dirs[dir][1])%n_j;
            while map[x][y] == ' '{
                x = (x + n_i - dirs[dir][0])%n_i;
                y = (y + n_j - dirs[dir][1])%n_j;
            }
            break;
        }
    }
    let row = x+1;
    let column = y+1;
    println!("row:{}, column:{},dir:{}",row,column,dir);
    let code = 1000*row + 4*column + dir;
    println!("code:{}",code);
}

pub fn find_n(k6: i32) -> usize{
    let k = (k6 as usize)/6;
    let mut delta = 1;
    let mut n: usize = 1;
    while n*n < k{
        n += delta;
        delta *= 2;
    }
    while !((n*n == k) || (delta<= 0)) {
        if n*n < k{
            n += delta;
        }else{
            n -= delta;
        }
        delta /= 2;
    }
    return n;
}

pub fn compute_dir_side(dir_ortho:usize, dir_up:usize) -> usize {
    return (3 - dir_ortho - dir_up)%3;
}

pub fn compute_sense_side(dir_ortho:usize, dir_up:usize, sense_ortho:i32, sense_up:i32) -> i32 {
    return (((4 + dir_up - dir_ortho)%3) as i32 - 1)*sense_ortho*sense_up;
}

pub struct CubeFace {
    dir_ortho: usize,
    sense_ortho: i32,
    dir_up: usize,
    sense_up: i32,
    dir_side: usize,
    sense_side: i32
}

pub struct CubeMap {
    simple_map: Vec<Vec<usize>>,
    faces: [Vec<CubeFace>;6]
}

pub fn vector_to_face_number(dir_ortho:usize, sense_ortho:i32) -> usize{
    return (((7+(1 + dir_ortho as i32)*sense_ortho))%7) as usize;
}

pub fn face_after_rotation(neihb_offset:[i32;2],face:&CubeFace) -> CubeFace{
    if neihb_offset == [1,0]{
        //go up
        let dir_ortho = face.dir_up;
        let dir_up = face.dir_ortho;
        let sense_ortho = face.sense_up;
        let sense_up = -face.sense_ortho;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        let face = CubeFace{dir_ortho,sense_ortho,dir_up,sense_up,dir_side,sense_side};
        return face;
    }else if neihb_offset == [-1,0]{
        //go down
        //rotate, move, rotate
        let dir_ortho = face.dir_up;
        let dir_up = face.dir_ortho;
        let sense_ortho = -face.sense_up;
        let sense_up = face.sense_ortho;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        let face = CubeFace{dir_ortho,sense_ortho,dir_up,sense_up,dir_side,sense_side};
        return face;
    }else if neihb_offset == [0,-1]{
        //go right
        //rotate, move
        let dir_ortho = face.dir_side;
        let dir_up = face.dir_ortho;
        let sense_ortho = face.sense_side;
        let sense_up = -face.sense_ortho;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        //rotate again
        let dir_up = dir_side;
        let sense_up = -sense_side;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        let face = CubeFace{dir_ortho,sense_ortho,dir_up,sense_up,dir_side,sense_side};
        return face;
    }else /*if neihb_offset == [0,1]*/{
        // go left
        //rotate, move
        let dir_ortho = face.dir_side;
        let dir_up = face.dir_ortho;
        let sense_ortho = -face.sense_side;
        let sense_up = -face.sense_ortho;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        //rotate again
        let dir_up = dir_side;
        let sense_up = sense_side;
        let dir_side = compute_dir_side(dir_ortho, dir_up);
        let sense_side = compute_sense_side(dir_ortho, dir_up, sense_ortho, sense_up);
        let face = CubeFace{dir_ortho,sense_ortho,dir_up,sense_up,dir_side,sense_side};
        return face;
    }
}

pub fn map_with_one_square_per_cube_face(map: &Vec<Vec<char>>, n:usize) -> CubeMap {
    let mut faces: [Vec<CubeFace>;6] = [vec![],vec![],vec![],vec![],vec![],vec![]];
    let mut simple_map: Vec<Vec<usize>> = vec![];
    let mut nb_neighb_faces: Vec<Vec<usize>> = vec![];
    let mut set_faces: Vec<Vec<bool>> = vec![];
    let n_i = map.len();
    let n_j = map[0].len();
    for i in 0..(n_i/n){
        simple_map.push(vec![]);
        nb_neighb_faces.push(vec![]);
        set_faces.push(vec![]);
        for j in 0..(n_j/n){
            if map[n*i+n/2][n*j+n/2] == ' '{
                simple_map[i].push(0);
                nb_neighb_faces[i].push(0);
                set_faces[i].push(true);
            }else{
                simple_map[i].push(7);
                nb_neighb_faces[i].push(0);
                set_faces[i].push(false);
            }
        }
    }

    //set one face
    let dir_ortho = 0;
    let sense_ortho = 1;
    let dir_up = 1;
    let sense_up = 1;
    let dir_side = 2;
    let sense_side = 1;
    let face = CubeFace{dir_ortho,sense_ortho,dir_up,sense_up,dir_side,sense_side};
    let face_num = vector_to_face_number(dir_ortho,sense_ortho);
    faces[face_num - 1].push(face);
    let mut nb_faces_set = 0;
    for i in 0..(n_i/n){
        for j in 0..(n_j/n){
            if (nb_faces_set == 0) && (simple_map[i][j] == 7) {
                simple_map[i][j] = face_num;
                set_faces[i][j] = true;
                nb_faces_set = 1;
            }
            if (i > 1) && (simple_map[i-1][j] > 0) {
                nb_neighb_faces[i][j] += 1;
            }
            if (j > 1) && (simple_map[i][j-1] > 0) {
                nb_neighb_faces[i][j] += 1;
            }
            if (i+1 < simple_map.len()) && (simple_map[i+1][j] > 0) {
                nb_neighb_faces[i][j] += 1;
            }
            if (j+1 < simple_map[0].len()) && (simple_map[i][j+1] > 0) {
                nb_neighb_faces[i][j] += 1;
            }
        }
    }
    // others faces
    while nb_faces_set < 6{
        for i in 0..(n_i/n){
            for j in 0..(n_j/n){
                if !set_faces[i][j]{
                    let mut has_neihb_set = false;
                    let mut neihb_offset: [i32;2] = [-2,-2];
                    if (i > 0) && set_faces[i-1][j] && (simple_map[i-1][j] > 0) {
                        has_neihb_set = true;
                        neihb_offset = [-1,0];
                    }
                    if (j > 0) && set_faces[i][j-1] && (simple_map[i][j-1] > 0){
                        has_neihb_set = true;
                        neihb_offset = [0,-1];
                    }
                    if (i+1 < simple_map.len()) && set_faces[i+1][j] && (simple_map[i+1][j] > 0){
                        has_neihb_set = true;
                        neihb_offset = [1,0];
                    }
                    if (j+1 < simple_map[0].len()) && set_faces[i][j+1] && (simple_map[i][j+1] > 0){
                        has_neihb_set = true;
                        neihb_offset = [0,1];
                    }
                    if has_neihb_set{
                        let neihb_i = (i as i32 + neihb_offset[0]) as usize;
                        let neihb_j = (j as i32 + neihb_offset[1]) as usize;
                        if neihb_offset == [1,0]{
                            //go up
                            let face = face_after_rotation(neihb_offset,&faces[simple_map[neihb_i][neihb_j] - 1][0]);
                            simple_map[i][j] = vector_to_face_number(face.dir_ortho, face.sense_ortho);
                            set_faces[i][j] = true;
                            nb_faces_set += 1;
                            faces[simple_map[i][j] - 1].push(face);
                        }
                        if neihb_offset == [-1,0]{
                            //go down
                            //rotate, move, rotate
                            let face = face_after_rotation(neihb_offset,&faces[simple_map[neihb_i][neihb_j] - 1][0]);
                            simple_map[i][j] = vector_to_face_number(face.dir_ortho, face.sense_ortho);
                            set_faces[i][j] = true;
                            nb_faces_set += 1;
                            faces[simple_map[i][j] - 1].push(face);
                        }
                        if neihb_offset == [0,-1]{
                            //go right
                            let face = face_after_rotation(neihb_offset,&faces[simple_map[neihb_i][neihb_j] - 1][0]);
                            simple_map[i][j] = vector_to_face_number(face.dir_ortho, face.sense_ortho);
                            set_faces[i][j] = true;
                            nb_faces_set += 1;
                            faces[simple_map[i][j] - 1].push(face);
                        }
                        if neihb_offset == [0,1]{
                            // go left
                            let face = face_after_rotation(neihb_offset,&faces[simple_map[neihb_i][neihb_j] - 1][0]);
                            simple_map[i][j] = vector_to_face_number(face.dir_ortho, face.sense_ortho);
                            set_faces[i][j] = true;
                            nb_faces_set += 1;
                            faces[simple_map[i][j] - 1].push(face);
                        }
                        
                    }
                }
            }
        }
    }

    return CubeMap{simple_map,faces};
}

pub fn travel_on_cube(i:usize, j:usize, step:&[usize;2], map:&Vec<Vec<char>>, n:usize,cube_map:&CubeMap) -> [usize;3]{
    let n_i = map.len();
    let n_j = map[0].len();
    let mut next_i = (i + step[0])%n_i;
    let mut next_j = (j + step[1])%n_j;
    if (i/n == next_i/n) && (j/n == next_j/n){
        return [next_i,next_j,0];
    }
    let current_face_num = cube_map.simple_map[i/n][j/n];
    let mut neihb_offset: [i32;2] = [((step[0]+1)%n_i) as i32 - 1,((step[1]+1)%n_j) as i32 - 1];
    // *-1 because we move out of the face
    neihb_offset[0] *= -1;
    neihb_offset[1] *= -1;
    let face = face_after_rotation(neihb_offset,&cube_map.faces[current_face_num - 1][0]);
    let next_face_num = vector_to_face_number(face.dir_ortho,face.sense_ortho);
    let mut r = 0;
    if face.dir_up != cube_map.faces[next_face_num - 1][0].dir_up {
        r = 1;
        if face.sense_up != cube_map.faces[next_face_num - 1][0].sense_side{
            r += 2;
        }
    }else if face.sense_up != cube_map.faces[next_face_num - 1][0].sense_up {
        r = 2;
    }
    let mut base_i = 0;
    let mut base_j = 0;
    let mut delta_i = 0;
    let mut delta_j = 0;
    for simple_i in 0..(cube_map.simple_map.len()){
        for simple_j in 0..(cube_map.simple_map[0].len()){
            if cube_map.simple_map[simple_i][simple_j] == next_face_num {
                base_i = simple_i*n;
                delta_i = next_i-n*(next_i/n);
                base_j = simple_j*n;
                delta_j = next_j-n*(next_j/n);
            }
        }
    }
    next_i = base_i + delta_i;
    next_j = base_j + delta_j;
    if r%2 == 1{
        //to do check
        if r>= 2{
            next_j = base_j + delta_i ;
            next_i = base_i +n-1 - delta_j;
        }else{
            next_j = base_j + n-1 - delta_i ;
            next_i = base_i + delta_j;
        }
    }else if r >= 2{
        next_i = next_i + n-1 - 2*delta_i;
        next_j = next_j + n-1 - 2*delta_j;
    }
    return [next_i,next_j,r];
}

pub fn part2() {
    let mut map: Vec<Vec<char>> = vec![];
    let mut drawing_map = true;
    let mut n_i = 0;
    let mut n_j = 0;
    let mut x = 0;
    let mut y = 0;
    let mut dir=0;
    let mut steps=0;
    let mut nb_char=0;
    let mut cube_map:CubeMap = CubeMap{simple_map:vec![],faces:[vec![],vec![],vec![],vec![],vec![],vec![]]};
    let mut n:usize = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == ""{
                    drawing_map = false;
                    n = find_n(nb_char);
                    cube_map = map_with_one_square_per_cube_face(&map,n);
                    continue;
                }
                if drawing_map{
                    map.push(vec![]);
                    n_i += 1;
                    for c in line_content.chars() {
                        map[n_i-1].push(c);
                        if (x==0) && (map[x][y]==' '){
                            y += 1;
                        }
                        if c != ' '{
                            nb_char += 1;
                        }
                    }
                    if map[n_i-1].len() > n_j {
                        for i in 0..(n_i-1){
                            for _ in n_j..map[n_i-1].len(){
                                map[i].push(' ');
                            }
                        }
                    }
                    while map[n_i-1].len() < n_j{
                        map[n_i-1].push(' ');
                    }
                    n_j = map[n_i-1].len();

                }else{
                    let dirs = [[0,1],[1,0],[0,n_j-1],[n_i-1,0]];
                    for c in line_content.chars() {
                        let make_a_turn = (c == 'L') || (c == 'R');
                        if make_a_turn{
                            for _ in 0..steps{
                                let [next_i,next_j,r] = travel_on_cube(x, y, &dirs[dir], &map, n, &cube_map);
                                if map[next_i][next_j] == '.'{
                                    x = next_i;
                                    y = next_j;
                                    dir = (dir + r)%4;
                                }else{
                                    break;
                                }
                            }
                            if c == 'L'{
                                dir = (dir + 3)%4;
                            }
                            if c == 'R' {
                                dir = (dir + 1)%4;
                            }
                            steps = 0;
                        }else{
                            steps = 10*steps + (c as i32) - ('0' as i32);
                        }
                    }
                    
                }

            }
        }
    }else{
        println!("lines not ok");
    }
    let dirs = [[0,1],[1,0],[0,n_j-1],[n_i-1,0]];
    for _ in 0..steps{
        let [next_i,next_j,r] = travel_on_cube(x, y, &dirs[dir], &map, n, &cube_map);
        if map[next_i][next_j] == '.'{
            x = next_i;
            y = next_j;
            dir = (dir + r)%4;
        }else{
            break;
        }
    }
    print!("\n");
    let row = x+1;
    let column = y+1;
    println!("row:{}, column:{},dir:{}",row,column,dir);
    let code = 1000*row + 4*column + dir;
    println!("code:{}",code);
}
