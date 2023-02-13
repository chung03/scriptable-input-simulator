use enigo::*;
use std::env;
use std::fs::File;
use std::io::*;
use std::path::Path;
use crate::command_types::*;

mod command_types;

fn main() {
    println!("Reading arguments now");

    let args: Vec<String> = env::args().collect();

    if !validate_args(&args) {
        std::process::exit(1);
    }

    let file_name = parse_args(args);

    let mut command_sequence: Vec<ParsedCommand> = vec![];

    read_input_file(&file_name, &mut command_sequence);
    execute_commands(command_sequence);
}

// true means the arguments are valid. Otherwise, the arguments are invalid and the program should stop
fn validate_args(args: &Vec<String>) -> bool {

    let num_args = args.len();
    if num_args != 2 {
        println!("There should be exactly 2 arguments, but instead there were {}", num_args);
        return false;
    }

    return true;
}

fn parse_args(args: Vec<String>) -> String{
    for argument in args.iter() {
        println!("Read argument: {}", argument);
    }

    return args[1].clone();
}

fn read_input_file(file_name: &String, command_vector: &mut Vec<ParsedCommand>) {
    let full_name: &Path = Path::new(file_name);
    let file: Result<File> = File::open(full_name);

    match file
    {
        Ok(file) => {
            let buf_reader = BufReader::new(file).lines();
            for line in buf_reader {
                if let Ok(command) = line {
                    if !command.is_empty() {
                        command_vector.push(parse_command_from_line(&command));
                    }
                }
            }
        },
        Err(error_reason) => {
            panic!("The input file {} could not be read: {}", full_name.display(), error_reason);
        } 
    }
}

fn execute_commands(command_vector: Vec<ParsedCommand>) {
    let mut enigo = Enigo::new();

    for parsed_command in command_vector{
        match parsed_command{
            ParsedCommand::LayoutKeyUse(key, button_action) => { 
                match button_action {
                    ButtonAction::Press => {
                        enigo.key_down(Key::Layout(key));
                    },
                    ButtonAction::Release => {
                        enigo.key_up(Key::Layout(key));
                    },
                    ButtonAction::None => { println!("This should not happen!"); }
                }
             },
            ParsedCommand::SpecialKeyUse(key, button_action) => { println!("SpecialKeyUse unimplemented"); },
            ParsedCommand::KeySequence(sequence) => {
                enigo.key_sequence(sequence.as_str());
            },
            ParsedCommand::MouseClick(mouse_button) => {
                enigo.mouse_click(mouse_button);
            },
            ParsedCommand::MouseDown(mouse_button) => {
                enigo.mouse_down(mouse_button);
            },
            ParsedCommand::MouseRelease(mouse_button) => {
                enigo.mouse_up(mouse_button);
            },
            ParsedCommand::MouseMove{x, y} => {
                enigo.mouse_move_to(x, y);
            },
            ParsedCommand::Wait(wait_time_ms) => {
                let wait_duration = std::time::Duration::from_millis(wait_time_ms);
                std::thread::sleep(wait_duration);
            }
        }
    }
}