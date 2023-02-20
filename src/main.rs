use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::*;
use std::path::Path;
use clap::Parser;
use log::{error, info};
use log4rs;

use crate::command_types::*;
use crate::command_executor::*;

mod command_types;
mod command_executor;
mod screen_compare;

#[derive(Parser)]
#[command(author, version, about = "", long_about = None)]
struct Cli {
    #[arg(short='f', long="file_name", value_name = "file")]
    arg_file_name: String,

    #[arg(short='s', long="start_delay", value_name = "start_delay_ms", default_value_t=0)]
    #[arg(long_help="The program will wait this long before executing the commands. This is in milliseconds")]
    arg_start_delay: u64,

    #[arg(short='t', long="times_to_execute_commands", value_name = "times_to_execute_commands", default_value_t=1)]
    arg_times_to_execute_commands: u64
}

fn main() {

    // Make logs relative to the executable's directory
    let mut current_log_file_path = env::current_exe().unwrap();
    current_log_file_path.pop();

    env::set_current_dir(&current_log_file_path).expect("Cannot set current directory to the executable's directory");

    let log_initialization_result = log4rs::init_file("resources/log4rs.yaml", Default::default());
    match log_initialization_result {
        Ok(_) => {},
        Err(reason) => { println!("Failed to initialize logging: {}\nContinuing without logging", reason); }
    };
    info!(target: "commands_debug", "Reading arguments now");

    let args = Cli::parse();
    let file_name = args.arg_file_name;

    if args.arg_start_delay > 0 {
        let start_delay: Duration = Duration::from_millis(args.arg_start_delay);
        std::thread::sleep(start_delay);
    }

    let mut command_sequence: Vec<ParsedCommand> = vec![];
    read_input_file(&file_name, &mut command_sequence);

    for _i in 0.. args.arg_times_to_execute_commands {
        execute_commands(&command_sequence);
    }
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
            error!(target: "commands_debug", "The input file {} could not be read: {}", full_name.display(), error_reason);
            panic!("The input file {} could not be read: {}", full_name.display(), error_reason);
        } 
    }
}