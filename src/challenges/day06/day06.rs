#[path = "../../utils/mod.rs"] mod utils;


pub fn part1() {
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut buf:Vec<char> = [].to_vec();
                let mut input_chars:Vec<char> = line_content.chars().collect();
                let mut i:i32=0;
                while (i as usize) < input_chars.len(){
                    while buf.len() > 4{
                        buf.remove(0);
                    }     
                    let mut duplicate:bool = (buf.len() < 4);
                    for j in 0..(buf.len()){
                        for k in (j+1)..(buf.len()) {
                            if buf[j] == buf[k] {
                                duplicate = true;
                            }
                        }
                    }
                    buf.push(input_chars[i as usize]);
                    if !duplicate {
                        println!("result {i:?}");
                        i = input_chars.len() as i32;
                    }else{
                        i += 1;
                    }
                    
                }
            }
        }
    }else{
        println!("lines not ok");
    }

}

pub fn part2() {
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut buf:Vec<char> = [].to_vec();
                let mut input_chars:Vec<char> = line_content.chars().collect();
                let mut i:i32=0;
                while (i as usize) < input_chars.len(){
                    while buf.len() > 14{
                        buf.remove(0);
                    }     
                    let mut duplicate:bool = (buf.len() < 14);
                    for j in 0..(buf.len()){
                        for k in (j+1)..(buf.len()) {
                            if buf[j] == buf[k] {
                                duplicate = true;
                            }
                        }
                    }
                    buf.push(input_chars[i as usize]);
                    if !duplicate {
                        println!("result {i:?}");
                        i = input_chars.len() as i32;
                    }else{
                        i += 1;
                    }
                    
                }
            }
        }
    }else{
        println!("lines not ok");
    }
}
