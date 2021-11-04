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
use std::path::Path;

pub fn get_name() -> String {
    match env::var("IAY_VIRTUAL_ENV") {
        Ok(venv_path) => {
            let venv_name = Path::new(&venv_path[..]).file_name();
            if let Some(name) = venv_name {
                if let Some(valid_name) = name.to_str() {
                    return colors::colored_string(
                        &format!("({})", valid_name),
                        "bright black",
                        "",
                    );
                }
            }
        }
        Err(_) => {}
    }
    return colors::colored_string("", "white", "");
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
        _ => return colors::colored_string("", "white", ""),
    }
}
