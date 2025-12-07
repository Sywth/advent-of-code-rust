mod advent_of_code;
mod util;

const DAY: u32 = 7;
fn main() {
    match DAY {
        1 => advent_of_code::day_1::main(),
        2 => advent_of_code::day_2::main(),
        3 => advent_of_code::day_3::main(),
        7 => advent_of_code::day_7::main(),
        _ => println!("Day not implemented yet"),
    }
}
