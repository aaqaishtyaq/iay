use std::env;
use git2::{ Repository, Status };

pub fn status() -> Option<(String, String)> {
    let current_dir = env::var("PWD").unwrap();

    let repo = match Repository::open(current_dir) {
        Ok(r) => r,
        Err(_) => return None
    };

    let reference = repo.head().unwrap();
    let branch;

    if reference.is_branch() {
        branch = format!("{}", reference.shorthand().unwrap());
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = format!("{}", id);
    }

    let mut repo_stat = String::new();
    let file_stat = repo.statuses(None).unwrap();
    for file in file_stat.iter() {
        match file.status() {
            Status::WT_NEW |
            Status::WT_MODIFIED |
            Status::WT_DELETED |
            Status::WT_TYPECHANGE |
            Status::WT_RENAMED => {
                let stat_char = env::var("GIT_DIRTY").unwrap_or("±".into());
                repo_stat = stat_char;
                break;
            },
            Status::INDEX_NEW |
            Status::INDEX_MODIFIED |
            Status::INDEX_DELETED |
            Status::INDEX_TYPECHANGE |
            Status::INDEX_RENAMED => {
                let stat_char = env::var("GIT_CLEAN").unwrap_or("·".into());
                repo_stat = stat_char;
            }
            _ => {}

        }
    }
    return Some((branch, repo_stat))
}
