use enigo::*;

pub enum ButtonAction {
    Press,
    Release,
    None
}
pub enum ParsedCommand {
    LayoutKeyUse(char, ButtonAction),
    SpecialKeyUse(enigo::Key, ButtonAction),
    KeySequence(String),
    MouseClick(MouseButton),
    MouseDown(MouseButton),
    MouseRelease(MouseButton),
    MouseMove{x: i32, y: i32},
    Wait(u64)
}

pub fn parse_command_from_line(line: &String) -> ParsedCommand{
    if line.starts_with("key_sequence: ") {
        println!("parse_command_from_line: read key_sequence");

        let split_line: Vec<&str> = line.split("key_sequence: ").collect();
        
        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            return ParsedCommand::KeySequence(String::from(split_line[1]));
        } 
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    if line.starts_with("layout_key: ") {
        println!("parse_command_from_line: read layout_key");

        let split_line: Vec<&str> = line.split("layout_key: ").collect();
        
        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let split_line_key_and_action: Vec<&str> = split_line[1].split(" ").collect();

            if split_line_key_and_action.len() == 2 {
                let parse_result_char = split_line_key_and_action[0].parse::<char>();
                let parsed_button_action = split_line_key_and_action[1];

                let mut button_action: ButtonAction = ButtonAction::None;
                if parsed_button_action == "release" {
                    button_action = ButtonAction::Release;
                }
                else if parsed_button_action == "press" {
                    button_action = ButtonAction::Press;
                }

                if let Ok(key) = parse_result_char {
                    match button_action
                    {
                        ButtonAction::None => { println!("The line is not formatted properly, not using it as a command"); },
                        _ => { return ParsedCommand::LayoutKeyUse(key, button_action);}
                    }
                }
                else {
                    println!("The line is not formatted properly, not using it as a command");
                }
            }
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        } 
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    else if line.starts_with("wait: ") {
        println!("parse_command_from_line: wait");
        let split_line: Vec<&str> = line.split("wait: ").collect();

        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let parse_result = split_line[1].parse::<u64>();

            if let Ok(wait_time) = parse_result {
                return ParsedCommand::Wait(wait_time);
            } 
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        }
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    else if line.starts_with("mouse_click: ") {
        println!("parse_command_from_line: mouse_click");
        let split_line: Vec<&str> = line.split("mouse_click: ").collect();

        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let button_specifier = String::from(split_line[1]);
            if button_specifier == "left" {
                return ParsedCommand::MouseClick(MouseButton::Left);
            }
            if button_specifier == "right" {
                return ParsedCommand::MouseClick(MouseButton::Right);
            }
            if button_specifier == "middle" {
                return ParsedCommand::MouseClick(MouseButton::Middle);
            }
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        }
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    else if line.starts_with("mouse_down: ") {
        println!("parse_command_from_line: mouse_down");
        let split_line: Vec<&str> = line.split("mouse_down: ").collect();

        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let button_specifier = String::from(split_line[1]);
            if button_specifier == "left" {
                return ParsedCommand::MouseDown(MouseButton::Left);
            }
            if button_specifier == "right" {
                return ParsedCommand::MouseDown(MouseButton::Right);
            }
            if button_specifier == "middle" {
                return ParsedCommand::MouseDown(MouseButton::Middle);
            }
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        }
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    else if line.starts_with("mouse_release: ") {
        println!("parse_command_from_line: mouse_release");
        let split_line: Vec<&str> = line.split("mouse_release: ").collect();

        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let button_specifier = String::from(split_line[1]);
            if button_specifier == "left" {
                return ParsedCommand::MouseRelease(MouseButton::Left);
            }
            if button_specifier == "right" {
                return ParsedCommand::MouseRelease(MouseButton::Right);
            }
            if button_specifier == "middle" {
                return ParsedCommand::MouseRelease(MouseButton::Middle);
            }
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        }
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }
    else if line.starts_with("mouse_move: ") {
        println!("parse_command_from_line: mouse_move");
        let split_line: Vec<&str> = line.split("mouse_move: ").collect();

        for split in &split_line {
            println!("{}", split);
        }

        if split_line.len() == 2 {
            let split_line_coordinates: Vec<&str> = split_line[1].split(" ").collect();

            if split_line_coordinates.len() == 2 {
                let parse_result_x = split_line_coordinates[0].parse::<i32>();
                let parse_result_y = split_line_coordinates[1].parse::<i32>();

                if let (Ok(x), Ok(y)) = (parse_result_x, parse_result_y) {
                    return ParsedCommand::MouseMove{x, y};
                }
                else {
                    println!("The line is not formatted properly, not using it as a command");
                }
            }
            else {
                println!("The line is not formatted properly, not using it as a command");
            }
        }
        else {
            println!("The line is not formatted properly, not using it as a command");
        }
    }

    return ParsedCommand::Wait(1);
}