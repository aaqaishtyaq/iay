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
    nix_shell_indicator(env::var("IN_NIX_SHELL").ok().as_deref())
}

fn nix_shell_indicator(nix_shell: Option<&str>) -> String {
    match nix_shell {
        Some("pure") => colors::colored_string("(nix) ", "green", ""),
        Some(_) => colors::colored_string("(nix) ", "red", ""),
        None => colors::colored_string("", "white", ""),
    }
}

#[cfg(test)]
mod tests {
    use super::nix_shell_indicator;
    use crate::colors;

    #[test]
    fn nix_indicator_separates_the_prompt_character() {
        assert_eq!(
            nix_shell_indicator(Some("pure")),
            colors::colored_string("(nix) ", "green", "")
        );
    }
}
