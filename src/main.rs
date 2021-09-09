use std::env;
use tico::tico;

fn main() {
    print!("{}", cwd());
    println!("{}", prompt_char());
}

fn cwd() -> String {
    let path = env::var("PWD").unwrap();
    let is_dir_short = env::var("SHIRTEN_DIR").unwrap_or("1".into());

    match is_dir_short.as_ref() {
        "0" => path,
        _ => tico(&path[..], Option::None)
    }
}

fn prompt_char() -> String {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("% ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("# ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char,
        _ => user_char
    }
}
