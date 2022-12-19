#[path = "../../utils/mod.rs"] mod utils;


pub fn part1() {
    let mut building_crates: bool = true;
    let mut crates:Vec<Vec<char>> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == "" {
                    building_crates = false;
                }else if building_crates {
                    let mut letter:char = ' ';
                    for (i, c) in line_content.chars().enumerate() {
                        if i%4 == 1 {
                            letter = c;
                            let pos = i/4;
                            while ! (pos < crates.len()) {
                                let mut empty_vec = [].to_vec();
                                crates.push(empty_vec);
                            }
                            if letter != ' '{
                                crates[pos].insert(0, letter);
                            }
                            
                        }
                    }
                }else{
                    let split = line_content.split(" ").collect::<Vec<&str>>();
                    let n:i32 = split[1].parse().unwrap();
                    let mut origin:i32 = split[3].parse().unwrap();
                    let mut destination:i32 = split[5].parse().unwrap();
                    origin -= 1;
                    destination -= 1;
                    for i in 0..n{
                        while ! ((origin as usize) < crates.len() && (destination as usize) < crates.len()) {
                            println!("potential error");
                            let mut empty_vec = [].to_vec();
                            crates.push(empty_vec);
                        }
                        let letter = crates[origin as usize].pop().unwrap();
                        crates[destination as usize].push(letter);
                    }
                }
                
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut res: String = String::from("");
    for v in crates {
        let letter = v[&v.len()-1];
        res += &(letter.to_string());
    }
    println!("{}",res);

}

pub fn part2() {
    let mut building_crates: bool = true;
    let mut crates:Vec<Vec<char>> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == "" {
                    building_crates = false;
                }else if building_crates {
                    let mut letter:char = ' ';
                    for (i, c) in line_content.chars().enumerate() {
                        if i%4 == 1 {
                            letter = c;
                            let pos = i/4;
                            while ! (pos < crates.len()) {
                                let mut empty_vec = [].to_vec();
                                crates.push(empty_vec);
                            }
                            if letter != ' '{
                                crates[pos].insert(0, letter);
                            }
                            
                        }
                    }
                }else{
                    let split = line_content.split(" ").collect::<Vec<&str>>();
                    let n:i32 = split[1].parse().unwrap();
                    let mut origin:i32 = split[3].parse().unwrap();
                    let mut destination:i32 = split[5].parse().unwrap();
                    origin -= 1;
                    destination -= 1;
                    let insert_pos = crates[destination as usize].len();
                    for i in 0..n{
                        while ! ((origin as usize) < crates.len() && (destination as usize) < crates.len()) {
                            println!("potential error");
                            let mut empty_vec = [].to_vec();
                            crates.push(empty_vec);
                        }
                        let letter = crates[origin as usize].pop().unwrap();
                        crates[destination as usize].insert(insert_pos,letter);
                    }
                }
                
            }
        }
    }else{
        println!("lines not ok");
    }
    let mut res: String = String::from("");
    for v in crates {
        let letter = v[&v.len()-1];
        res += &(letter.to_string());
    }
    println!("{}",res);
}
