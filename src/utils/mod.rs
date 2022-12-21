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

pub fn lines_from_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    #[cfg(feature = "small")]
    println!("using small_input.txt");
    #[cfg(feature = "small")]
    let complete_path = String::from(env!("CARGO_MANIFEST_DIR")) + "/src/" + &path_to_file_from_project_root("small_input.txt");
    #[cfg(not(feature = "small"))]
    let complete_path = String::from(env!("CARGO_MANIFEST_DIR")) + "/src/" + &path_to_file_from_project_root(filename);
    let file = File::open(complete_path)?;
    Ok(io::BufReader::new(file).lines())
}