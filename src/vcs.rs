/*
IAY | Minimalist prompt for Bash/Zsh!
Copyright (C) 2021 Aaqa Ishtyaq
*/
use git2::{Oid, Repository, Status, StatusOptions};
use iay::colors;
use lazy_static::lazy_static;
use std::cell::Cell;
use std::env;
use std::path::Path;

// Taken from: https://github.com/Ryooooooga/almel/blob/467e8f699e840c418a7eed5e0e22cf9c34ed1dca/src/segments/git_repo.rs
lazy_static! {
    static ref STATUS_CONFLICTED: Status = Status::CONFLICTED;
    static ref STATUS_UNSTAGED: Status =
        Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED | Status::WT_TYPECHANGE;
    static ref STATUS_STAGED: Status = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE;
    static ref STATUS_MODIFIED: Status = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    static ref STATUS_NEW: Status = Status::WT_NEW;
    static ref STATUS_DELETED: Status = Status::WT_DELETED | Status::INDEX_DELETED;
}

pub fn vcs_status() -> Option<(String, String)> {
    let current_dir = env::var("PWD").unwrap();

    let mut repo: Option<Repository> = None;
    let current_path = Path::new(&current_dir[..]);
    for path in current_path.ancestors() {
        if let Ok(r) = Repository::open(path) {
            repo = Some(r);
            break;
        }
    }

    // return if not a git repository
    repo.as_ref()?;

    let mut repo = repo.unwrap();

    let mut commit_dist: String = "".into();
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
    let mut branch_color_deduced = (&git_clean_color[..]).to_string();

    let file_stats = get_repo_statuses(repo);

    if file_stats.intersects(*STATUS_NEW) {
        let stat_symbol = env::var("IAY_GIT_STATUS_STAGED").unwrap_or_else(|_| "!".into());
        branch_color_deduced = (&git_wt_added_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_wt_added_color[..], "bold");
    }

    if file_stats.intersects(*STATUS_UNSTAGED) {
        let stat_symbol = env::var("IAY_GIT_STATUS_STAGED").unwrap_or_else(|_| "±".into());
        branch_color_deduced = (&git_branch_modified_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_wt_modified_color[..], "bold");
    }

    if file_stats.intersects(*STATUS_STAGED) {
        let stat_symbol = env::var("IAY_GIT_STATUS_STAGED").unwrap_or_else(|_| "±".into());
        branch_color_deduced = (&git_branch_modified_color[..]).to_string();
        repo_stat += &colors::colored_string(&stat_symbol, &git_index_modified_color[..], "bold");
    }

    if is_stashed(repo) {
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
    options.include_untracked(true);

    repo.statuses(Some(&mut options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or_else(|_| Status::empty())
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = (r.head().ok())?;
    if !head.is_branch() {
        return None;
    }

    let head_name = (head.shorthand())?;
    let head_branch = (r.find_branch(head_name, git2::BranchType::Local).ok())?;
    let upstream = (head_branch.upstream().ok())?;
    let head_oid = (head.target())?;
    let upstream_oid = (upstream.get().target())?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}
