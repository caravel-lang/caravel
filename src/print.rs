use super::position::Position;
use colored::*;

pub enum Status {
    Ok,
    Error,
    Warning,
    Info,
}

pub fn print_message(msg: &str, status: Status) {
    let status_msg = match status {
        Status::Ok => "[Ok]".bright_green(),
        Status::Error => "[Error]".bright_red(),
        Status::Warning => "[Warning]".bright_yellow(),
        Status::Info => "[Info]".bright_blue(),
    };

    println!("{:9} {}", status_msg, msg);
}

fn print_code_ln(line_no: i32, line: &str, secondary: bool) {
    if secondary {
        println!(
            "{:>4} {}",
            (line_no + 1).to_string().bright_black(),
            line.bright_black()
        )
    } else {
        println!("{:>4} {}", (line_no + 1).to_string().bright_black(), line)
    };
}

pub fn print_message_with_context(
    msg: &str,
    status: Status,
    pos: Position,
    length: i32,
    input: &str,
) {
    let lines: Vec<&str> = input.split('\n').collect();

    // Index of the correct line
    let mut index_of_line = 0;
    // Number of lines processed thus far
    let mut line_count = 0;

    for (index, char) in input.chars().enumerate() {
        if char == '\n' {
            line_count += 1;
        }

        if line_count == pos.get_row() {
            index_of_line = index as i32;
            break;
        }
    }

    // How many chars into line is the position
    let line_offset = pos.get_index() - index_of_line;

    print_message(msg, status);

    if pos.get_row() - 1 >= 0 {
        print_code_ln(pos.get_row() - 1, lines[pos.get_row() as usize - 1], true);
    }

    print_code_ln(pos.get_row(), lines[pos.get_row() as usize], false);
    println!(
        "     {}{}",
        " ".repeat(line_offset as usize),
        "^".repeat(length as usize).cyan()
    );

    if pos.get_row() + 1 < lines.len() as i32 {
        print_code_ln(pos.get_row() + 1, lines[pos.get_row() as usize + 1], true);
    }
}
