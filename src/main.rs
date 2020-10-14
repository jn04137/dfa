use std::io::prelude::*;
use std::io::{self,BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("DFA-Test.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut text = String::new();
    
    // Gets the alphabet
    buf_reader.read_line(&mut text)?;
    println!("{}", text);
    let alphabet: Vec<&str> = text.split(|c| c=='{' || c=='}' || c==',').collect();
    text.clear();
    
    // Gets the states
    buf_reader.read_line(&mut text)?;
    println!("{}", text);
    let states: Vec<&str> = text.split(|c| c=='{' || c=='}' || c==',').collect();
    text.clear();

    // Gets the start state
    buf_reader.read_line(&mut text)?;
    println!("{}", text);
    let fin_state = text;
  
    let mut text = String::new();
    // Gets the accept states
    buf_reader.read_line(&mut text)?;
    println!("{}", text);
    let accept_states: Vec<&str> = text.split(|c| c=='{' || c=='}' || c==',').collect();
    text.clear();

    Ok(())
}
