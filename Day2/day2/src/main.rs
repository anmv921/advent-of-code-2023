use std::fs;

fn main() {
    // Init variables
    let file_path: &str = r#"C:\Users\andri\Desktop\AdventOfCode2023\Day2\day2\data\input.txt"#;
    const RED_CUBES: i32 = 12;
    const GREEN_CUBES: i32 = 13;
    const BLUE_CUBES: i32 = 14;

    // Read input
    let vec_lines: Vec<String> = read_input(file_path);
    
    for line in vec_lines {
        println!("{line}");
    }
}

fn part_one() {
    println!("Another function.");
}

fn read_input(in_filepath: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(in_filepath)
        .expect("Error reading file");
    let lines: std::str::Lines<'_> = contents.lines();
    let out_vs: Vec<String> = lines.map(|l: &str| String::from(l))
        .collect();
    out_vs
}