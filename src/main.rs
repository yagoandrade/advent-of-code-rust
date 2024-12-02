use std::env;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        return;
    }
    let file_path = &args[1];

    let input = read_file(file_path);
    solutions::year2024::day01::part01(&input);
    solutions::year2024::day01::part02(&input);
}

fn read_file(file_path: &str) -> String {
    use std::fs;
    fs::read_to_string(file_path).expect("Something went wrong reading the file")
}
