#[path = "../../utils/mod.rs"] mod utils;

pub fn part1() {
    let mut current_rank = 0;
    let mut ranks: Vec<usize> = vec![];
    let mut values: Vec<i32> = vec![];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let value: i32 = line_content.parse().unwrap();
                ranks.push(current_rank);
                values.push(value);
                current_rank += 1;
            }
        }
    }else{
        println!("lines not ok");
    }
    let n = current_rank;
    for i in 0..n{
        let mut temp_rank = ranks[i] as  i32 + values[i];
        while temp_rank < 0{
            temp_rank += (n as  i32) - 1;
        }
        let new_rank = (temp_rank as usize)%(n-1);
        if new_rank >= ranks[i]{
            for j in 0..n{
                if (ranks[j] > ranks[i]) && (ranks[j] <= (new_rank as usize)){
                    ranks[j] -= 1;
                }
            }
        }else {
            for j in 0..n{
                if (ranks[j] < ranks[i]) && (ranks[j] >= (new_rank as usize)){
                    ranks[j] = (ranks[j] + 1)%n;
                }
            }
        }
        ranks[i] = new_rank as usize;
    }
    let mut rank_ref = 0;
    for i in 0..n{
        if values[i] == 0{
            rank_ref = ranks[i];
        }
    }
    let rank_val1 = (rank_ref+1000)%n;
    let rank_val2 = (rank_ref+2000)%n;
    let rank_val3 = (rank_ref+3000)%n;
    let mut val1 = 0;
    let mut val2 = 0;
    let mut val3 = 0;
    println!("rank of 0 : {}",rank_ref);
    for i in 0..n{
        if ranks[i] == rank_val1 {
            val1 = values[i];
            println!("val1 {} at rank {}",values[i],ranks[i]);
        }else if ranks[i] == rank_val2 {
            val2 = values[i];
            println!("val2 {} at rank {}",values[i],ranks[i]);
        }else if ranks[i] == rank_val3 {
            val3 = values[i];
            println!("val3 {} at rank {}",values[i],ranks[i]);
        }
    }
    println!("coord : {},{},{}",val1,val2,val3);
    println!("sum : {}",val1+val2+val3);
}

pub fn part2() {
    let mut current_rank = 0;
    let mut ranks: Vec<usize> = vec![];
    let mut values: Vec<i64> = vec![];
    let decryption_key: i64 = 811589153;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let value: i64 = line_content.parse().unwrap();
                ranks.push(current_rank);
                values.push(value*decryption_key);
                current_rank += 1;
            }
        }
    }else{
        println!("lines not ok");
    }
    let n = current_rank;
    for k in 0..10{
        println!("mixing numbers ... (round {})",k+1);
        for i in 0..n{
            let mut temp_rank = ranks[i] as  i64 + values[i];
            if temp_rank < 0 {
                temp_rank += ((n as i64)-1)*(1 - temp_rank/((n as i64)-1))
            }
            let new_rank = (temp_rank as usize)%(n-1);
            if new_rank >= ranks[i]{
                for j in 0..n{
                    if (ranks[j] > ranks[i]) && (ranks[j] <= (new_rank as usize)){
                        ranks[j] -= 1;
                    }
                }
            }else {
                for j in 0..n{
                    if (ranks[j] < ranks[i]) && (ranks[j] >= (new_rank as usize)){
                        ranks[j] = (ranks[j] + 1)%n;
                    }
                }
            }
            ranks[i] = new_rank as usize;
        }
    }
    let mut rank_ref = 0;
    for i in 0..n{
        if values[i] == 0{
            rank_ref = ranks[i];
        }
    }
    let rank_val1 = (rank_ref+1000)%n;
    let rank_val2 = (rank_ref+2000)%n;
    let rank_val3 = (rank_ref+3000)%n;
    let mut val1 = 0;
    let mut val2 = 0;
    let mut val3 = 0;
    println!("rank of 0 : {}",rank_ref);
    for i in 0..n{
        if ranks[i] == rank_val1 {
            val1 = values[i];
            println!("val1 {} at rank {}",values[i],ranks[i]);
        }else if ranks[i] == rank_val2 {
            val2 = values[i];
            println!("val2 {} at rank {}",values[i],ranks[i]);
        }else if ranks[i] == rank_val3 {
            val3 = values[i];
            println!("val3 {} at rank {}",values[i],ranks[i]);
        }
    }
    println!("coord : {},{},{}",val1,val2,val3);
    println!("sum : {}",val1+val2+val3);
}
