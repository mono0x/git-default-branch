use clap::Parser;
use gix::bstr::ByteSlice;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(version, about = "Get the default branch of a Git repository")]
struct Args {
    /// Run as if git-default-branch was started in <path>
    #[arg(short = 'C', value_name = "path", default_value = ".")]
    dir: PathBuf,

    #[arg(short, long, default_value = "origin")]
    remote: String,
}

fn main() {
    let args = Args::parse();

    match run(&args.dir, &args.remote) {
        Ok(branch) => println!("{branch}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}

fn remote_head_branch(
    repo: &gix::Repository,
    remote: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let prefix = format!("refs/remotes/{remote}/");
    let Some(r) = repo.try_find_reference(&format!("{prefix}HEAD"))? else {
        return Ok(None);
    };
    let target = r.target();
    let name = target.try_name().ok_or("HEAD is not symbolic")?;
    Ok(Some(
        name.as_bstr()
            .to_str()?
            .strip_prefix(&prefix)
            .ok_or("Invalid ref format")?
            .to_string(),
    ))
}

fn run(path: &Path, remote: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = gix::discover(path)?;

    if let Some(branch) = remote_head_branch(&repo, remote)? {
        return Ok(branch);
    }

    // gix is built without network transport, so refreshing the remote HEAD symref is
    // delegated to the git CLI. Failures (git missing, offline) intentionally fall through
    // to the branch name guesses below.
    // https://qiita.com/ymm1x/items/b22bddc9fbc192ae1a70
    // https://stackoverflow.com/questions/28666357/how-to-get-default-git-branch/44750379#44750379
    let _ = process::Command::new("git")
        .args(["remote", "set-head", remote, "--auto"])
        .current_dir(path)
        .output();

    if let Some(branch) = remote_head_branch(&repo, remote)? {
        return Ok(branch);
    }

    for name in ["main", "master"] {
        if repo
            .try_find_reference(&format!("refs/heads/{name}"))?
            .is_some()
        {
            return Ok(name.to_string());
        }
    }
    Err("Could not determine default branch".into())
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Test setup and assertions should fail immediately on errors."
)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    fn git(dir: &Path, args: &[&str]) {
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
    }

    fn init_repo(dir: &Path, branch: &str) {
        git(dir, &["init", "--initial-branch", branch]);
        git(dir, &["config", "user.name", "Test"]);
        git(dir, &["config", "user.email", "test@example.com"]);
    }

    fn commit(dir: &Path, msg: &str) {
        fs::write(dir.join("test.txt"), msg).unwrap();
        git(dir, &["add", "."]);
        git(dir, &["commit", "-m", msg]);
    }

    /// Creates a repository on branch `default` and clones it with the given remote name,
    /// returning the clone. The temporary directory is returned to keep it alive.
    fn clone_repo(remote: &str) -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let repo_dir = tmp.path().join("repo");
        let clone_dir = tmp.path().join("clone");

        fs::create_dir(&repo_dir).unwrap();
        init_repo(&repo_dir, "default");
        commit(&repo_dir, "initial");

        git(
            tmp.path(),
            &[
                "clone",
                "--origin",
                remote,
                repo_dir.to_str().unwrap(),
                clone_dir.to_str().unwrap(),
            ],
        );

        (tmp, clone_dir)
    }

    #[test]
    fn test_main_branch() {
        let tmp = tempfile::tempdir().unwrap();
        init_repo(tmp.path(), "main");
        commit(tmp.path(), "initial");

        assert_eq!(run(tmp.path(), "origin").unwrap(), "main");
    }

    #[test]
    fn test_master_branch() {
        let tmp = tempfile::tempdir().unwrap();
        init_repo(tmp.path(), "master");
        commit(tmp.path(), "initial");

        assert_eq!(run(tmp.path(), "origin").unwrap(), "master");
    }

    #[test]
    fn test_origin_head() {
        let (_tmp, clone_dir) = clone_repo("origin");

        assert_eq!(run(&clone_dir, "origin").unwrap(), "default");
    }

    #[test]
    fn test_non_origin_remote() {
        let (_tmp, clone_dir) = clone_repo("upstream");

        assert_eq!(run(&clone_dir, "upstream").unwrap(), "default");
    }

    #[test]
    fn test_deleted_origin_head() {
        let (_tmp, clone_dir) = clone_repo("origin");
        let _ = fs::remove_file(clone_dir.join(".git/refs/remotes/origin/HEAD"));

        assert_eq!(run(&clone_dir, "origin").unwrap(), "default");
    }
}
