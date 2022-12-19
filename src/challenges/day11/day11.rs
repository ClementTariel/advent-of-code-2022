#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut item_from_monkey: Vec<Vec<i32>> = [[].to_vec()].to_vec();
    let mut activity_from_monkey: Vec<i32> = [].to_vec();
    let mut operation_from_monkey: Vec<Box<dyn Fn(i32) -> i32>> = vec![];
    let mut criteria_from_monkey: Vec<i32> = [].to_vec();
    let mut true_next_monkey_from_monkey: Vec<i32> = [].to_vec();
    let mut false_next_monkey_from_monkey: Vec<i32> = [].to_vec();
    let mut current_monkey = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content != "" {
                    let split = line_content.split(" ").collect::<Vec<&str>>();
                    if split[0] == "Monkey" {
                        println!("monkey {}, {current_monkey:?}",split[1]);
                        item_from_monkey.push([].to_vec());
                        activity_from_monkey.push(0);
                        let split2 = split[1].split(":").collect::<Vec<&str>>();
                        current_monkey = split2[0].parse().unwrap();
                    }else if split[2] == "Starting"{
                        //println!("starting");
                        let split2 = line_content.split(": ").collect::<Vec<&str>>();
                        let split3 = split2[1].split(", ").collect::<Vec<&str>>();
                        let n = split3.len();
                        for  i in 0..n{
                            let worry_level:i32 = split3[i].parse().unwrap();
                            item_from_monkey[current_monkey].push(worry_level);
                        }
                    }else if split[2] == "Operation:"{
                        //println!("operation");
                        let mut monkey_operation:Box<dyn Fn(i32, i32) -> i32> = Box::new(|a:i32, b:i32| -> i32{0});
                        let mut monkey_worry_increase:Box<dyn Fn(i32) -> i32>;
                        if split[6] == "+" {
                            //println!("monkey {} + '{}'",current_monkey, split[6]);
                            monkey_operation = Box::new(|a:i32, b:i32| -> i32{a + b});
                        }else if split[6] == "*"{
                            //println!("monkey {} * '{}'",current_monkey, split[6]);
                            monkey_operation = Box::new(|a:i32, b:i32| -> i32{a * b});
                        }else{
                            println!("operation incompatible '{}'",split[6]);
                        }
                        if split[7] == "old" {
                            monkey_worry_increase = Box::new(move |x:i32| -> i32{
                                monkey_operation(x,x)
                            });
                        }else{
                            let n:i32 = split[7].parse().unwrap();
                            monkey_worry_increase = Box::new(move |x:i32| -> i32{
                                monkey_operation(x,n)
                            });
                        }
                        operation_from_monkey.push(monkey_worry_increase);
                    }else if split[2] == "Test:"{
                        //println!("test");
                        let n:i32 = split[5].parse().unwrap();
                        criteria_from_monkey.push(n);
                    }else if split[5] == "true:"{
                        //println!("true");
                        let n:i32 = split[9].parse().unwrap();
                        true_next_monkey_from_monkey.push(n);
                    }else if split[5] == "false:"{
                        //println!("false");
                        let n:i32 = split[9].parse().unwrap();
                        false_next_monkey_from_monkey.push(n);
                    }
                    
                }else{
                    current_monkey += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    for j in 0..20{
        for i in 0..=current_monkey {
            while item_from_monkey[i].len() > 0{
                activity_from_monkey[i] += 1;
                let mut item = item_from_monkey[i][0];
                item_from_monkey[i].remove(0);
                item = operation_from_monkey[i](item)/3;
                let next_monkey:usize;
                if (item%criteria_from_monkey[i]) == 0 {
                    next_monkey = true_next_monkey_from_monkey[i] as usize;
                }else{
                    next_monkey = false_next_monkey_from_monkey[i] as usize;
                }
                item_from_monkey[next_monkey].push(item);
            }
            
        }
    }
    for i in 0..=current_monkey {
        println!("{}",activity_from_monkey[i]);
    }
    let mut first_activity:i32=0;
    let mut second_activity:i32=0;
    for i in 0..=current_monkey {
        if activity_from_monkey[i] > second_activity {
            second_activity = activity_from_monkey[i];
        }
        if activity_from_monkey[i] > first_activity {
            second_activity = first_activity;
            first_activity = activity_from_monkey[i];
        }
    }
    println!("{}",first_activity);
    println!("{}",second_activity);
    println!("{}",first_activity*second_activity);
    
}

pub fn part2() {
    let mut item_from_monkey: Vec<Vec<i64>> = [[].to_vec()].to_vec();
    let mut activity_from_monkey: Vec<i64> = [].to_vec();
    let mut operation_from_monkey: Vec<Box<dyn Fn(i64) -> i64>> = vec![];
    let mut criteria_from_monkey: Vec<i64> = [].to_vec();
    let mut true_next_monkey_from_monkey: Vec<i64> = [].to_vec();
    let mut false_next_monkey_from_monkey: Vec<i64> = [].to_vec();
    let mut current_monkey = 0;
    let mut big_modulo = 1;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if line_content != "" {
                    let split = line_content.split(" ").collect::<Vec<&str>>();
                    if split[0] == "Monkey" {
                        println!("monkey {}, {current_monkey:?}",split[1]);
                        item_from_monkey.push([].to_vec());
                        activity_from_monkey.push(0);
                        let split2 = split[1].split(":").collect::<Vec<&str>>();
                        current_monkey = split2[0].parse().unwrap();
                    }else if split[2] == "Starting"{
                        //println!("starting");
                        let split2 = line_content.split(": ").collect::<Vec<&str>>();
                        let split3 = split2[1].split(", ").collect::<Vec<&str>>();
                        let n = split3.len();
                        for  i in 0..n{
                            let worry_level:i64 = split3[i].parse().unwrap();
                            item_from_monkey[current_monkey].push(worry_level);
                        }
                    }else if split[2] == "Operation:"{
                        //println!("operation");
                        let mut monkey_operation:Box<dyn Fn(i64, i64) -> i64> = Box::new(|a:i64, b:i64| -> i64{0});
                        let mut monkey_worry_increase:Box<dyn Fn(i64) -> i64>;
                        if split[6] == "+" {
                            //println!("monkey {} + '{}'",current_monkey, split[6]);
                            monkey_operation = Box::new(|a:i64, b:i64| -> i64{a + b});
                        }else if split[6] == "*"{
                            //println!("monkey {} * '{}'",current_monkey, split[6]);
                            monkey_operation = Box::new(|a:i64, b:i64| -> i64{a * b});
                        }else{
                            println!("operation incompatible '{}'",split[6]);
                        }
                        if split[7] == "old" {
                            monkey_worry_increase = Box::new(move |x:i64| -> i64{
                                monkey_operation(x,x)
                            });
                        }else{
                            let n:i64 = split[7].parse().unwrap();
                            monkey_worry_increase = Box::new(move |x:i64| -> i64{
                                monkey_operation(x,n)
                            });
                        }
                        operation_from_monkey.push(monkey_worry_increase);
                    }else if split[2] == "Test:"{
                        //println!("test");
                        let n:i64 = split[5].parse().unwrap();
                        criteria_from_monkey.push(n);
                        big_modulo *= n;
                    }else if split[5] == "true:"{
                        //println!("true");
                        let n:i64 = split[9].parse().unwrap();
                        true_next_monkey_from_monkey.push(n);
                    }else if split[5] == "false:"{
                        //println!("false");
                        let n:i64 = split[9].parse().unwrap();
                        false_next_monkey_from_monkey.push(n);
                    }
                    
                }else{
                    current_monkey += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    for j in 0..10000{
        for i in 0..=current_monkey {
            while item_from_monkey[i].len() > 0{
                activity_from_monkey[i] += 1;
                let mut item = item_from_monkey[i][0];
                item_from_monkey[i].remove(0);
                item = operation_from_monkey[i](item);
                item = item%big_modulo;
                let next_monkey:usize;
                if (item%criteria_from_monkey[i]) == 0 {
                    next_monkey = true_next_monkey_from_monkey[i] as usize;
                }else{
                    next_monkey = false_next_monkey_from_monkey[i] as usize;
                }
                item_from_monkey[next_monkey].push(item);
            }
            
        }
        if (j==0) || (j == 19) || (j==20){
            println!("j = {}",j);
            for i in 0..=current_monkey {
                println!("{}",activity_from_monkey[i]);
            }
        }
    }
    for i in 0..=current_monkey {
        println!("{}",activity_from_monkey[i]);
    }
    let mut first_activity:i64=0;
    let mut second_activity:i64=0;
    for i in 0..=current_monkey {
        if activity_from_monkey[i] > second_activity {
            second_activity = activity_from_monkey[i];
        }
        if activity_from_monkey[i] > first_activity {
            second_activity = first_activity;
            first_activity = activity_from_monkey[i];
        }
    }
    println!("{}",first_activity);
    println!("{}",second_activity);
    println!("---");
    let res:i64 = first_activity*second_activity;
    println!("{}",res);
}
