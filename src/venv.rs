/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use iay::colors;
use std::env;
use std::path::Path;

pub fn get_name() -> String {
    if let Ok(venv_path) = env::var("IAY_VIRTUAL_ENV") {
        let venv_name = Path::new(&venv_path[..]).file_name();
        if let Some(name) = venv_name {
            if let Some(valid_name) = name.to_str() {
                return colors::colored_string(&format!("({})", valid_name), "bright black", "");
            }
        }
    }

    colors::colored_string("", "white", "")
}

pub fn in_nix_shell() -> String {
    match env::var("IN_NIX_SHELL") {
        Ok(p) => {
            if p == "pure" {
                colors::colored_string("(nix)", "green", "")
            } else {
                colors::colored_string("(nix)", "red", "")
            }
        }
        _ => colors::colored_string("", "white", ""),
    }
}
