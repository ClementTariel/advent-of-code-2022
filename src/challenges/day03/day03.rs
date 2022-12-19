#[path = "../../utils/mod.rs"] mod utils;

fn char_to_item_priority(c: char) -> i32{
    if c as u32 - 'A' as u32 >= 26 {
        return 1 + (c as u32 - 'a' as u32) as i32;
    }else{
        return 27 + (c as u32 - 'A' as u32) as i32;
    }
}

pub fn part1() {
    let mut total = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut items: [i32;52] = [0;52];
                let milestone = &line_content.len()/2;
                for (i, c) in line_content.chars().enumerate() {
                    if i < milestone {
                        items[(char_to_item_priority(c)-1) as usize] = 1;
                    }else{
                        if items[(char_to_item_priority(c)-1) as usize] == 1 {
                            total += char_to_item_priority(c);
                            break;
                        }
                    }
                    
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
    let mut counter = 0;
    let mut items: [i32;52] = [0;52];
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                if counter == 0 {
                    items = [0;52];
                }
                for c in line_content.chars() {
                    if items[(char_to_item_priority(c)-1) as usize] == counter {
                        items[(char_to_item_priority(c)-1) as usize] = counter + 1;
                        if counter == 2 {
                            total += char_to_item_priority(c);
                            break;
                        }
                    }
                }
                counter = (counter + 1)%3;
                
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("{total:?}");
}
