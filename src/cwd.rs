/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use iay::colors;
use std::env;
use tico::tico;

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("IAY_EXPAND_TILDE").unwrap_or_else(|_| "0".into());

    let cwd_color = if path.contains(&home) {
        env::var("IAY_CWD_HOME_COLOR").unwrap_or_else(|_| "bright red".into())
    } else {
        env::var("IAY_CWD_ROOT_COLOR").unwrap_or_else(|_| "bright cyan".into())
    };

    if let "0" = tilde_expand.as_ref() {
        let home_dir = &home;
        let home_dir_ext = format!("{}{}", home_dir, "/");
        if (&path == home_dir) || path.starts_with(&home_dir_ext) {
            path = path.replacen(&home_dir[..], "~", 1);
        }
    };

    let cwd_shorten = env::var("IAY_SHORTEN_CWD").unwrap_or_else(|_| "1".into());
    match cwd_shorten.as_ref() {
        "0" => Some(colors::colored_string(&path, &cwd_color, "bold")),
        _ => Some(colors::colored_string(
            &tico(&path, Option::None),
            &cwd_color,
            "bold",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cwd() {
        let path = format!("{}", env::current_dir().unwrap().display());

        env::set_var("IAY_SHORTEN_CWD", "0");
        env::set_var("IAY_EXPAND_TILDE", "1");
        env::set_var("IAY_CWD_HOME_COLOR", "bright red");

        assert_eq!(
            cwd(),
            Some(colors::colored_string(&path, "bright red", "bold"))
        )
    }
}
