use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

pub fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// pub fn get_last_day() -> i32{
//     let mut last_day = 0;
//     for file in fs::read_dir("./src/challenges").unwrap() {
//         let mut day_name = file.unwrap().path().display().to_string();
//         let day = day_name[day_name.len()-2..day_name.len()].parse().unwrap();
//         if day > last_day {
//             last_day = day;
//         }
//     }
//     return last_day;
// }

// pub fn day_to_file_path(day: i32) -> String {
//     let day_name = String::from("day") + if day<10 {"0"}else{""} + &day.to_string();
//     let file_path = String::from("challenges/") + &day_name +"/" + &day_name + ".rs";
//     return file_path;
// }