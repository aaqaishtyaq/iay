mod cwd;
mod prompt_char;
mod vcs;

fn main() {
    print!("{}", cwd::cwd());
    match vcs::vcs() {
        Some(br) => print!(" {}", br),
        None => print!("")
    }
    println!("{}", prompt_char::prompt_char());
}

