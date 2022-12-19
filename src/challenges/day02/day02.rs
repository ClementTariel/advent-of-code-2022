#[path = "../../utils/mod.rs"] mod utils;

fn score(a:char,x:char) -> i32 {
    //score for the shape
    let mut xscore = 1 + (x as u32 - 'X' as u32) as i32;
    //score for the resutl of the match
    xscore += 3*((4 + (x as u32 - 'X' as u32) as i32 - (a as u32 - 'A' as u32) as i32 )%3);
    return xscore;
}

fn score2(a:char,x:char) -> i32 {
    //score for the shape
    let mut xscore = 1 + (2 + (a as u32 - 'A' as u32) as i32 + (x as u32 - 'X' as u32) as i32)%3;
    //score for the resutl of the match
    xscore += 3*(x as u32 - 'X' as u32) as i32 ;
    return xscore;
}

pub fn part1() {
    let mut total_score = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content != "" {
                    let a: char = line_content.chars().nth(0).unwrap();
                    let x: char = line_content.chars().nth(2).unwrap();
                    total_score += &score(a, x)
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{total_score:?}");
}

pub fn part2() {
    let mut total_score = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content != "" {
                    let a: char = line_content.chars().nth(0).unwrap();
                    let x: char = line_content.chars().nth(2).unwrap();
                    total_score += &score2(a, x)
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{total_score:?}");
}
