use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // --------------------------------------------------------------------------------------------
    // Read input file
    let file_path: &str = r"C:\Users\andri\Desktop\AdventOfCode2023\Day1\day1\data\input.txt";
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines: std::str::Lines<'_> = contents.lines();
    // --------------------------------------------------------------------------------------------
    let mut hm_numbers: HashMap<String, u128> = HashMap::new();
    hm_numbers.insert(String::from("one"), 1);
    hm_numbers.insert(String::from("two"), 2);
    hm_numbers.insert(String::from("three"), 3);
    hm_numbers.insert(String::from("four"), 4);
    hm_numbers.insert(String::from("five"), 5);
    hm_numbers.insert(String::from("six"), 6);
    hm_numbers.insert(String::from("seven"), 7);
    hm_numbers.insert(String::from("eight"), 8);
    hm_numbers.insert(String::from("nine"), 9);
    // --------------------------------------------------------------------------------------------
    
    // Get calibration value
    let mut sum: u128 = 0;

    for line in lines {

        let mut index_first: usize = 100000;
        let mut index_last: usize = 0;

        let mut first: String = String::new();
        let mut last: String = String::new();

        let arr_patterns: [&str; 18] = ["1", "2", "3", "4", "5", "6", "7", "8", "9",
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for pattern in arr_patterns {
            if line.contains(pattern) {
                let vec_match_indices: Vec<_> = line.match_indices(pattern).collect();

                let mut tpl_m: &(usize, &str) = vec_match_indices.first().expect("Erro");
                
                let (ind, _found_pat) = tpl_m;
                let mut index_m: usize = *ind;

                if index_m <= index_first {
                    first.clear();
                    first.push_str(pattern);
                    index_first = index_m;
                }

                tpl_m = vec_match_indices.last().expect("Erro");
                let (ind, _found_pat) = tpl_m;
                index_m = *ind;

                if index_m >= index_last {
                    last.clear();
                    last.push_str(pattern);
                    index_last = index_m;
                }
            }
        }

        let re_num = Regex::new(r"\d").unwrap();

        if !re_num.is_match(&first) {
            let key: String = String::from(first);
            let val: u128 = hm_numbers.get(&key).copied().unwrap();
            first = val.to_string();
        }
        if !re_num.is_match(&last) {
            let key: String = String::from(last);
            let val: u128 = hm_numbers.get(&key).copied().unwrap();
            last = val.to_string();
        }

        let mut calibration_value: String = String::new();
        calibration_value.push_str(&first);
        calibration_value.push_str(&last);

        let calibration_value: u128 = calibration_value
            .trim()
            .parse()
            .expect("NaN!");

        sum += calibration_value;
    }
    println!("{sum}");
}