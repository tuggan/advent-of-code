use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("../../input/day4").expect("File not found");
    let buf_reader = BufReader::new(file);
    
    let mut finds = 0;
    let mut overlaps = 0;
    
    for line in buf_reader.lines() {
        let string = line?;
        let groups = string.split(",").collect::<Vec<&str>>();
        let group1 = groups[0].split('-');
        let s1: Vec<&str> = group1.collect();
        let start1 = s1[0].parse::<i32>().unwrap();
        let end1 = s1[1].parse::<i32>().unwrap();
        
        let group2 = groups[1].split('-');
        let s2: Vec<&str> = group2.collect();
        let start2 = s2[0].parse::<i32>().unwrap();
        let end2 = s2[1].parse::<i32>().unwrap();

        if start1 <= start2 && end1 >= end2 || start2 <= start1 && end2 >= end1 {
            finds += 1;
        } else if start1 <= start2 && end1 >= start2 || start2 <= start1 && end2 >= start1{
            overlaps += 1;
        }
    }
    
    println!("Found: {finds}");
    println!("Overlaps: {}", overlaps+finds);
    
    Ok(())
}