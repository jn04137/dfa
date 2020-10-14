use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn vec_format(text: &str) -> Vec<&str> {
    let string: &str = text.trim().trim_matches(|c| c== '}' || c=='{');
    let vector: Vec<&str> = string.split(',').collect(); 
    vector
}

fn main() -> std::io::Result<()> {
    let file = File::open("DFA-Test.txt")?;
    let mut buf_reader = BufReader::new(file);
    
    // Gets the alphabet
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let alphabet: Vec<&str> = vec_format(&text); 
    for x in alphabet {
        print!("{} ", x);
    };
    text.clear();

    println!();
    
    // Gets the states
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let states: Vec<&str> = vec_format(&text);
    for x in states{
        print!("{} ", x);
    };
    text.clear();

    println!();

    // Gets the start state
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let fin_state = text;
    print!("{}", fin_state);
   
    // Gets the accept states
    let mut text = String::new(); // new text var is necessary here b/c prev var was owned by prev func
    buf_reader.read_line(&mut text)?;
    let accept_states: Vec<&str> = vec_format(&text);
    for x in accept_states {
        print!("{} ", x);
    };
    text.clear();

    Ok(())
}
