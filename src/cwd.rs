/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use iay::colors;
use std::env;
use tico::tico;

fn home_dir() -> Option<String> {
    if let Ok(home) = env::var("HOME") {
        if !home.is_empty() {
            return Some(home);
        }
    }

    // $HOME isn't always set (e.g. minimal containers, `env -i`), so fall
    // back to the conventional Linux home path.
    if cfg!(target_os = "linux") {
        if let Ok(user) = env::var("USER") {
            if !user.is_empty() {
                return Some(format!("/home/{}", user));
            }
        }
    }

    None
}

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let home = home_dir();
    let tilde_expand = env::var("IAY_EXPAND_TILDE").unwrap_or_else(|_| "0".into());

    let in_home = home.as_deref().is_some_and(|h| path.contains(h));

    let cwd_color = if in_home {
        env::var("IAY_CWD_HOME_COLOR").unwrap_or_else(|_| "bright red".into())
    } else {
        env::var("IAY_CWD_ROOT_COLOR").unwrap_or_else(|_| "bright cyan".into())
    };

    if let ("0", Some(home)) = (tilde_expand.as_ref(), home.as_deref()) {
        let home_ext = format!("{}{}", home, "/");
        if (path == home) || path.starts_with(&home_ext) {
            path = path.replacen(home, "~", 1);
        }
    };

    let cwd_shorten = env::var("IAY_SHORTEN_CWD").unwrap_or_else(|_| "1".into());
    match cwd_shorten.as_ref() {
        "0" => Some(colors::colored_string(&path, &cwd_color, "bold")),
        _ => Some(colors::colored_string(
            &tico(&path, home.as_deref()),
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
        env::set_var("IAY_CWD_ROOT_COLOR", "bright red");

        assert_eq!(
            cwd(),
            Some(colors::colored_string(&path, "bright red", "bold"))
        )
    }
}
