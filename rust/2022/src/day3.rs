use std::{fs::File, io::BufReader};
use std::io::prelude::*;

// use std::iter::Unfold;

fn main() -> std::io::Result<()> {
    let file = File::open("../../input/day3").expect("File not found");
    let buf_reader = BufReader::new(file);
    
    let total = part_1(buf_reader);
        
    println!("Part 1: {total}");

    let file = File::open("input/day3").expect("File not found");
    let buf_reader = BufReader::new(file);
    let p2 = part_2(buf_reader);
    
    println!("Part 2: {p2}");
    
    Ok(())
}

fn part_1(buf_reader: BufReader<File>) -> i64 {
    let mut total: i64 = 0;

    for line in buf_reader.lines() {
        let string = line.unwrap();
        // println!("String: {}, Lenght: {}", string, string.len());
        let (first, second) = string.split_at(string.len()/2);
        // println!("First: {}, Second: {}", first, second);
            
        for c in first.chars() {
            let mut found = false;
            for c2 in second.chars() {
                if c == c2 {
                    found = true;
                    total += sum(c) as i64;
                    // print!("{c}");
                    break
                }
            }
            if found {
                break
            }
        }
    }
    return total;

}

fn part_2(buf_reader: BufReader<File>) -> i64 {
    let mut file: Vec<Vec<u8>> = Vec::new();
    
    for line in buf_reader.lines() {
        file.push(line.unwrap().into_bytes());        
    }
    
    let mut total: i64 = 0;

    // for x in Unfold::new(0i32, |x| { (*x) = (*x)+3; if *x < file.len() {Some(*x)} else {None}}) { 
    // for x in seq(0, |x| x+3).take_while(|&x| x < file.len()) {
    let mut x = 0;
    while x < file.len() {
        file[x].sort_unstable();
        file[x].dedup();
        file[x+1].sort_unstable();
        file[x+1].dedup();
        file[x+2].sort_unstable();
        file[x+2].dedup();
        
        // println!("{:?}\n{:?}\n{:?}", file[x], file[x+1], file[x+2]);

        let mut i1 = 0;
        let mut i2 = 0;
        let mut i3 = 0;

        let mut not_found = true;
        while not_found {
            if file[x][i1] == file[x+1][i2] && file[x+1][i2] == file[x+2][i3] {
                // println!("Found match {}", file[x][i1] as char);
                total += sum(file[x][i1] as char) as i64;
                not_found = false;
            }
            // println!("{}, {}, {}", file[x][i1], file[x+1][i2], file[x+2][i3]);
            if file[x][i1] < file[x+1][i2] {
                i1 += 1;
            }
            if file[x+1][i2] < file[x+2][i3] {
                i2 += 1;
            }
            if file[x+2][i3] < file[x][i1] {
                i3 += 1;
            }
            // println!("{}, {}, {}", file[x][i1], file[x+1][i2], file[x+2][i3]);
        }
        x += 3;
    }

    return total;
}

fn sum(c: char) -> u8 {
    if {c as u8} < {'a' as u8} {
        return {c as u8} - {'A' as u8}+27;
    }
    return c as u8 - 'a' as u8+1;
}
