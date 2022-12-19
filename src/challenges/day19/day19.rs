use std::default;

#[path = "../../utils/mod.rs"] mod utils;

pub fn max_geodes(current_max:i32, bots: &mut [i32;4], resources: &mut [i32;4], optimisation: &[i32;4], cost: &Vec<Vec<i32>>, time_left: i32) -> i32 {
    let n = cost.len();
    if time_left <= 0{
        return resources[3];
    }
    //if geodes bots can be produced each turns
    if (bots[0] >= cost[3][0]) && (bots[2] >= cost[3][2]){
        return resources[3] + bots[3]*time_left + ((time_left*(time_left-1))/2)
    }
    //quick upper bound for the number of geodes
    if resources[3] + bots[3]*time_left + ((time_left*(time_left-1))/2) <= current_max{
        return 0;
    }
    //more complicated upper bound for the number of geodes
    let mut ore = resources[0];
    let mut clay = resources[1];
    let mut obsidian = resources[2];
    let mut geodes = resources[3];
    let mut ore_bot = bots[0];
    let mut clay_bot = bots[1];
    let mut obsidian_bot = bots[2];
    let mut geodes_bot = bots[3];
    let mut time_left_for_upper_bound = time_left;
    while (time_left_for_upper_bound > 0) && (obsidian_bot < cost[3][2]){
        ore += ore_bot;
        clay += clay_bot;
        obsidian += obsidian_bot;
        geodes += geodes_bot;
        if clay + clay_bot >=  cost[2][1] {
            obsidian_bot += 1;
            clay -=  cost[2][1];
        }else{
            clay_bot += 1;
        }
        if obsidian >= cost[3][2]{
            obsidian -= cost[3][2];
            geodes_bot += 1;
        }
        time_left_for_upper_bound -= 1;
    }
    if geodes + geodes_bot*time_left_for_upper_bound + ((time_left_for_upper_bound*(time_left_for_upper_bound-1))/2) <= current_max{
        return 0;
    }
    
    for i in 0..n{
        resources[i] += bots[i];
    }
    let mut final_max = current_max;
    for i in 0..n{
        let k = n-1-i;
        if bots[k] < optimisation[k]{
            let mut got_the_resources = true;
            for j in 0..cost[k].len(){
                // + bots[j] because the resource was not supposed to be mined already
                if resources[j] < cost[k][j] + bots[j]{
                    got_the_resources = false;
                    break;
                }
            }
            if got_the_resources{
                for j in 0..cost[k].len(){
                    resources[j] -= cost[k][j];
                }
                bots[k] += 1;
                let local_max = max_geodes(final_max, bots, resources, optimisation, cost, time_left-1);
                if local_max > final_max{
                    final_max = local_max;
                }
                bots[k] -= 1;
                for j in 0..cost[k].len(){
                    resources[j] += cost[k][j];
                }
            }
        }
    }
    let local_max = max_geodes(final_max, bots, resources, optimisation, cost, time_left-1);
    if local_max > final_max{
        final_max = local_max;
    }
    for i in 0..n{
        resources[i] -= bots[i];
    }
    return final_max;
}

pub fn part1() {
    let time_limit = 24;
    let mut quality_sum = 0;
    let mut blueprint_id = 1;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut cost: Vec<Vec<i32>> = vec![];
                let split = line_content.split("Each ore robot costs ").collect::<Vec<&str>>();
                // ore robots
                let split_1 = split[1].split(" ore. Each clay robot costs ").collect::<Vec<&str>>();
                let mut cost_row:Vec<i32> = vec![0,0,0];
                cost_row[0] = split_1[0].parse().unwrap();
                cost.push(cost_row);
                // clay robots
                let split_2 = split_1[1].split(" ore. Each obsidian robot costs ").collect::<Vec<&str>>();
                let mut cost_row:Vec<i32> = vec![0,0,0];
                cost_row[0] = split_2[0].parse().unwrap();
                cost.push(cost_row);
                // obsidian robots
                let mut cost_row:Vec<i32> = vec![0,0,0];
                let split_3 = split_2[1].split(" ore and ").collect::<Vec<&str>>();
                cost_row[0] = split_3[0].parse().unwrap();
                let split_4 = split_3[1].split(" clay. Each geode robot costs ").collect::<Vec<&str>>();
                cost_row[1] = split_4[0].parse().unwrap();
                cost.push(cost_row);
                // geode cracking robots
                let mut cost_row:Vec<i32> = vec![0,0,0];
                cost_row[0] = split_4[1].parse().unwrap();
                let split_5 = split_3[2].split(" obsidian.").collect::<Vec<&str>>();
                cost_row[2] = split_5[0].parse().unwrap();
                cost.push(cost_row);
                let mut bots: [i32;4] = [1,0,0,0];
                let mut resources: [i32;4] = [0,0,0,0];
                let mut optimisation: [i32;4] = [cost[0][0],0,0,time_limit*time_limit];
                if cost[1][0] > optimisation[0]{
                    optimisation[0] = cost[1][0];
                }
                if cost[2][0] > optimisation[0]{
                    optimisation[0] = cost[2][0];
                }
                if cost[3][0] > optimisation[0]{
                    optimisation[0] = cost[3][0];
                }
                if cost[2][1] > optimisation[1]{
                    optimisation[1] = cost[2][1];
                }
                if cost[3][2] > optimisation[2]{
                    optimisation[2] = cost[3][2];
                }
                println!("blueprint {}; ore cost {}, clay cost {}, obs cost {} and {}, geode cost {} and {}",blueprint_id,cost[0][0],cost[1][0],cost[2][0],cost[2][1],cost[3][0],cost[3][2]);
                let current_max = 0;
                let mut final_max = max_geodes(current_max, &mut bots, &mut resources, &optimisation, &cost, time_limit);
                let quality_level = final_max*blueprint_id;
                quality_sum += quality_level;
                println!("max geode = {}, quality level = {}, (cumulated quality sum = {})",final_max,quality_level,quality_sum);
                blueprint_id += 1;
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("cumulated quality sum = {}",quality_sum);

}


pub fn part2() {
    let time_limit = 32;
    let mut final_product = 1;
    let mut blueprint_id = 1;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if blueprint_id <= 3{
                    let mut cost: Vec<Vec<i32>> = vec![];
                    let split = line_content.split("Each ore robot costs ").collect::<Vec<&str>>();
                    // ore robots
                    let split_1 = split[1].split(" ore. Each clay robot costs ").collect::<Vec<&str>>();
                    let mut cost_row:Vec<i32> = vec![0,0,0];
                    cost_row[0] = split_1[0].parse().unwrap();
                    cost.push(cost_row);
                    // clay robots
                    let split_2 = split_1[1].split(" ore. Each obsidian robot costs ").collect::<Vec<&str>>();
                    let mut cost_row:Vec<i32> = vec![0,0,0];
                    cost_row[0] = split_2[0].parse().unwrap();
                    cost.push(cost_row);
                    // obsidian robots
                    let mut cost_row:Vec<i32> = vec![0,0,0];
                    let split_3 = split_2[1].split(" ore and ").collect::<Vec<&str>>();
                    cost_row[0] = split_3[0].parse().unwrap();
                    let split_4 = split_3[1].split(" clay. Each geode robot costs ").collect::<Vec<&str>>();
                    cost_row[1] = split_4[0].parse().unwrap();
                    cost.push(cost_row);
                    // geode cracking robots
                    let mut cost_row:Vec<i32> = vec![0,0,0];
                    cost_row[0] = split_4[1].parse().unwrap();
                    let split_5 = split_3[2].split(" obsidian.").collect::<Vec<&str>>();
                    cost_row[2] = split_5[0].parse().unwrap();
                    cost.push(cost_row);
                    let mut bots: [i32;4] = [1,0,0,0];
                    let mut resources: [i32;4] = [0,0,0,0];
                    let mut optimisation: [i32;4] = [cost[0][0],0,0,time_limit*time_limit];
                    if cost[1][0] > optimisation[0]{
                        optimisation[0] = cost[1][0];
                    }
                    if cost[2][0] > optimisation[0]{
                        optimisation[0] = cost[2][0];
                    }
                    if cost[3][0] > optimisation[0]{
                        optimisation[0] = cost[3][0];
                    }
                    if cost[2][1] > optimisation[1]{
                        optimisation[1] = cost[2][1];
                    }
                    if cost[3][2] > optimisation[2]{
                        optimisation[2] = cost[3][2];
                    }
                    println!("blueprint {}; ore cost {}, clay cost {}, obs cost {} and {}, geode cost {} and {}",blueprint_id,cost[0][0],cost[1][0],cost[2][0],cost[2][1],cost[3][0],cost[3][2]);
                    let current_max = 0;
                    let mut final_max = max_geodes(current_max, &mut bots, &mut resources, &optimisation, &cost, time_limit);
                    final_product *= final_max;
                    println!("max geode = {}, (cumulated product = {})",final_max,final_product);
                    blueprint_id += 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("final product = {}",final_product);

}