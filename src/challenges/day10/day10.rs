#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut signal_value = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                if "noop" == String::from(split[0]){
                    cycle_count += 1;
                    if (cycle_count - 20)%40 == 0 {
                        signal_value += cycle_count * register_value;
                    }
                    //println!("noop");
                }else{
                    //println!("addx");
                    for i in 0..2 {
                        cycle_count += 1;
                        if (cycle_count - 20)%40 == 0 {
                            signal_value += cycle_count * register_value;
                        }
                    }
                    let increment_value:i32 = split[1].parse().unwrap();
                    register_value += increment_value;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{signal_value:?}");
    
}

pub fn part2() {
    let mut register_value = 1;
    let mut cycle_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                if "noop" == String::from(split[0]){
                    cycle_count += 1;
                    if (((cycle_count+39)%40)+1 <= register_value+2) && (cycle_count%40 >= register_value){
                        print!("#");
                    }else{
                        print!("_");
                    }
                    if cycle_count%40 == 0{
                        print!("\n");
                    }
                }else{
                    for i in 0..2 {
                        cycle_count += 1;
                        if (((cycle_count+39)%40)+1 <= register_value+2) && (cycle_count%40 >= register_value){
                            print!("#");
                        }else{
                            print!("_");
                        }
                        if (cycle_count%40) == 0{
                            print!("\n");
                        }
                    }
                    let increment_value:i32 = split[1].parse().unwrap();
                    register_value += increment_value;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    
}
