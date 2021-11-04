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
use git2::{Repository, Status};
use iay::colors;
use std::env;
use std::path::Path;

pub fn vcs_status() -> Option<(String, String)> {
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
                repo_stat +=
                    &colors::colored_string(&stat_symbol, &git_index_modified_color[..], "bold");
            }

            // STATE: unstaged (working tree modified)
            Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED => {
                let stat_symbol = env::var("GIT_WT_MODIFIED").unwrap_or("±".into());
                // Nice blue color for the status
                branch_color_deduced = &git_branch_modified_color;
                repo_stat +=
                    &colors::colored_string(&stat_symbol, &git_index_modified_color[..], "bold");
            }

            // STATE: unstaged (working tree new files added)
            Status::WT_NEW => {
                let stat_symbol = env::var("IAY_GIT_WT_ADDED").unwrap_or("!".into());
                branch_color_deduced = &git_wt_added_color;
                repo_stat += &colors::colored_string(&stat_symbol, &git_wt_added_color[..], "bold");
            }

            // STATE: committed (changes have been saved in the repo)
            _ => {}
        }
    }

    let branch_color = env::var("IAY_BRANCH_COLOR").unwrap_or(branch_color_deduced.into());
    let commit_color = env::var("IAY_COMMIT_COLOR").unwrap_or("magenta".into());

    if reference.is_branch() {
        branch = colors::colored_string(
            &format!("{}{}", reference.shorthand().unwrap(), commit_dist),
            &branch_color[..],
            "bold",
        );
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = colors::colored_string(
            &format!("{:.6}{}", id, commit_dist),
            &commit_color[..],
            "bold",
        );
    }

    let mut vcs_stat = String::new();
    if repo_stat.chars().count() >= 1 {
        let open_pair = colors::colored_string("[", &branch_color[..], "bold");
        let close_pair = colors::colored_string("]", &branch_color[..], "bold");
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
