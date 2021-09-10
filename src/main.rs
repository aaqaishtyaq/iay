mod cwd;
mod prompt_char;
mod vcs;

fn main() {
    // println!("{:?}", env::var("GIT_DIRTY"));
    print!("{}", cwd::cwd());
    let (branch, status) = match vcs::status() {
        Some((x, y)) => {
            (x, y)
        },
        None => ("".into(), "".into())
    };
    print!(" {} {}", branch, status);
    println!("{}", prompt_char::prompt_char());
}
