/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use iay::colors;
use std::env;

pub fn prompt_char() -> String {
    let user_char = env::var("IAY_PROMPT_CHAR").unwrap_or_else(|_| "%".into());
    let root_char = env::var("IAY_PROMPT_CHAR_ROOT").unwrap_or_else(|_| "#".into());
    let user_char_color = env::var("IAY_PROMPT_CHAR_COLOR").unwrap_or_else(|_| "blue".into());
    let root_char_color = env::var("IAY_PROMPT_CHAR_ROOT_COLOR").unwrap_or_else(|_| "red".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => colors::colored_string(&root_char, &root_char_color, "bold"),
        _ => colors::colored_string(&user_char, &user_char_color, "bold"),
    }
}
