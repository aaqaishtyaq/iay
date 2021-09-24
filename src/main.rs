mod cwd;
mod prompt_char;
mod vcs;
mod venv;

use std::env;

use clap::{App, Arg};
use colored::*;

fn main() {
    let matches = App::new("iay")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("minimal")
                .short("m")
                .long("minimal")
                .help("use minimal variant"),
        )
        .arg(
            Arg::with_name("zsh")
                .short("z")
                .long("zsh")
                .help("Use ZSH formatting"),
        )
        .get_matches();
    if matches.is_present("minimal") {
        println!("{}", iay_minimal(matches.is_present("zsh")));
    } else {
        println!("{}", iay(matches.is_present("zsh")));
    }
}

fn iay(zsh: bool) -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => "[directory does not exist]".color("red"),
    };

    let (branch, status) = match env::var("DISABLE_VCS").unwrap_or("0".into()).as_ref() {
        "0" => vcs::vcs_status().unwrap_or(("".into(), "".into())),
        _ => ("".into(), "".into()),
    };

    let venv = venv::get_name();
    let prompt_char = prompt_char::prompt_char();

    if zsh {
        format!(
            "%{{{cwd} {branch} {status}%}} %{{\n{venv}{pchar}%}} ",
            cwd = cwd,
            branch = branch,
            status = status,
            venv = venv,
            pchar = prompt_char
        )
    } else {
        format!(
            "{cwd} {branch} {status}\n{venv}{nix}{pchar} ",
            cwd = cwd,
            branch = branch,
            status = status,
            venv = venv,
            pchar = prompt_char,
            nix = venv::in_nix_shell()
        )
    }
}

fn iay_minimal(zsh: bool) -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => "[directory does not exist]".color("red"),
    };
    let vcs_tuple = vcs::vcs_status();
    let mut vcs_component = String::new();
    if let Some((branch, status)) = vcs_tuple {
        vcs_component = format!(" {} {} ", branch, status);
    } else {
        vcs_component.push(' ');
    }
    let venv = venv::get_name();
    let prompt_char = prompt_char::prompt_char();

    if zsh {
        let fmt = format!(
            "{cwd}{vcs}{venv}{pchar} ",
            cwd = cwd,
            vcs = vcs_component,
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
            } else {
                if ch == 0x1b_u8.into() {
                    // ESC char, always starts colors
                    ret.push_str(&format!("%{{{esc}", esc = ch));
                    color = true;
                } else {
                    ret.push(ch);
                }
            }
        }
        ret
    } else {
        format!(
            "{cwd}{vcs}{venv}{pchar} ",
            cwd = cwd,
            vcs = vcs_component,
            venv = venv,
            pchar = prompt_char
        )
    }
}
