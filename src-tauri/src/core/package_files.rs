//! Package content shared by install and copy-mode deployment.
//! Never follow a package link outside its root. Local migration can explicitly
//! materialize reviewed external dependencies before importing that snapshot.
use anyhow::{bail, Context, Result};
use std::path::Path;
use walkdir::WalkDir;

pub fn included(entry: &walkdir::DirEntry) -> bool {
    entry.depth() == 0 || !(entry.depth() == 1 && entry.file_name() == ".git")
}

/// Validate before removing any existing destination. Relative internal links
/// survive relocation; absolute/external and dangling links need source repair.
pub fn validate(source: &Path) -> Result<()> {
    let root = source.canonicalize()?;
    for item in WalkDir::new(&root).follow_links(false).into_iter().filter_entry(included) {
        let item = item?;
        if item.file_type().is_symlink() {
            let link = std::fs::read_link(item.path())?;
            let resolved = item.path().canonicalize().with_context(|| format!("Unresolved package link: {}", item.path().display()))?;
            if link.is_absolute() || !resolved.starts_with(&root) || resolved.starts_with(root.join(".git")) {
                bail!("Package link requires an explicit local snapshot: {} -> {}", item.path().display(), link.display());
            }
        } else if !item.file_type().is_file() && !item.file_type().is_dir() {
            bail!("Unsupported package entry: {}", item.path().display());
        }
    }
    Ok(())
}

pub fn copy(source: &Path, destination: &Path) -> Result<()> {
    validate(source)?;
    copy_inner(source, destination, true)
}

fn copy_inner(source: &Path, destination: &Path, root: bool) -> Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        if root && entry.file_name() == ".git" { continue; }
        let target = destination.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            let link = std::fs::read_link(entry.path())?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(link, target)?;
            #[cfg(windows)]
            if entry.path().is_dir() {
                std::os::windows::fs::symlink_dir(link, target)?;
            } else {
                std::os::windows::fs::symlink_file(link, target)?;
            }
            #[cfg(not(any(unix, windows)))]
            bail!("Preserving package links is not supported on this platform");
        } else if kind.is_dir() {
            copy_inner(&entry.path(), &target, false)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    // Set directory modes after copying so a read-only source remains copyable.
    std::fs::set_permissions(destination, std::fs::metadata(source)?.permissions())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_nested_git_but_not_root_git() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src"); let dst = tmp.path().join("dst");
        for p in [".git", "fixtures/example/.git"] {
            std::fs::create_dir_all(src.join(p)).unwrap();
            std::fs::write(src.join(p).join("config"), "fixture config").unwrap();
        }
        copy(&src, &dst).unwrap();
        assert!(!dst.join(".git").exists());
        assert_eq!(std::fs::read_to_string(dst.join("fixtures/example/.git/config")).unwrap(), "fixture config");
    }
    #[cfg(unix)]
    #[test]
    fn preserves_internal_links_and_modes_and_rejects_external_links() {
        use std::os::unix::fs::{symlink, PermissionsExt};
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src"); let dst = tmp.path().join("dst");
        std::fs::create_dir_all(src.join("scripts")).unwrap();
        std::fs::write(src.join("scripts/run"), "#!/bin/sh\n").unwrap();
        std::fs::set_permissions(src.join("scripts/run"), std::fs::Permissions::from_mode(0o751)).unwrap();
        std::fs::set_permissions(src.join("scripts"), std::fs::Permissions::from_mode(0o750)).unwrap();
        symlink("scripts/run", src.join("run")).unwrap();
        symlink("scripts", src.join("bin")).unwrap();
        copy(&src, &dst).unwrap();
        assert_eq!(std::fs::read_link(dst.join("run")).unwrap(), Path::new("scripts/run"));
        assert_eq!(std::fs::read_to_string(dst.join("bin/run")).unwrap(), "#!/bin/sh\n");
        assert_eq!(std::fs::metadata(dst.join("scripts/run")).unwrap().permissions().mode() & 0o777, 0o751);
        assert_eq!(std::fs::metadata(dst.join("scripts")).unwrap().permissions().mode() & 0o777, 0o750);
        symlink(tmp.path(), src.join("external")).unwrap();
        assert!(validate(&src).unwrap_err().to_string().contains("explicit local snapshot"));
    }
}
