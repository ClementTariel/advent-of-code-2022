#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut max_sum = 0;
    let mut current_sum = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content == "" {
                    if current_sum > max_sum {
                        max_sum = current_sum;
                    }
                    current_sum = 0;
                }else{
                    let line_value: i32 = line_content.parse().unwrap();
                    current_sum += line_value;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{max_sum:?}");
}

pub fn part2() {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut sum_3 = 0;
    let mut current_sum = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                if line_content == "" {
                    if current_sum > sum_3 {
                        sum_3 = current_sum;
                    }
                    if current_sum > sum_2 {
                        sum_3 = sum_2;
                        sum_2 = current_sum;
                    }
                    if current_sum > sum_1 {
                        sum_2 = sum_1;
                        sum_1 = current_sum;
                    }
                    current_sum = 0;
                }else{
                    let line_value: i32 = line_content.parse().unwrap();
                    current_sum += line_value;
                }
            }
        }
    }
    let total_sum = sum_1+sum_2+sum_3;
    println!("{total_sum:?}");
}
