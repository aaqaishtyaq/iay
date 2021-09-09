use std::env;
use tico::tico;

pub fn cwd() -> String {
    let path = env::var("PWD").unwrap();
    let is_dir_short = env::var("SHIRTEN_DIR").unwrap_or("1".into());

    match is_dir_short.as_ref() {
        "0" => path,
        _ => tico(&path[..], Option::None)
    }
}
