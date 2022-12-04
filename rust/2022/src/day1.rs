use std::{fs::File, io::BufReader};
use std::io::prelude::*;

use std::vec::Vec;


fn main() -> std::io::Result<()> {
    let file = File::open("../../input/day1").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut vec: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;
    for line in buf_reader.lines() {
        let s: String = line?;
        if s.len() == 0 {
            vec.push(sum);
            sum = 0;
            continue;
        }
        sum += s.parse::<i32>().unwrap();
        
    }
    vec.push(sum);
    
    let max_val = vec.iter().max().unwrap();
    
    println!("Max: {}", max_val);
    

    vec.sort();

    sum = 0;
    for _ in 0..3 {
        sum += vec.pop().unwrap();
    }

    println!("Top 3: {}", sum);

    Ok(())
}
