#[path = "../../utils/mod.rs"] mod utils;

use std::collections::HashMap;

pub struct Valve{
    id: usize,
    tag: String,
    flow_rate: i32,
    tag_of_next: Vec<String>,
    open: bool
}

pub fn compute_max_flow(current_id: usize, valves: &Vec<Valve>, weights: &Vec<Vec<i32>>, mut total_flow_rate: i32,mut total_flow: i32, open: &mut Vec<bool>, mut total_time: i32) -> i32{
    let mut max_flow = total_flow;
    let mut next: Vec<usize> = [].to_vec();
    for j in 0..weights.len(){
        if (weights[current_id][j] > 0) && (!open[j]){
            next.push(j);
        }
    }
    if next.len() > 0{
        for i in 0..next.len(){
            let j = next[i];
            let already_open = open[j];
            if !already_open {
                open[j] = true;
                let mut delta_t = 1 + weights[current_id][j] as i32;
                if total_time+delta_t >= 30 {
                    delta_t = 30 - total_time;
                    let potential_max_flow= total_flow + total_flow_rate*delta_t;
                    if potential_max_flow > max_flow {
                        max_flow = potential_max_flow;
                    }
                }else{
                    total_flow += total_flow_rate*delta_t;
                    total_flow_rate += valves[j].flow_rate;
                    total_time += delta_t;
                    let potential_max_flow = compute_max_flow(j, valves, weights, total_flow_rate, total_flow, open, total_time);
                    if potential_max_flow > max_flow {
                        max_flow = potential_max_flow;
                    }
                    total_time -= delta_t;
                    total_flow_rate -= valves[j].flow_rate;
                    total_flow -= total_flow_rate*delta_t;
                }
                open[j] = false;
            }
        }
    }else{
        return max_flow + total_flow_rate*(30-total_time);
    }
    return max_flow;
}

pub fn part1() {
    let mut valves: Vec<Valve> = vec![];
    let mut weights: Vec<Vec<i32>> = vec![];
    let mut tag_to_id:HashMap<String, usize> = HashMap::new();
    let mut last_id_available: usize = 0;
    let mut aa_id: usize = 42;
    if let Ok(lines) = utils::lines_from_file("./small_input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" has flow rate=").collect::<Vec<&str>>();
                let tag = String::from(split[0].split("Valve ").collect::<Vec<&str>>()[1]);
                if tag == String::from("AA"){
                    aa_id = last_id_available;
                    println!("{aa_id:?}");
                }
                let split2 = split[1].split("; tunnel").collect::<Vec<&str>>();
                let flow_rate:i32 = split2[0].parse().unwrap();
                let split3 = split2[1].split("to valve").collect::<Vec<&str>>()[1].splitn(2, ' ').nth(1).unwrap();
                let mut tag_of_next:Vec<String> = split3.split(", ").map(|s| s.to_string()).collect();
                let valve = Valve{id: last_id_available, tag, flow_rate, tag_of_next, open:false};
                last_id_available += 1;
                tag_to_id.insert(String::from(&valve.tag),valve.id);
                valves.push(valve);
                let mut new_col: Vec<i32> = vec![0];
                let n = weights.len(); 
                for i in 0..n {
                    weights[i].push(0);
                    new_col.push(0);
                }
                weights.push(new_col);
                
            }
        }
    }else{
        println!("lines not ok");
    }
    for (valve_tag, valve_id) in &tag_to_id {
        let valve: &Valve = &valves[*valve_id];
        let i = valve.id;
        for t in &valve.tag_of_next {
            if let Some(j) = tag_to_id.get(t) {
                weights[i][*j] = 1;
            }
        }
    }
    let n = valves.len();
    for i in 0..n{
        println!("{}, {}",valves[i].tag, valves[i].flow_rate);
    }
    for i in 0..n {
        for j in 0..i{
            if (i == aa_id) || (j == aa_id) || ((valves[i].flow_rate > 0) && (valves[j].flow_rate > 0)){
                let mut d = 0;
                let mut next: Vec<usize> = [i,n].to_vec();
                let mut seen: Vec<bool> = [].to_vec();
                for _ in 0..n {
                    seen.push(false);
                }
                while next.len() > 0 && next[0] != j {
                    if next[0] == n{
                        next.push(n);
                        d += 1;
                    }else{
                        for k in 0..n{
                            if (!seen[k]) && (weights[next[0]][k] == 1){
                                seen[k] = true;
                                next.push(k);
                            }
                        }
                    }
                    next.remove(0);
                }
                weights[i][j] = d;
                weights[j][i] = d;
            }
        }
    }
    for i in 0..n{
        for j in 0..n{
            if ((valves[i].flow_rate ==  0) && (i != aa_id)) || (valves[j].flow_rate ==  0){
                weights[i][j] = 0;
            }
        }
    }
    for i in 0..n{
        print!("{i:?} : \t");
        for j in 0..n{
            print!(" {}",weights[i][j]);
        }
        print!("\n");
    }
    let mut total_flow_rate= 0;
    let mut total_flow = 0;
    let mut open: Vec<bool> = [].to_vec();
    let mut total_time = 0;
    for _ in 0..n {
        open.push(false);
    }
    let mut max_flow = 0;
    for j in 0..n{
        if weights[aa_id][j] > 0{
            println!("starting from {} -> {}",aa_id, j);
            open[j] = true;
            total_time += 1 + weights[aa_id][j];
            total_flow_rate += valves[j].flow_rate;
            let max_flow_from_j = compute_max_flow(j, &valves, &weights, total_flow_rate, total_flow, &mut open, total_time);
            total_flow_rate -= valves[j].flow_rate;
            total_time -= 1 + weights[aa_id][j];
            open[j] = false;
            if max_flow_from_j > max_flow{
                max_flow = max_flow_from_j;
            }
        }
    }
    println!("{}",max_flow);
}

pub fn compute_max_flow2(current_id: usize, second_id: usize, delay: i32, valves: &Vec<Valve>, weights: &Vec<Vec<i32>>, mut total_flow_rate: i32,mut total_flow: i32, open: &mut Vec<bool>, mut total_time: i32) -> i32{
    let mut max_flow = total_flow;
    let mut next: Vec<usize> = [].to_vec();
    for j in 0..weights.len(){
        if (weights[current_id][j] > 0) && (!open[j]) && (j != second_id){
            next.push(j);
        }
    }
    if next.len() ==  0{
        if (weights[current_id][second_id] > 0) && (weights[current_id][second_id] < delay) && (!open[second_id]){
            next.push(second_id);
        }
    }
    if next.len() > 0{
        for i in 0..next.len(){
            let j = next[i];
            let mut i1 = second_id;
            let mut i2 = j;
            let mut short_time = delay;
            let mut new_delay = 1 + weights[current_id][j] - delay;
            if weights[current_id][j] < delay {
                i1 = j;
                i2 = second_id;
                short_time = 1 + weights[current_id][j];
                new_delay = delay - short_time;
            }
            if total_time+short_time >= 30 {
                short_time = 30 - total_time;
                let potential_max_flow= total_flow + total_flow_rate*short_time;
                if potential_max_flow > max_flow {
                    max_flow = potential_max_flow;
                }
            }else{
                total_flow += total_flow_rate*short_time;
                total_time += short_time;
                open[i1] = true;
                total_flow_rate += valves[i1].flow_rate;
                let potential_max_flow = compute_max_flow2(i1, i2, new_delay, valves, weights, total_flow_rate, total_flow, open, total_time);
                if potential_max_flow > max_flow {
                    max_flow = potential_max_flow;
                }
                total_flow_rate -= valves[i1].flow_rate;
                open[i1] = false;
                total_time -= short_time;
                total_flow -= total_flow_rate*short_time;
            }
        }
    }else{
        let mut new_delay = delay;
        if total_time+delay >= 30 {
            new_delay = 30-total_time;
        }
        total_flow += total_flow_rate*new_delay;
        total_time += new_delay;
        if !open[second_id]{
            open[second_id] = true;
            total_flow_rate += valves[second_id].flow_rate;
            max_flow = total_flow + total_flow_rate*(30-total_time);
            total_flow_rate -= valves[second_id].flow_rate;
            open[second_id] = false;
        }else{
            max_flow = total_flow + total_flow_rate*(30-total_time);
        }
        total_time -= new_delay;
        total_flow -= total_flow_rate*new_delay;
    }
    return max_flow;
}


pub fn part2() {
    let mut valves: Vec<Valve> = vec![];
    let mut weights: Vec<Vec<i32>> = vec![];
    let mut tag_to_id:HashMap<String, usize> = HashMap::new();
    let mut last_id_available: usize = 0;
    let mut aa_id: usize = 42;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" has flow rate=").collect::<Vec<&str>>();
                let tag = String::from(split[0].split("Valve ").collect::<Vec<&str>>()[1]);
                if tag == String::from("AA"){
                    aa_id = last_id_available;
                    println!("{aa_id:?}");
                }
                let split2 = split[1].split("; tunnel").collect::<Vec<&str>>();
                let flow_rate:i32 = split2[0].parse().unwrap();
                let split3 = split2[1].split("to valve").collect::<Vec<&str>>()[1].splitn(2, ' ').nth(1).unwrap();
                let mut tag_of_next:Vec<String> = split3.split(", ").map(|s| s.to_string()).collect();
                let valve = Valve{id: last_id_available, tag, flow_rate, tag_of_next, open:false};
                last_id_available += 1;
                tag_to_id.insert(String::from(&valve.tag),valve.id);
                valves.push(valve);
                let mut new_col: Vec<i32> = vec![0];
                let n = weights.len(); 
                for i in 0..n {
                    weights[i].push(0);
                    new_col.push(0);
                }
                weights.push(new_col);
                
            }
        }
    }else{
        println!("lines not ok");
    }
    for (valve_tag, valve_id) in &tag_to_id {
        let valve: &Valve = &valves[*valve_id];
        let i = valve.id;
        for t in &valve.tag_of_next {
            if let Some(j) = tag_to_id.get(t) {
                weights[i][*j] = 1;
            }
        }
    }
    let n = valves.len();
    for i in 0..n{
        println!("{}, {}",valves[i].tag, valves[i].flow_rate);
    }
    for i in 0..n {
        for j in 0..i{
            if (i == aa_id) || (j == aa_id) || ((valves[i].flow_rate > 0) && (valves[j].flow_rate > 0)){
                let mut d = 0;
                let mut next: Vec<usize> = [i,n].to_vec();
                let mut seen: Vec<bool> = [].to_vec();
                for _ in 0..n {
                    seen.push(false);
                }
                while next.len() > 0 && next[0] != j {
                    if next[0] == n{
                        next.push(n);
                        d += 1;
                    }else{
                        for k in 0..n{
                            if (!seen[k]) && (weights[next[0]][k] == 1){
                                seen[k] = true;
                                next.push(k);
                            }
                        }
                    }
                    next.remove(0);
                }
                weights[i][j] = d;
                weights[j][i] = d;
            }
        }
    }
    for i in 0..n{
        for j in 0..n{
            if ((valves[i].flow_rate ==  0) && (i != aa_id)) || (valves[j].flow_rate ==  0){
                weights[i][j] = 0;
            }
        }
    }
    for i in 0..n{
        print!("{i:?} : \t");
        for j in 0..n{
            print!(" {}",weights[i][j]);
        }
        print!("\n");
    }
    let mut total_flow_rate= 0;
    let mut total_flow = 0;
    let mut open: Vec<bool> = [].to_vec();
    let mut total_time = 4;
    for _ in 0..n {
        open.push(false);
    }
    let mut max_flow = 0;
    for i in 0..n{
        for j in 0..i{
            if (weights[aa_id][i] > 0) && (weights[aa_id][j] > 0){
                let mut i1 = i;
                let mut i2 = j;
                if weights[aa_id][i] > weights[aa_id][j] {
                    i1 = j;
                    i2 = i;
                }
                let mut short_time = 1 + weights[aa_id][i1];
                let mut delay = weights[aa_id][i2] - weights[aa_id][i1];
                total_flow += total_flow_rate*short_time;
                total_time += short_time;
                open[i1] = true;
                total_flow_rate += valves[i1].flow_rate;
                let max_flow_from_i1 = compute_max_flow2(i1, i2, delay, &valves, &weights, total_flow_rate, total_flow, &mut open, total_time);
                total_flow_rate -= valves[i1].flow_rate;
                open[i1] = false;
                total_time -= short_time;
                total_flow -= total_flow_rate*short_time;
                if max_flow_from_i1 > max_flow{
                    max_flow = max_flow_from_i1;
                }
                println!("best flow computed yet : {}",max_flow);
            }
        }
    }
    println!("best flow final :\n{}",max_flow);
}
