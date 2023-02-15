use enigo::*;
use std::time::Duration;
use std::fs::File;
use std::io::*;
use std::path::Path;
use clap::Parser;
use crate::command_types::*;

mod command_types;


#[derive(Parser)]
#[command(author, version, about = "", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "file")]
    file_name: String,

    #[arg(short, long, value_name = "start_delay_ms")]
    start_delay: Option<String>
}

fn main() {
    println!("Reading arguments now");

    let args = Cli::parse();
    let file_name = args.file_name;

    if let Some(start_delay) = args.start_delay {
        if let Ok(parsed_time) = start_delay.parse::<u64>(){
            let wait_before_start_ms: Duration = Duration::from_millis(parsed_time);
            std::thread::sleep(wait_before_start_ms);
        }
    }

    let mut command_sequence: Vec<ParsedCommand> = vec![];

    read_input_file(&file_name, &mut command_sequence);
    execute_commands(command_sequence);
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
                    ButtonAction::Click => {
                        enigo.key_click(Key::Layout(key));
                    },
                    ButtonAction::None => { println!("This should not happen! Doing nothing"); }
                }
             },
            ParsedCommand::SpecialKeyUse(key, button_action) => { 
                match button_action {
                    ButtonAction::Press => {
                        enigo.key_down(key);
                    },
                    ButtonAction::Release => {
                        enigo.key_up(key);
                    },
                    ButtonAction::Click => {
                        enigo.key_click(key);
                    },
                    ButtonAction::None => { println!("This should not happen! Doing nothing"); }
                } 
            },
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