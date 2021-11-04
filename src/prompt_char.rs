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

pub fn prompt_char() -> String {
    let user_char = env::var("IAY_PROMPT_CHAR").unwrap_or("% ".into());
    let root_char = env::var("IAY_PROMPT_CHAR_ROOT").unwrap_or("# ".into());
    let user_char_color = env::var("IAY_PROMPT_CHAR_COLOR").unwrap_or("blue".into());
    let root_char_color = env::var("IAY_PROMPT_CHAR_ROOT_COLOR").unwrap_or("red".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return colors::colored_string(&root_char, &root_char_color, "bold"),
        _ => return colors::colored_string(&user_char, &user_char_color, "bold"),
    }
}
