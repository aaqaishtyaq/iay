/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use git2::{Oid, Repository, Status, StatusOptions};
use iay::colors;
use std::cell::Cell;
use std::env;

fn vcs_status() -> Option<(String, String)> {
    let current_dir = env::current_dir().ok()?;
    let mut repo = Repository::discover(current_dir).ok()?;

    let mut commit_dist: String = "".into();
    if enabled("IAY_GIT_SHOW_UPSTREAM", true) {
        if let Some((ahead, behind)) = get_ahead_behind(&repo) {
            if ahead > 0 {
                commit_dist.push_str(&colors::colored_string(
                    &format!(" {}⇡", ahead),
                    "magenta",
                    "bold",
                ));
            }
            if behind > 0 {
                commit_dist.push_str(&colors::colored_string(
                    &format!(" {}⇣", behind),
                    "cyan",
                    "bold",
                ));
            }
        }
    }

    let (repo_stat, branch_color_deduced) = build_git_status_tray(&mut repo);

    let branch_color = env::var("IAY_BRANCH_COLOR").unwrap_or(branch_color_deduced);
    let commit_color = env::var("IAY_COMMIT_COLOR").unwrap_or_else(|_| "magenta".into());

    let reference = match repo.head() {
        Ok(r) => r,
        Err(_) => return None,
    };

    let branch = if reference.is_branch() {
        colors::colored_string(
            &format!("{}{}", reference.shorthand().unwrap(), commit_dist),
            &branch_color[..],
            "bold",
        )
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        colors::colored_string(
            &format!("{:.6}{}", id, commit_dist),
            &commit_color[..],
            "bold",
        )
    };

    let mut vcs_stat = String::new();
    if repo_stat.chars().count() >= 1 {
        let open_pair = colors::colored_string(" [", &branch_color[..], "bold");
        let close_pair = colors::colored_string("]", &branch_color[..], "bold");
        vcs_stat = [open_pair, repo_stat, close_pair].concat()
    }

    Some((branch, vcs_stat))
}

fn build_git_status_tray(repo: &mut Repository) -> (String, String) {
    let git_clean_color = env::var("IAY_GIT_CLEAN_COLOR").unwrap_or_else(|_| "green".into());
    let git_wt_added_color = env::var("IAY_GIT_WT_ADDED_COLOR").unwrap_or_else(|_| "yellow".into());
    let git_index_modified_color =
        env::var("IAY_GIT_INDEX_MODIFIED_COLOR").unwrap_or_else(|_| "green".into());
    let git_wt_modified_color = env::var("IAY_GIT_WT_MODIFIED").unwrap_or_else(|_| "red".into());
    let git_branch_modified_color =
        env::var("IAY_GIT_BRANCH_MODIFIED_COLOR").unwrap_or_else(|_| "blue".into());
    let mut repo_stat = String::new();
    let mut branch_color_deduced = (git_clean_color[..]).to_string();

    let file_stats = get_repo_statuses(repo);

    if file_stats.intersects(Status::WT_NEW) {
        let stat_symbol = configured_symbol("IAY_GIT_STATUS_NEW", "!");
        branch_color_deduced = (git_wt_added_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_wt_added_color[..], "bold");
    }

    if file_stats.intersects(
        Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED | Status::WT_TYPECHANGE,
    ) {
        let stat_symbol = configured_symbol("IAY_GIT_STATUS_UNSTAGED", "±");
        branch_color_deduced = (git_branch_modified_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_wt_modified_color[..], "bold");
    }

    if file_stats.intersects(
        Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_RENAMED
            | Status::INDEX_TYPECHANGE,
    ) {
        let stat_symbol = configured_symbol("IAY_GIT_STATUS_STAGED", "±");
        branch_color_deduced = (git_branch_modified_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_index_modified_color[..], "bold");
    }

    if enabled("IAY_GIT_SHOW_STASH", true) && is_stashed(repo) {
        let stat_symbol = env::var("IAY_GIT_STATUS_STASH").unwrap_or_else(|_| "$".into());
        repo_stat += &colors::colored_string(&stat_symbol, &branch_color_deduced[..], "bold");
    }

    (repo_stat, branch_color_deduced)
}

fn is_stashed(repo: &mut Repository) -> bool {
    let stashed = Cell::new(false);

    let _ = repo.stash_foreach(|_a: usize, _b: &str, _c: &Oid| -> bool {
        stashed.set(true);
        // stop as soon as we determine that there's any stash
        false
    });

    stashed.get()
}

fn get_repo_statuses(repo: &Repository) -> Status {
    let mut options = StatusOptions::new();
    options.include_untracked(enabled("IAY_GIT_CHECK_UNTRACKED", true));

    repo.statuses(Some(&mut options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or_else(|_| Status::empty())
}

fn enabled(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => !matches!(value.as_str(), "0" | "false" | "no"),
        Err(_) => default,
    }
}

fn configured_symbol(name: &str, default: &str) -> String {
    env::var(name)
        .or_else(|_| env::var("IAY_GIT_STATUS_STAGED"))
        .unwrap_or_else(|_| default.into())
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = (r.head().ok())?;
    if !head.is_branch() {
        return None;
    }

    let head_name = head.shorthand().ok()?;
    let head_branch = (r.find_branch(head_name, git2::BranchType::Local).ok())?;
    let upstream = (head_branch.upstream().ok())?;
    let head_oid = (head.target())?;
    let upstream_oid = (upstream.get().target())?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}

fn vcs_tray() -> String {
    let vcs_tuple = vcs_status();
    let mut vcs_component = String::new();
    if let Some((branch, status)) = vcs_tuple {
        vcs_component = format!(" {}{} ", branch, status);
    } else {
        vcs_component.push(' ');
    }

    vcs_component
}

pub fn status() -> String {
    match env::var("IAY_DISABLE_VCS")
        .unwrap_or_else(|_| "0".into())
        .as_ref()
    {
        "0" => vcs_tray(),
        _ => " ".into(),
    }
}
