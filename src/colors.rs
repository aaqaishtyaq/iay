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
use ansi_term::Colour;

pub fn colored_string(string: &str, color: &str, style: &str) -> String {
    let ansi_color = ansi_colour(color);

    let ansi_style = match style {
        "bold" => ansi_color.bold(),
        "dimmed" => ansi_color.dimmed(),
        "italic" => ansi_color.italic(),
        _ => ansi_color.normal(),
    };

    return ansi_style.paint(string).to_string();
}

pub fn ansi_colour(color: &str) -> ansi_term::Colour {
    return match color {
        "black" => Colour::Fixed(0),
        "red" => Colour::Fixed(1),
        "green" => Colour::Fixed(2),
        "yellow" => Colour::Fixed(3),
        "blue" => Colour::Fixed(4),
        "purple" => Colour::Fixed(5),
        "cyan" => Colour::Fixed(6),
        "white" => Colour::Fixed(7),
        "bright black" => Colour::Fixed(8),
        "bright red" => Colour::Fixed(9),
        "bright green" => Colour::Fixed(10),
        "bright yellow" => Colour::Fixed(11),
        "bright blue" => Colour::Fixed(12),
        "bright purple" => Colour::Fixed(13),
        "bright cyan" => Colour::Fixed(14),
        "bright white" => Colour::Fixed(15),
        // Colors to support legacy colored colors
        "magenta" => Colour::Fixed(5),
        "bright magenta" => Colour::Fixed(13),
        // White color as fallback
        _ => Colour::Fixed(7),
    };
}
