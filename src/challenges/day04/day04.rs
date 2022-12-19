#[path = "../../utils/mod.rs"] mod utils;


pub fn part1() {
    let mut total = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(",").collect::<Vec<&str>>();
                let space1 = split[0].split("-").collect::<Vec<&str>>();
                let space2 = split[1].split("-").collect::<Vec<&str>>();
                let l1: i32 = space1[0].parse().unwrap();
                let r1: i32  = space1[1].parse().unwrap();
                let l2: i32  = space2[0].parse().unwrap();
                let r2: i32  = space2[1].parse().unwrap();
                if (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2){
                    total += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{total:?}");
}

pub fn part2() {
    let mut total = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(",").collect::<Vec<&str>>();
                let space1 = split[0].split("-").collect::<Vec<&str>>();
                let space2 = split[1].split("-").collect::<Vec<&str>>();
                let l1: i32 = space1[0].parse().unwrap();
                let r1: i32  = space1[1].parse().unwrap();
                let l2: i32  = space2[0].parse().unwrap();
                let r2: i32  = space2[1].parse().unwrap();
                if (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2)
                || (l1 <= l2 && r1 >= l2) || (l1 <= r2 && r1 >= r2){
                    total += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{total:?}");
}
