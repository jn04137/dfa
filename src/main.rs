use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

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
    
    // Gets the states
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let states: Vec<&str> = vec_format(&text);

    // Gets the start state
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let start_state = text;
   
    // Gets the accept states
    let mut text = String::new(); // new text var is necessary here b/c prev var was owned by prev func
    buf_reader.read_line(&mut text)?;
    let accept_states: Vec<&str> = vec_format(&text);

    let mut transitive_functions: HashMap<String, char> = HashMap::new();
    
    let mut text = String::new();
    let mut bytes = buf_reader.read_line(&mut text)?;
    while bytes != 0 {
        let state = text.chars().nth(1).unwrap();
        let input = text.chars().nth(3).unwrap();
        let next_state = text.chars().nth(7).unwrap();

        let mut key = String::new();
        
        key.push(state);
        key.push(input);

        transitive_functions.insert(key, next_state);
        text.clear();
        bytes = buf_reader.read_line(&mut text)?;
    }

    for (key, val) in transitive_functions.iter() {
        println!("key: {}, val: {}", key, val);
    }

    Ok(())
}
