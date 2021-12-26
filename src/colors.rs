/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
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

    ansi_style.paint(string).to_string()
}

pub fn ansi_colour(color: &str) -> ansi_term::Colour {
    match color {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bright_red() {
        assert_eq!(ansi_colour("bright red"), Colour::Fixed(9))
    }

    #[test]
    fn test_fallback() {
        assert_eq!(ansi_colour(""), Colour::Fixed(7))
    }

    #[test]
    fn test_blue() {
        assert_eq!(ansi_colour("blue"), Colour::Fixed(4))
    }
}
