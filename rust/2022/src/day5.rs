use std::{fs::File, io::BufReader};
use std::io::prelude::*;

use std::collections::VecDeque;
use std::cmp::min;

fn main() -> std::io::Result<()> {
    let path = "../../input/day5";
    part1(path)?;
    part2(path)?;
    Ok(())
}

fn part1(path: &str) -> std::io::Result<()> {
    let file = File::open(path).expect("File not found");
    let buf_reader = BufReader::new(file);
    
    let mut stacks: Vec<VecDeque<u8>> = Vec::new();

    let mut lines = buf_reader.lines().into_iter();
    loop {
        let s: String = lines.next().unwrap()?;

        if s.len() == 0 {
            break
        }
        
        let b = s.as_bytes();
        for i in 0..(s.len()/4+1) {
            if b[i*4+1] == b'1' {
                break
            }
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            if b[i*4+1] == b' ' {continue};
            stacks[i].push_back(b[i*4+1]);
        }

    }
    
    loop {
        let s: String = match lines.next() {
            Some(Ok(t)) => t,
            _ => break,
        };
        
        let moves: Vec<&str> = s.split(' ').collect();
        let from = moves[3].parse::<usize>().unwrap()-1;
        let to = moves[5].parse::<usize>().unwrap()-1;
        let n = min(
            moves[1].parse::<usize>().unwrap(),
            stacks[from].len()
        );

        for _ in 0..n {
            let val = stacks[from].pop_front().unwrap();
            stacks[to].push_front(val);
        }
    }
    
    for mut stack in stacks {
        let c = match stack.pop_front() {
            Some(t) => t as char,
            None => continue,
        };
        print!("{}", c);
    }
    println!("");
    
    Ok(())
}

fn part2(path: &str) -> std::io::Result<()> {
    let file = File::open(path).expect("File not found");
    let buf_reader = BufReader::new(file);
    
    let mut stacks: Vec<VecDeque<u8>> = Vec::new();

    let mut lines = buf_reader.lines().into_iter();
    loop {
        let s: String = lines.next().unwrap()?;

        if s.len() == 0 {
            break
        }
        
        let b = s.as_bytes();
        for i in 0..(s.len()/4+1) {
            if b[i*4+1] == b'1' {
                break
            }
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            if b[i*4+1] == b' ' {continue};
            stacks[i].push_front(b[i*4+1]);
        }

    }
    
    loop {
        let s: String = match lines.next() {
            Some(Ok(t)) => t,
            _ => break,
        };
        
        let moves: Vec<&str> = s.split(' ').collect();
        let from = moves[3].parse::<usize>().unwrap()-1;
        let to = moves[5].parse::<usize>().unwrap()-1;
        let n = min(
            moves[1].parse::<usize>().unwrap(),
            stacks[from].len()
        );

        let at = stacks[from].len()-n;
        let mut tmp = stacks[from].split_off(at);
        stacks[to].append(&mut tmp);

    }
    
    for mut stack in stacks {
        let c = match stack.pop_back() {
            Some(t) => t as char,
            None => continue,
        };
        print!("{}", c);
    }
    println!("");
    
    Ok(())
}