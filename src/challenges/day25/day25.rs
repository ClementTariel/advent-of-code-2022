#[path = "../../utils/mod.rs"] mod utils;

pub fn decipher_digit(c: char) -> i8 {
    if c == '=' {
        return -2;
    }
    if c == '-' {
        return -1;
    }
    return c as i8 - '0' as i8;
}

pub fn cipher_digit(d: i8) -> char {
    return ['=','-','0','1','2'][(d+2) as usize];
}

pub fn convert_to_snafu(n: i64) -> Vec<char>{
    let mut d: i8;
    let mut q = n;
    let mut snafu:Vec<char> = vec![];
    while q != 0{
        d = (q%5) as i8;
        q = q/5;
        if d > 2{
            d -=  5;
            q += 1;
        }
        snafu.push(cipher_digit(d));
    }
    return snafu;
}

pub fn convert_from_snafu(snafu: Vec<char>) -> i64{
    let mut n = 0;
    let mut p: i64 = 1;
    for c in snafu{
        if c == '='{
            n += -2*p;
        }
        if c == '-'{
            n += -1*p;
        }
        if c == '1'{
            n += 1*p;
        }
        if c == '2'{
            n += 2*p;
        }
        p *= 5;
    }
    return n;
}

pub fn part1() {
    let mut fuel_count = 0;
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let mut snafu: Vec<char> = vec![];
                for c in line_content.chars(){
                    snafu.insert(0, c);
                }
                for i in 0..snafu.len(){
                    print!("{}",snafu[snafu.len()-1-i]);
                }
                let n = convert_from_snafu(snafu);
                println!(" -> {}",n);
                fuel_count += n;
            }
        }
    }else{
        println!("lines not ok");
    }
    println!("fuel_count : {}, written in snafu :",fuel_count);
    let snafu = convert_to_snafu(fuel_count);
    for i in 0..snafu.len(){
        print!("{}",snafu[snafu.len()-1-i]);
    }
    print!("\n");
}

pub fn part2() {
    println!("there is no part 2 !");
}
