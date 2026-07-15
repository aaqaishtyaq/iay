/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
mod cwd;
mod prompt_char;
mod vcs;
mod venv;

use clap::Parser;
use iay::colors;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use minimal variant
    #[clap(short, long)]
    minimal: bool,

    /// Use ZSH formatting
    #[clap(short, long)]
    zsh: bool,
}

fn main() {
    let cmd = Args::parse();

    if cmd.minimal {
        println!("{}", iay_prompt_minimal(cmd.zsh));
    } else {
        println!("{}", iay_prompt(cmd.zsh));
    }
}

fn iay_prompt(zsh: bool) -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => colors::colored_string("[directory does not exist]", "red", ""),
    };

    let vcs_status = vcs::status();
    let venv = venv::get_name();
    let prompt_char = prompt_char::prompt_char(zsh);

    let prompt = if zsh {
        format!(
            "{cwd}{vcs}\n{venv}{nix}{pchar} ",
            cwd = cwd,
            vcs = vcs_status,
            venv = venv,
            nix = venv::in_nix_shell(),
            pchar = prompt_char
        )
    } else {
        format!(
            "{cwd}{vcs}\n{venv}{nix}{pchar} ",
            cwd = cwd,
            vcs = vcs_status,
            venv = venv,
            pchar = prompt_char,
            nix = venv::in_nix_shell()
        )
    };

    escape_non_printing(&prompt, zsh)
}

fn iay_prompt_minimal(zsh: bool) -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => colors::colored_string("[directory does not exist]", "red", " "),
    };

    let vcs_status = vcs::status();

    let venv = venv::get_name();
    let prompt_char = prompt_char::prompt_char(zsh);

    let prompt = format!(
        "{cwd}{vcs}{venv}{nix}{pchar} ",
        cwd = cwd,
        vcs = vcs_status,
        venv = venv,
        nix = venv::in_nix_shell(),
        pchar = prompt_char
    );

    escape_non_printing(&prompt, zsh)
}

/// Shell line editors must be told that ANSI SGR escape sequences take no
/// display width. Zsh uses `%{...%}` and Bash/readline uses `\[...\]`.
fn escape_non_printing(prompt: &str, zsh: bool) -> String {
    let (open, close) = if zsh { ("%{", "%}") } else { ("\\[", "\\]") };
    let mut escaped = String::with_capacity(prompt.len());
    let mut remainder = prompt;

    while let Some(start) = remainder.find('\x1b') {
        escaped.push_str(&remainder[..start]);
        let sequence = &remainder[start..];
        let Some(end) = sequence.find('m') else {
            escaped.push_str(sequence);
            return escaped;
        };

        escaped.push_str(open);
        escaped.push_str(&sequence[..=end]);
        escaped.push_str(close);
        remainder = &sequence[end + 1..];
    }

    escaped.push_str(remainder);
    escaped
}

#[cfg(test)]
mod tests {
    use super::escape_non_printing;

    #[test]
    fn escapes_ansi_for_bash() {
        assert_eq!(
            escape_non_printing("\x1b[1;31mred\x1b[0m", false),
            "\\[\x1b[1;31m\\]red\\[\x1b[0m\\]"
        );
    }

    #[test]
    fn escapes_ansi_for_zsh() {
        assert_eq!(
            escape_non_printing("\x1b[1;31mred\x1b[0m", true),
            "%{\x1b[1;31m%}red%{\x1b[0m%}"
        );
    }
}
