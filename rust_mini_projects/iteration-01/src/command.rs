use crate::{Interpreter, MEMORY_SIZE, helpers};
///definition of commands
pub enum Command {
    ToRight,
    ToLeft,
    Increment,
    Decrement,
    CharToString,
    SaveChar,
    JumpRight,
    JumpLeft,
}
/// Implementation from the exercise - using Commands to work with cells
impl Command {
    pub fn call_command(&self, interpreter: &mut Interpreter) {
        match self {
            Command::ToRight => {
                //Move pointer to the right
                //It works the same way in reverse with the last cell and the move right operation.
                interpreter.pointer_bunks += 1;
                interpreter.pointer_bunks %= MEMORY_SIZE;
                interpreter.command_position += 1;
            }
            Command::ToLeft => {
                //If the pointer points to the first cell (index 0) and a move left operation occurs,
                //the pointer will now point to the last cell (index 29999).
                if interpreter.pointer_bunks == 0 {
                    interpreter.pointer_bunks = MEMORY_SIZE - 1;
                } else {
                    interpreter.pointer_bunks -= 1;
                }
                interpreter.command_position += 1;
            }
            Command::Increment => {
                //If the cell has a value of 255 and an increment occurs, the new cell value will be 0 - this is ensured by my wrapping_add
                interpreter.bunks[interpreter.pointer_bunks] =
                    interpreter.bunks[interpreter.pointer_bunks].wrapping_add(1);
                interpreter.command_position += 1;
            }
            Command::Decrement => {
                //It works the same way in reverse with the last cell and the move right operation - wrapping_sub has a similar function
                interpreter.bunks[interpreter.pointer_bunks] =
                    interpreter.bunks[interpreter.pointer_bunks].wrapping_sub(1);
                interpreter.command_position += 1;
            }
            Command::CharToString => {
                //Print the character represented by the cell under the pointer
                let char = interpreter.bunks[interpreter.pointer_bunks] as char;
                print!("{}", char);
                interpreter.command_position += 1;
            }
            Command::SaveChar => {
                //Read a character and save it to the cell under the pointer
                let byte = helpers::read_byte_from_stdin();
                interpreter.bunks[interpreter.pointer_bunks] = byte;
                interpreter.command_position += 1;
            }
            Command::JumpRight => {
                //Jump from [ past the corresponding ] if the cell under the pointer equals 0
                if interpreter.bunks[interpreter.pointer_bunks] == 0
                    && interpreter
                        .map_of_jumps
                        .contains_key(&interpreter.command_position)
                {
                    interpreter.command_position =
                        interpreter.map_of_jumps[&interpreter.command_position];
                }
                interpreter.command_position += 1;
            }
            Command::JumpLeft => {
                //Jump from ] to the corresponding [ if the cell under the pointer is not equal to 0
                if interpreter.bunks[interpreter.pointer_bunks] != 0
                    && interpreter
                        .map_of_jumps
                        .contains_key(&interpreter.command_position)
                {
                    interpreter.command_position =
                        interpreter.map_of_jumps[&interpreter.command_position];
                }
                interpreter.command_position += 1;
            }
        }
    }
}

///Here I parse all words into commands later
pub fn parse_commands(words: Vec<String>) -> Vec<Command> {
    words
        .iter()
        .filter_map(|word| word_to_command(word))
        .collect()
}
///Hand in hand parse_commands and word_to_command - I'm looking for keywords and then returning individual enums to parse_commands for better understanding of the program flow :)
pub fn word_to_command(word: &str) -> Option<Command> {
    match word {
        "time" | "be" | "last" | "of" | "and" | "not" | "the" | "it" | "yeah" | "will" => {
            Some(Command::ToRight)
        }

        "year" | "have" | "other" | "in" | "that" | "out" | "a" | "i" | "no" | "would" => {
            Some(Command::ToLeft)
        }
        "people" | "do" | "new" | "to" | "but" | "up" | "this" | "you" | "yes" | "can" => {
            Some(Command::Increment)
        }
        "way" | "say" | "good" | "for" | "or" | "so" | "his" | "he" | "well" | "could" => {
            Some(Command::Decrement)
        }
        "man" | "go" | "old" | "on" | "as" | "then" | "which" | "they" | "aye" | "should" => {
            Some(Command::CharToString)
        }
        "day" | "get" | "great" | "with" | "if" | "more" | "an" | "she" | "hello" | "may" => {
            Some(Command::SaveChar)
        }
        "thing" | "make" | "high" | "at" | "than" | "now" | "their" | "we" | "ha" | "must" => {
            Some(Command::JumpRight)
        }
        "child" | "see" | "small" | "by" | "when" | "just" | "what" | "there" | "dear"
        | "might" => Some(Command::JumpLeft),
        _ => None,
    }
}
