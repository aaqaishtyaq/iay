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

    if zsh {
        format!(
            "%{{{cwd}{vcs}%}} %{{\n{venv}{pchar}%}} ",
            cwd = cwd,
            vcs = vcs_status,
            venv = venv,
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
    }
}

fn iay_prompt_minimal(zsh: bool) -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => colors::colored_string("[directory does not exist]", "red", " "),
    };

    let vcs_status = vcs::status();

    let venv = venv::get_name();
    let prompt_char = prompt_char::prompt_char(zsh);

    if zsh {
        let fmt = format!(
            "{cwd}{vcs}{venv}{pchar} ",
            cwd = cwd,
            vcs = vcs_status,
            venv = venv,
            pchar = prompt_char
        );
        let mut ret = String::new();
        let mut color = false;
        for ch in fmt.chars() {
            if color {
                if ch == 'm' {
                    // colors always end with m
                    ret.push_str("m%}");
                    color = false;
                } else {
                    ret.push(ch)
                }
            } else if ch == 0x1b_u8.into() {
                // ESC char, always starts colors
                ret.push_str(&format!("%{{{esc}", esc = ch));
                color = true;
            } else {
                ret.push(ch);
            }
        }
        ret
    } else {
        format!(
            "{cwd}{vcs}{venv}{pchar} ",
            cwd = cwd,
            vcs = vcs_status,
            venv = venv,
            pchar = prompt_char
        )
    }
}
