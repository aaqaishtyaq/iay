use std::env;
use colored::*;
use std::path::Path;

pub fn get_name() -> colored::ColoredString {
    match env::var("IAY_VIRTUAL_ENV") {
        Ok(venv_path) => {
            let venv_name = Path::new(&venv_path[..]).file_name();
            if let Some(name) = venv_name {
                if let Some(valid_name) = name.to_str() {
                    return format!("({})", valid_name).bright_black();
                }
            }
        }
        Err(_) => {}
    }
    return "".white()
}

pub fn in_nix_shell() -> colored::ColoredString {
    match env::var("IN_NIX_SHELL") {
        Ok(p) => {
            if p == "pure" {
                "(nix) ".green()
            } else {
                "(nix) ".red()
            }
        }
        _ => return "".white(),
    }
}

