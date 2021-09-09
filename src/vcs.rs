use std::env;
use git2::Repository;

pub fn vcs() -> Option<String> {
    let current_dir = env::var("PWD").unwrap();

    let repo = match Repository::open(current_dir) {
        Ok(r) => r,
        Err(_) => return None
    };

    let reference = repo.head().unwrap();

    if reference.is_branch() {
        Some(format!("{}", reference.shorthand().unwrap()))
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        Some(format!("{}", id))
    }
}
