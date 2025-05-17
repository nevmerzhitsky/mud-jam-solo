use std::io;

use crate::area::World;

pub trait CharAction {
    fn execute(&self, w: &World, subject_id: u32);
}

#[derive(Debug)]
pub struct UnknownCommand {}

#[derive(Debug)]
pub struct Quit {}

#[derive(Debug)]
pub struct MoveNorth {}

#[derive(Debug)]
pub struct MoveSouth {}

#[derive(Debug)]
pub struct Say {
    params: Vec<String>,
}

impl CharAction for Quit {
    fn execute(&self, _w: &World, _subject_id: u32) {
        panic!("You decided to quit")
    }
}

impl CharAction for UnknownCommand {
    fn execute(&self, _w: &World, _subject_id: u32) {
        println!("Unknown command")
    }
}

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

impl CharAction for Say {
    fn execute(&self, _w: &World, _subject_id: u32) {
        println!("SAY: {:?}", self.params);
    }
}

pub fn ask_command_as_action() -> Box<dyn CharAction> {
    println!("Your action? ");

    let mut input = String::with_capacity(200);
    input.clear();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("> {n} bytes read: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    command_to_action(input)
}

fn command_to_action(input: String) -> Box<dyn CharAction> {
    let mut words = input.split_whitespace();
    let command = words.next().unwrap_or("").to_ascii_lowercase();
    let params: Vec<String> = words.map(|m| m.to_string()).collect();

    println!(">>> Command: \"{}\" {:?}", command, params);

    // TODO Use clap library as the input parser to all available commands
    // TODO Return Command instance: input + action instance
    match command.as_str() {
        "quit" => Box::new(Quit {}),
        "north" => Box::new(MoveNorth {}),
        "south" => Box::new(MoveSouth {}),
        "say" => Box::new(Say { params }),
        _ => Box::new(UnknownCommand {}),
    }
}
