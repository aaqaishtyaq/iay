use std::env;

pub fn prompt_char() -> String {
    let user_char = env::var("PROMPT_CHAR").unwrap_or(" % ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or(" # ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char,
        _ => user_char
    }
}
