use gix::bstr::ByteSlice;
use std::process;

fn main() {
    match run(".") {
        Ok(branch) => println!("{}", branch),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = gix::discover(path)?;

    let branch = if let Ok(r) = repo.find_reference("refs/remotes/origin/HEAD") {
        let target = r.target();
        let name = target.try_name().ok_or("HEAD is not symbolic")?;
        name.as_bstr()
            .to_str()?
            .strip_prefix("refs/remotes/origin/")
            .ok_or("Invalid ref format")?
            .to_string()
    } else {
        ["main", "master"]
            .iter()
            .find(|&&name| repo.find_reference(&format!("refs/heads/{}", name)).is_ok())
            .ok_or("Could not determine default branch")?
            .to_string()
    };

    Ok(branch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    fn init_repo(dir: &std::path::Path, branch: &str) {
        Command::new("git")
            .args(["init", "--initial-branch", branch])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir)
            .output()
            .unwrap();
    }

    fn commit(dir: &std::path::Path, msg: &str) {
        fs::write(dir.join("test.txt"), msg).unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", msg])
            .current_dir(dir)
            .output()
            .unwrap();
    }

    #[test]
    fn test_main_branch() {
        let tmp = tempfile::tempdir().unwrap();
        init_repo(tmp.path(), "main");
        commit(tmp.path(), "initial");

        let result = run(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(result, "main");
    }

    #[test]
    fn test_master_branch() {
        let tmp = tempfile::tempdir().unwrap();
        init_repo(tmp.path(), "master");
        commit(tmp.path(), "initial");

        let result = run(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(result, "master");
    }

    #[test]
    fn test_origin_head() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_dir = tmp.path().join("repo");
        let clone_dir = tmp.path().join("clone");

        fs::create_dir(&repo_dir).unwrap();
        init_repo(&repo_dir, "default");
        commit(&repo_dir, "initial");

        Command::new("git")
            .args([
                "clone",
                repo_dir.to_str().unwrap(),
                clone_dir.to_str().unwrap(),
            ])
            .output()
            .unwrap();

        let result = run(clone_dir.to_str().unwrap()).unwrap();
        assert_eq!(result, "default");
    }
}
