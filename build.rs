use std::env;
use std::fs::{self};

pub fn get_last_day() -> i32{
    let mut last_day = 0;
    for file in fs::read_dir(String::from(env!("CARGO_MANIFEST_DIR"))+"/src/challenges/").unwrap() {
        let day_name = file.unwrap().path().display().to_string();
        let day = day_name[day_name.len()-2..day_name.len()].parse().unwrap();
        if day > last_day {
            last_day = day;
        }
    }
    return last_day;
}

pub fn day_as_str(day: i32) -> String{
    return String::from("") + if day<10 {"0"}else{""} + &day.to_string();
}
pub fn day_to_file_path(day: i32) -> String {
    let day_name = String::from("day")+&day_as_str(day);
    let file_path = String::from("challenges/") + &day_name +"/" + &day_name + ".rs";
    return file_path;
}

fn main() {
    let current_day = get_last_day();
    let day_str = day_as_str(current_day);
    let file_path = day_to_file_path(current_day);
    match option_env!("AOC_DAY") {
        Some(val) => {
            match val.parse::<i32>() {
                Ok(n) => {
                    if (n >= 1) && (n <= 25)  {
                        println!("cargo:rustc-env=AOC_DAY={}",day_as_str(n));
                    }else{
                        panic!("\n\n/!\\ AOC_DAY should be between 1 and 25\n\n");
                    }
                },
                Err(e) => {
                    panic!("\n\n/!\\ AOC_DAY should be between 1 and 25\n\n");
                },
              }
            
        },
        None => println!("cargo:rustc-env=AOC_DAY={}",day_str),
    }
    println!("cargo:warning=day set");
    

}