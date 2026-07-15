use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_REPO_ID: AtomicUsize = AtomicUsize::new(0);

struct TestRepo {
    path: PathBuf,
}

impl TestRepo {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "iay-vcs-regression-{}-{}",
            std::process::id(),
            NEXT_REPO_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();

        git(&path, &["init", "-q"]);
        git(&path, &["config", "user.name", "iay test"]);
        git(&path, &["config", "user.email", "iay@example.invalid"]);
        write_file(&path, "tracked.txt", "initial\n");
        git(&path, &["add", "tracked.txt"]);
        git(&path, &["commit", "-qm", "initial"]);

        Self { path }
    }

    fn prompt(&self) -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_iay"))
            .current_dir(&self.path)
            .env("PWD", &self.path)
            .output()
            .unwrap();
        assert!(output.status.success());
        strip_terminal_control(&String::from_utf8(output.stdout).unwrap())
    }

    fn branch(&self) -> String {
        git_output(&self.path, &["branch", "--show-current"])
    }

    fn configure_upstream(&self, commit: &str) {
        let branch = self.branch();
        git(&self.path, &["remote", "add", "origin", "."]);
        git(
            &self.path,
            &[
                "update-ref",
                &format!("refs/remotes/origin/{branch}"),
                commit,
            ],
        );
        git(
            &self.path,
            &["config", &format!("branch.{branch}.remote"), "origin"],
        );
        git(
            &self.path,
            &[
                "config",
                &format!("branch.{branch}.merge"),
                &format!("refs/heads/{branch}"),
            ],
        );
    }
}

fn strip_terminal_control(value: &str) -> String {
    let mut plain = String::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            for terminator in chars.by_ref() {
                if terminator == 'm' {
                    break;
                }
            }
        } else {
            plain.push(character);
        }
    }

    plain.replace("\\[", "").replace("\\]", "")
}

impl Drop for TestRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn git(repo: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(["-C", repo.to_str().unwrap()])
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

fn git_output(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(["-C", repo.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?} failed");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn write_file(repo: &Path, name: &str, contents: &str) {
    fs::write(repo.join(name), contents).unwrap();
}

#[test]
fn preserves_legacy_vcs_indicators() {
    let clean = TestRepo::new();
    assert!(
        !clean.prompt().contains('['),
        "clean repo should have no status tray"
    );

    let untracked = TestRepo::new();
    write_file(&untracked.path, "new.txt", "new\n");
    assert!(untracked.prompt().contains('!'));

    let unstaged = TestRepo::new();
    write_file(&unstaged.path, "tracked.txt", "modified\n");
    assert!(unstaged.prompt().contains('±'));

    let staged = TestRepo::new();
    write_file(&staged.path, "tracked.txt", "staged\n");
    git(&staged.path, &["add", "tracked.txt"]);
    assert!(staged.prompt().contains('±'));

    let stashed = TestRepo::new();
    write_file(&stashed.path, "tracked.txt", "stashed\n");
    git(&stashed.path, &["stash", "push", "-qm", "prompt-test"]);
    assert!(stashed.prompt().contains('$'));

    let ahead = TestRepo::new();
    let base = git_output(&ahead.path, &["rev-parse", "HEAD"]);
    ahead.configure_upstream(&base);
    write_file(&ahead.path, "tracked.txt", "ahead\n");
    git(&ahead.path, &["add", "tracked.txt"]);
    git(&ahead.path, &["commit", "-qm", "ahead"]);
    assert!(ahead.prompt().contains('⇡'));

    let behind = TestRepo::new();
    let base = git_output(&behind.path, &["rev-parse", "HEAD"]);
    write_file(&behind.path, "tracked.txt", "behind\n");
    git(&behind.path, &["add", "tracked.txt"]);
    git(&behind.path, &["commit", "-qm", "upstream"]);
    let upstream = git_output(&behind.path, &["rev-parse", "HEAD"]);
    git(&behind.path, &["reset", "--hard", &base]);
    behind.configure_upstream(&upstream);
    assert!(behind.prompt().contains('⇣'));
}
