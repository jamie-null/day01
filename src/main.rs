use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let input_handle = File::open("./input.txt").expect("Couldn't open input!");
    let lines = io::BufReader::new(input_handle).lines();

    let mut seen = HashSet::new();
    for line in lines {
        let x: i64 = line.unwrap().parse::<i64>().expect("Couldn't parse line!");
        seen.insert(x);
    }

    for first in seen.iter(){
        let remainder = 2020 - first;
        for second in seen.iter(){
            let third = remainder - second;
            if seen.contains(&third) {
                println!("{} * {} * {} = {}",first,second,third,first*second*third);
            }
        }
    }
}
