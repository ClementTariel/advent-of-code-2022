use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[macro_export]
macro_rules! get_folder_path{
    () => {
        pub fn path_to_file_from_project_root(filename: &str) -> String{
            return String::from("challenges/day")+&(env!("AOC_DAY"))+"/"+filename;
        }
    }
    ;
}

get_folder_path!();
// pub fn lines_from_complete_path(filename: String) -> io::Result<io::Lines<io::BufReader<File>>>
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

// pub fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let complete_path = path_to_file_from_project_root!(stringify!(filename));
//     lines_from_complete_path(complete_path)
// }

// // original
// pub fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

pub fn lines_from_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let complete_path = String::from(env!("CARGO_MANIFEST_DIR")) + "/src/" + &path_to_file_from_project_root(filename);
    let file = File::open(complete_path)?;
    Ok(io::BufReader::new(file).lines())
}