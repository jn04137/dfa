use std::io::prelude::*;
use std::env;
use std::io::{BufReader, stdin};
use std::fs::File;
use std::collections::HashMap;

fn vec_format(text: &str) -> Vec<&str> {
    let string: &str = text.trim().trim_matches(|c| c== '}' || c=='{');
    let vector: Vec<&str> = string.split(',').collect(); 
    vector
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("\nFormat: cargo run <dfa.txt>\nPlease try again!");
    }
    let file = File::open(&args[1])?;

    let mut buf_reader = BufReader::new(file);
    
    // Gets the alphabet
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let _alphabet: Vec<&str> = vec_format(&text); 
    
    // Gets the states
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let _states: Vec<&str> = vec_format(&text);

    // Gets the start state
    let mut text = String::new();
    buf_reader.read_line(&mut text)?;
    let start_state: char = text.chars().nth(0).unwrap();
   
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
    
    let mut user_input = String::new();
    println!("Enter your string!");
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to readline");
    
    let mut curr_state = start_state;
    let mut func = String::new();
    for x in user_input.trim().chars(){
        func.push(curr_state);
        func.push(x);

        curr_state = *(transitive_functions.get(&func).unwrap());
        func.clear();
    }

    for x in accept_states {
        if x.chars().nth(0).unwrap() == curr_state {
            println!("This string is compatible with the given machine!"); 
            return Ok(());
        }
    }

    println!("This string is not compatible with the given machine!");
    Ok(())
}
