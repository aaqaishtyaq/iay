/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use iay::colors;
use std::env;
use tico::tico;

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("IAY_EXPAND_TILDE").unwrap_or("0".into());

    match tilde_expand.as_ref() {
        "0" => {
            let home_dir = &home.clone();
            let home_dir_ext = format!("{}{}", home_dir, "/");
            if (&path == home_dir) || *(&path.starts_with(&home_dir_ext)) {
                path = path.replacen(&home_dir[..], "~", 1);
            }
        }
        _ => {}
    };

    let cwd_shorten = env::var("IAY_SHORTEN_CWD").unwrap_or("1".into());
    let cwd_color = env::var("IAY_CWD_COLOR").unwrap_or("bright blue".into());
    match cwd_shorten.as_ref() {
        "0" => return Some(colors::colored_string(&path, &cwd_color, "bold")),
        _ => {
            return Some(colors::colored_string(
                &tico(&path, Option::None),
                &cwd_color,
                "bold",
            ))
        }
    }
}
