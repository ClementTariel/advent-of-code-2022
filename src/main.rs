mod utils;
#[path = "challenges/day17/day17.rs"] mod day17;




fn main() {

    // let current_day = utils::get_last_day();
    // println!("day{current_day}");
    // let file_path = utils::day_to_file_path(current_day);
    // println!("{}", file_path);
    // include!(file_path);
    println!("solving day17");
    println!("part1");
    day17::part1();
    println!("part2");
    day17::part2();
    
    
}


