use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

use crate::{
    command::{Command, parse_commands},
    helpers::read_first_argument,
};

mod command;
mod helpers;

const MEMORY_SIZE: usize = 30000;

///Simple interpreter struct, holds the cells themselves, current cell pointer, jump map and which command we are currently processing (for loops JumpRight, JumpLeft)
struct Interpreter {
    bunks: [u8; MEMORY_SIZE], //each cell is 8 bits, 2^8=256 - so when it overflows I can call wrapping which resets it...it's like that in the documentation...hopefully..
    pointer_bunks: usize,     //current cell
    map_of_jumps: HashMap<usize, usize>, //storing edges of a directed graph
    command_position: usize,  //added variable to track which command I'm on - purely for loops
}
///Added based on feedback from teacher
/// Here I am creating interpreter struct directly in new function ::new()
impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            bunks: [0; MEMORY_SIZE],
            pointer_bunks: 0,
            map_of_jumps: HashMap::new(),
            command_position: 0,
        }
    }
}

/// Parse words and clean them from dots, transform to lowercase and separate individually by spaces
///Any two words in Zipffuck code can be separated by one or more spaces or newlines.
/// Any word in Zipffuck code can be followed by a punctuation mark - the . character
fn parse_words(code: &str) -> Vec<String> {
    let iterator = code.split_whitespace();
    let mut words: Vec<String> = Vec::new();
    for word in iterator {
        let sanitized_word = word.trim_end_matches(['.', '\n']).to_lowercase();
        words.push(sanitized_word.to_string());
    }
    words
}

// Do not modify the body of the `main` function, as it is required for tests.
fn main() -> io::Result<()> {
    let input_file = read_first_argument();
    let input = read_to_string(input_file)?;

    match interpret_zipffuck(input) {
        true => Ok(()),
        false => Err(std::io::Error::other(
            "Zipffuck interpreter returned false.".to_string(),
        )),
    }
}

fn interpret_zipffuck(_code: String) -> bool {
    let sanitized_text = parse_words(&_code);
    let commands = parse_commands(sanitized_text);

    let mut interpreter = Interpreter::new();
    ///////loop checker + initializer
    if !init_loops(&mut interpreter, &commands) {
        return false;
    }

    //I have to use while, in for i I apparently can't change the iterator, hah
    while interpreter.command_position < commands.iter().len() {
        commands[interpreter.command_position].call_command(&mut interpreter);
    }

    true
}
///Loop initialization - I read everything and look for brackets, then I store in the hashmap where I can go from and to -
/// it's a strongly directed graph - so I'm adding paths both ways
fn init_loops(interpreter: &mut Interpreter, command: &[Command]) -> bool {
    let mut stack: Vec<usize> = Vec::new();
    for (i, item) in command.iter().enumerate() {
        match item {
            Command::JumpRight => {
                stack.push(i);
            }
            Command::JumpLeft => {
                if let Some(position) = stack.pop() {
                    interpreter.map_of_jumps.insert(position, i);
                    interpreter.map_of_jumps.insert(i, position);
                } else {
                    //this false is for the loop_flipped test case - when brackets are reversed and we have nothing to pop
                    return false;
                }
            }
            _ => {
                //I ignore other commands, I'm only interested in JumpRight and JumpLeft in this function, the rest goes to default and I continue with the for loop
            }
        }
    }
    ////if it's empty, nice, if it's full, very bad! (we have unpaired loops!!)
    stack.is_empty()
}
