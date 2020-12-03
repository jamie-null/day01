use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let input_handle = File::open("./input.txt").expect("Couldn't open input!");
    let lines = io::BufReader::new(input_handle).lines();

    let mut seen = HashSet::new();
    for line in lines {
        let x: i64 = line.unwrap().parse::<i64>().expect("Couldn't parse line!");
        if seen.contains(&x){
            println!("{} * {} = {}",x,2020-x,x * (2020-x));
        } else {
            seen.insert(2020-x);
        }
    }

}
