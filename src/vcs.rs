use colored::*;
use git2::{Repository, Status};
use std::env;
use std::path::Path;

pub fn vcs_status() -> Option<(colored::ColoredString, String)> {
    let current_dir = env::var("PWD").unwrap();

    let mut repo: Option<Repository> = None;
    let current_path = Path::new(&current_dir[..]);
    for path in current_path.ancestors() {
        match Repository::open(path) {
            Ok(r) => {
                repo = Some(r);
                break;
            }
            Err(_) => {}
        }
    }
    if repo.is_none() {
        return None;
    }
    let repo = repo.unwrap();

    let mut commit_dist: String = "".into();
    if let Some((ahead, behind)) = get_ahead_behind(&repo) {
        if ahead > 0 {
            commit_dist.push_str(" ↑");
        }
        if behind > 0 {
            commit_dist.push_str(" ↓");
        }
    }

    let reference = match repo.head() {
        Ok(r) => r,
        Err(_) => return None,
    };
    let branch;

    let git_clean_color = env::var("IAY_GIT_CLEAN_COLOR").unwrap_or("green".into());
    let git_wt_added_color = env::var("IAY_GIT_WT_ADDED_COLOR").unwrap_or("yellow".into());
    let git_wt_modified_color = env::var("IAY_GIT_WT_MODIFIED_COLOR").unwrap_or("red".into());
    let git_index_modified_color =
        env::var("IAY_GIT_INDEX_MODIFIED_COLOR").unwrap_or("green".into());
    let git_branch_modified_color =
        env::var("IAY_GIT_BRANCH_MODIFIED_COLOR").unwrap_or("blue".into());
    // let stat_char = env::var("IAY_GIT_CLEAN").unwrap_or("·".into());
    // let mut stat_char: Vec<colored::ColoredString> = vec![];
    let mut branch_color_deduced = &git_clean_color;
    let mut repo_stat = String::new();

    let file_stats = repo.statuses(None).unwrap();
    for file in file_stats.iter() {
        match file.status() {
            // STATE: staged (changes added to index)
            Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_TYPECHANGE
            | Status::INDEX_RENAMED => {
                let stat_symbol = env::var("IAY_GIT_INDEX_MODIFIED").unwrap_or("±".into());
                // Nice blue color for the status
                branch_color_deduced = &git_branch_modified_color;
                repo_stat += &stat_symbol
                    .color(&git_index_modified_color[..])
                    .bold()
                    .to_string();
            }

            // STATE: unstaged (working tree modified)
            Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED => {
                let stat_symbol = env::var("GIT_WT_MODIFIED").unwrap_or("±".into());
                // Nice blue color for the status
                branch_color_deduced = &git_branch_modified_color;
                repo_stat += &stat_symbol
                    .color(&git_index_modified_color[..])
                    .bold()
                    .to_string();
            }

            // STATE: unstaged (working tree modified)
            Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED => {
                let stat_symbol = env::var("GIT_WT_MODIFIED").unwrap_or("*".into());
                branch_color_deduced = &git_branch_modified_color;
                repo_stat += &stat_symbol
                    .color(&git_wt_modified_color[..])
                    .bold()
                    .to_string();
            }

            // STATE: unstaged (working tree new files added)
            Status::WT_NEW => {
                let stat_symbol = env::var("IAY_GIT_WT_ADDED").unwrap_or("!".into());
                branch_color_deduced = &git_wt_added_color;
                repo_stat += &stat_symbol
                    .color(&git_wt_added_color[..])
                    .bold()
                    .to_string();
            }

            // STATE: committed (changes have been saved in the repo)
            _ => {}
        }
    }

    let branch_color = env::var("IAY_BRANCH_COLOR").unwrap_or(branch_color_deduced.into());
    let commit_color = env::var("IAY_COMMIT_COLOR").unwrap_or("magenta".into());

    if reference.is_branch() {
        branch = format!("{}{}", reference.shorthand().unwrap(), commit_dist)
            .color(&branch_color[..])
            .bold();
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = format!("{:.6}{}", id, commit_dist)
            .color(commit_color)
            .bold();
    }

    let mut vcs_stat = String::new();
    if repo_stat.chars().count() >= 1 {
        let open_pair = "[".color(&branch_color[..]).bold().to_string();
        let close_pair = "]".color(&branch_color[..]).bold().to_string();
        vcs_stat = [open_pair, repo_stat, close_pair].concat()
    }

    return Some((branch, vcs_stat));
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
