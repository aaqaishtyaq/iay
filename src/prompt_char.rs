use colored::*;
use std::env;

pub fn prompt_char() -> colored::ColoredString {
    let user_char = env::var("IAY_PROMPT_CHAR").unwrap_or("% ".into());
    let root_char = env::var("IAY_PROMPT_CHAR_ROOT").unwrap_or("# ".into());
    let user_char_color = env::var("IAY_PROMPT_CHAR_COLOR").unwrap_or("blue".into());
    let root_char_color = env::var("IAY_PROMPT_CHAR_ROOT_COLOR").unwrap_or("red".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char.color(root_char_color).bold(),
        _ => return user_char.color(user_char_color).bold(),
    }
}
