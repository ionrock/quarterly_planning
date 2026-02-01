//! Find .qp directory: current dir first, then walk up to repo root.
//! Stops at repository root (or filesystem root). Nearest .qp wins (monorepo).

use std::path::{Path, PathBuf};

/// Find the .qp directory to use. Prefer current directory, then walk up
/// until we hit a repo root (directory containing .git) or filesystem root.
pub fn find_qp_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    let mut nearest_qp: Option<PathBuf> = None;

    loop {
        let qp_dir = current.join(".qp");
        if qp_dir.exists() && qp_dir.is_dir() {
            nearest_qp = Some(qp_dir);
        }
        if is_repo_root(&current) {
            return nearest_qp;
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return nearest_qp;
        }
    }
}

fn is_repo_root(path: &Path) -> bool {
    path.join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_qp_current_dir() {
        let tmp = std::env::temp_dir().join("qp_test_current");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join(".qp")).unwrap();
        assert_eq!(find_qp_root(&tmp), Some(tmp.join(".qp")));
        let _ = fs::remove_dir_all(&tmp);
    }
}
