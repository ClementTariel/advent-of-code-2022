#[macro_export]
macro_rules! include_day{
    ($n:expr) => {
        pub fn get_current_day() -> i32{
            let n:i32 = stringify!($n).parse().unwrap();
            return n;
        }
        include!(concat!("challenges/day",stringify!($n),"/day",stringify!($n),".rs"));
        
    };
    () => {
        pub fn get_current_day() -> i32{
            let n:i32 = env!("AOC_DAY").parse().unwrap();
            return n;
        }
        include!(concat!("challenges/day",env!("AOC_DAY"),"/day",env!("AOC_DAY"),".rs"));
    }
    ;
}

include_day!();

fn main() {
    println!("{}",env!("CARGO_MANIFEST_DIR"));
    println!("solving day{}",get_current_day());
    println!("part1");
    part1();
    println!("part2");
    part2();    
}


