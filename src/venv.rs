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
    let enabled = matches!(
        env::var("IAY_SHOW_NIX_SHELL").ok().as_deref(),
        Some("1" | "true" | "yes")
    );
    let color = env::var("IAY_NIX_SHELL_COLOR").unwrap_or_else(|_| "bright black".into());

    nix_shell_indicator(env::var("IN_NIX_SHELL").ok().as_deref(), enabled, &color)
}

fn nix_shell_indicator(nix_shell: Option<&str>, enabled: bool, color: &str) -> String {
    if enabled && nix_shell.is_some() {
        colors::colored_string("(nix) ", color, "")
    } else {
        colors::colored_string("", "white", "")
    }
}

#[cfg(test)]
mod tests {
    use super::nix_shell_indicator;
    use crate::colors;

    #[test]
    fn nix_indicator_is_hidden_by_default() {
        assert_eq!(
            nix_shell_indicator(Some("pure"), false, "bright black"),
            colors::colored_string("", "white", "")
        );
    }

    #[test]
    fn enabled_nix_indicator_is_grey_and_separates_the_prompt_character() {
        assert_eq!(
            nix_shell_indicator(Some("pure"), true, "bright black"),
            colors::colored_string("(nix) ", "bright black", "")
        );
    }
}
