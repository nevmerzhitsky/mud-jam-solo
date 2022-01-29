use std::io;

use crate::area::World;

pub trait CharAction {
    fn execute(&self, w: &World, subject_id: u32);
}

#[derive(Debug)]
pub struct MoveNorth {}

#[derive(Debug)]
pub struct MoveSouth {}

impl CharAction for MoveNorth {
    fn execute(&self, _w: &World, subject_id: u32) {
        println!("#{:?} moving to the north...", subject_id);
    }
}

impl CharAction for MoveSouth {
    fn execute(&self, _w: &World, subject_id: u32) {
        println!("#{:?} moving to the south...", subject_id);
    }
}

pub fn ask_command() -> Box<dyn CharAction> {
    println!("Your action? ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    command_to_action(input)
}

fn command_to_action(input: String) -> Box<dyn CharAction> {
    let mut words = input.split_whitespace();
    let command = words.next().unwrap_or("").to_ascii_lowercase();
    let params: Vec<&str> = words.collect();

    println!(">>> Command: \"{}\" {:?}", command, params);

    // TODO Use clap library as the input parser to all available commands
    // TODO Return Command instance: input + action instance
    match command.as_str() {
        "quit" => panic!("You decided to quit"),
        "north" => Box::new(MoveNorth {}),
        "south" => Box::new(MoveSouth {}),
        _ => panic!("Unknown command: {}", command),
    }
}
