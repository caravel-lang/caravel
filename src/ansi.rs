pub const RESET: &'static str = "\x1b[0m";
pub const BOLD: &'static str = "\x1b[1m";
pub const RED: &'static str = "\x1b[31m";

pub fn style(str: &str, escape_code: &'static str) -> String {
  format!("{}{}{}", escape_code, str, RESET)
}
