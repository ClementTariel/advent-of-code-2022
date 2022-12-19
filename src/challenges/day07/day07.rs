#[path = "../../utils/mod.rs"] mod utils;


pub fn part1() {
    let mut sizes:Vec<i32> = [0].to_vec();
    let mut total_sum = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let mut reading_ls: bool = split[0] != &String::from("$");
                if reading_ls{
                    //println!("{}",line_content);
                    if split[0] == &String::from("dir") {
                        //let dir_name = String::from(split[1]);
                        //println!("dir found with name {dir_name}");
                        continue;
                    }else{
                        let file_name = String::from(split[1]);
                        let file_size:i32 = String::from(split[0]).parse().unwrap();
                        //println!("file found with name {file_name} and size {file_size:?}");
                        let n = sizes.len();
                        sizes[n-1] += file_size;
                        continue;
                    }
                }else{
                    if split[1] == &String::from("ls") {
                        //println!("ls");
                        continue;
                    }
                    if split[1] == &String::from("cd") {
                        if split[2] == &String::from(".."){
                            //println!("go to prev dir");
                            let n = sizes.len();
                            let local_sum = sizes[n-1];
                            sizes[n-2] += local_sum;
                            if local_sum <= 100000{
                                total_sum += local_sum;
                            }
                            sizes.remove(n-1);
                        }else{
                            //let new_dir = String::from(split[2]);
                            //println!("go to dir {new_dir:?}"); 
                            sizes.push(0);
                        }
                        continue;
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    while sizes.len() > 1{
        println!("going back to /");
        let n = sizes.len();
        let local_sum = sizes[n-1];
        sizes[n-2] += local_sum;
        if local_sum <= 100000{
            total_sum += local_sum;
        }
        sizes.remove(n-1);
    }
    println!("{}",sizes[0]);
    println!("{}",total_sum);

}

fn insert_in_sorted_vec(mut v:Vec<i32>, val:i32) -> Vec<i32> {
    let mut i = 0;
    while i < v.len() && v[i] < val{
        i += 1;
    }
    v.insert(i, val);
    return v;
}

pub fn part2() {
    let mut sorted_dir_sizes:Vec<i32> = [].to_vec();
    let mut sizes:Vec<i32> = [0].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(" ").collect::<Vec<&str>>();
                let mut reading_ls: bool = split[0] != &String::from("$");
                if reading_ls{
                    //println!("{}",line_content);
                    if split[0] == &String::from("dir") {
                        //let dir_name = String::from(split[1]);
                        //println!("dir found with name {dir_name}");
                        continue;
                    }else{
                        let file_name = String::from(split[1]);
                        let file_size:i32 = String::from(split[0]).parse().unwrap();
                        //println!("file found with name {file_name} and size {file_size:?}");
                        let n = sizes.len();
                        sizes[n-1] += file_size;
                        continue;
                    }
                }else{
                    if split[1] == &String::from("ls") {
                        //println!("ls");
                        continue;
                    }
                    if split[1] == &String::from("cd") {
                        if split[2] == &String::from(".."){
                            //println!("go to prev dir");
                            let n = sizes.len();
                            let local_sum = sizes[n-1];
                            sizes[n-2] += local_sum;
                            sorted_dir_sizes = insert_in_sorted_vec(sorted_dir_sizes, local_sum);
                            sizes.remove(n-1);
                        }else{
                            //let new_dir = String::from(split[2]);
                            //println!("go to dir {new_dir:?}"); 
                            sizes.push(0);
                        }
                        continue;
                    }
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    while sizes.len() > 1{
        println!("going back to /");
        let n = sizes.len();
        let local_sum = sizes[n-1];
        sizes[n-2] += local_sum;
        sorted_dir_sizes = insert_in_sorted_vec(sorted_dir_sizes, local_sum);
        sizes.remove(n-1);
    }
    println!("{}",sizes[0]);
    let threshold = 30000000- (70000000 - sizes[0]);
    let mut i = 0;
    while i < sorted_dir_sizes.len() && sorted_dir_sizes[i] < threshold{
        i += 1;
    }
    println!("threshold : {threshold:?}");;
    println!("result : {}",sorted_dir_sizes[i]);
}
