//! Explicit catalog import. Categories are tags; imports never deploy.
use super::{
    central_repo, installer, repo_lock::RepoLock, skill_metadata, skill_store::SkillStore,
};
use crate::commands::skills::{store_installed_skill_unlocked, InstallSourceMetadata};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub source: String,
    pub name: String,
    pub category: Option<String>,
    pub existing_id: Option<String>,
    pub conflict: Option<String>,
}

pub fn preview(
    store: &SkillStore,
    root: &Path,
    category: Option<&str>,
    excluded: &[String],
) -> Result<Vec<CatalogEntry>> {
    let root = root.canonicalize()?;
    let base = if root.join("skills").is_dir() && !skill_metadata::is_valid_skill_dir(&root) {
        root.join("skills")
    } else {
        root
    };
    let mut paths = Vec::new();
    discover(&base, 0, excluded, &mut paths)?;
    let existing = store.get_all_skills()?;
    let mut entries = Vec::new();
    for path in paths {
        let canonical = path.canonicalize()?;
        let name = skill_metadata::infer_skill_name(&path);
        let relative = path.strip_prefix(&base)?;
        let inferred = relative
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().to_string());
        let existing_id = existing
            .iter()
            .find(|s| {
                s.source_type == "local"
                    && s.source_ref.as_ref().is_some_and(|r| {
                        Path::new(r).canonicalize().ok().as_ref() == Some(&canonical)
                    })
            })
            .map(|s| s.id.clone());
        let mut conflict = super::package_files::validate(&canonical).err().map(|e| e.to_string());
        if existing_id.is_none()
            && (existing.iter().any(|s| s.name == name)
                || central_repo::skills_dir()
                    .join(&name)
                    .symlink_metadata()
                    .is_ok())
        {
            conflict =
                Some("Library name already exists; resolve the name before importing".into());
        }
        entries.push(CatalogEntry {
            source: canonical.to_string_lossy().into(),
            name,
            category: category.map(str::to_owned).or(inferred),
            existing_id,
            conflict,
        });
    }
    let mut names = HashMap::new();
    let mut sources = HashMap::new();
    for e in &entries {
        *names.entry(e.name.clone()).or_insert(0) += 1;
        *sources.entry(e.source.clone()).or_insert(0) += 1;
    }
    for e in &mut entries {
        if names[&e.name] > 1 || sources[&e.source] > 1 {
            e.conflict = Some(
                "Duplicate name or source alias in catalog; select an unambiguous root".into(),
            );
        }
    }
    Ok(entries)
}

fn discover(path: &Path, depth: usize, excluded: &[String], out: &mut Vec<PathBuf>) -> Result<()> {
    if skill_metadata::is_valid_skill_dir(path) {
        out.push(path.to_owned());
        return Ok(());
    }
    if depth >= 8 {
        bail!("Catalog depth exceeds 8 at {}", path.display());
    }
    let mut children = std::fs::read_dir(path)?.collect::<std::io::Result<Vec<_>>>()?;
    children.sort_by_key(|e| e.file_name());
    for entry in children {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.')
            || [
                "node_modules",
                "venv",
                "__pycache__",
                "tui",
                "scripts",
                "docs",
                "agents",
            ]
            .contains(&name.as_str())
            || excluded.contains(&name)
        {
            continue;
        }
        if entry.path().is_dir() {
            discover(&entry.path(), depth + 1, excluded, out)?;
        }
    }
    Ok(())
}

pub fn apply(
    store: &SkillStore,
    root: &Path,
    category: Option<&str>,
    excluded: &[String],
    selected: &[String],
) -> Result<Vec<CatalogEntry>> {
    let _lock = RepoLock::acquire_foreground("catalog import")?;
    let entries = preview(store, root, category, excluded)?;
    for source in selected {
        if !entries.iter().any(|e| &e.source == source) {
            bail!("Selected source is no longer in catalog: {source}");
        }
    }
    let mut chosen: Vec<_> = entries
        .into_iter()
        .filter(|e| selected.contains(&e.source))
        .collect();
    for e in &chosen {
        if let Some(reason) = &e.conflict {
            bail!("{}: {}", e.source, reason);
        }
    }
    for e in &mut chosen {
        if e.existing_id.is_some() {
            continue;
        } // Never refresh content or overwrite manual tags on reimport.
        let result = installer::install_from_local(Path::new(&e.source), Some(&e.name))?;
        let metadata = InstallSourceMetadata {
            source_type: "local".into(),
            source_ref: Some(e.source.clone()),
            source_ref_resolved: None,
            source_subpath: None,
            source_branch: None,
            source_revision: None,
            remote_revision: None,
            update_status: "local_only".into(),
        };
        let id = store_installed_skill_unlocked(store, &result, &metadata, None)?;
        if let Some(category) = &e.category {
            store.set_tags_for_skill(&id, &[category.clone()])?;
        }
        e.existing_id = Some(id);
    }
    super::sync_metadata::write_all_from_db_unlocked(store)?;
    Ok(chosen)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn import_is_idempotent_and_preserves_manual_tags_without_deploying() {
        let _guard = central_repo::test_base_dir_lock();
        let tmp = tempfile::tempdir().unwrap();
        central_repo::set_test_base_dir_override(Some(tmp.path().join("library")));
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                central_repo::set_test_base_dir_override(None);
            }
        }
        let _reset = Reset;
        central_repo::ensure_central_repo(false).unwrap();
        let store = SkillStore::new(&tmp.path().join("test.db")).unwrap();
        let root = tmp.path().join("catalog");
        let package = root.join("devops/example");
        std::fs::create_dir_all(&package).unwrap();
        std::fs::write(package.join("SKILL.md"), "---\nname: example\n---\nHello").unwrap();
        let rows = preview(&store, &root, None, &[]).unwrap();
        assert_eq!(rows[0].category.as_deref(), Some("devops"));
        let result = apply(&store, &root, None, &[], &[rows[0].source.clone()]).unwrap();
        let id = result[0].existing_id.as_ref().unwrap();
        assert_eq!(store.get_tags_map().unwrap()[id], vec!["devops"]);
        store.set_tags_for_skill(id, &["custom".into()]).unwrap();
        apply(&store, &root, None, &[], &[rows[0].source.clone()]).unwrap();
        assert_eq!(store.get_all_skills().unwrap().len(), 1);
        assert_eq!(store.get_tags_map().unwrap()[id], vec!["custom"]);
        assert!(store.get_targets_for_skill(id).unwrap().is_empty());
        let fixture = package.join("fixtures/demo/.git");
        std::fs::create_dir_all(&fixture).unwrap();
        assert!(preview(&store, &root, None, &[]).unwrap()[0].conflict.is_none());
        #[cfg(unix)] {
            std::os::unix::fs::symlink("SKILL.md", package.join("alias.md")).unwrap();
            assert!(preview(&store, &root, None, &[]).unwrap()[0].conflict.is_none());
            std::os::unix::fs::symlink(tmp.path(), package.join("external")).unwrap();
            assert!(preview(&store, &root, None, &[]).unwrap()[0].conflict.as_ref().unwrap().contains("explicit local snapshot"));
            std::fs::remove_file(package.join("external")).unwrap();
        }
        let duplicate = root.join("other/example");
        std::fs::create_dir_all(&duplicate).unwrap();
        std::fs::write(duplicate.join("SKILL.md"), "---\nname: example\n---").unwrap();
        assert!(preview(&store, &root, None, &[])
            .unwrap()
            .iter()
            .all(|e| e.conflict.is_some()));
    }
    #[test]
    fn discovery_stops_at_packages_and_excludes_generated_trees() {
        let tmp = tempfile::tempdir().unwrap();
        for p in [
            "devops/a",
            "devops/a/examples/inner",
            "devops/sub/b",
            "node_modules/hidden",
            "excluded",
        ] {
            std::fs::create_dir_all(tmp.path().join(p)).unwrap();
            std::fs::write(tmp.path().join(p).join("SKILL.md"), "skill").unwrap();
        }
        let mut found = vec![];
        discover(tmp.path(), 0, &["excluded".into()], &mut found).unwrap();
        assert_eq!(
            found,
            vec![tmp.path().join("devops/a"), tmp.path().join("devops/sub/b")]
        );
    }
    #[test]
    fn invalid_package_does_not_touch_destination() {
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("category");
        let dest = tmp.path().join("existing");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(dest.join("precious"), "keep").unwrap();
        assert!(installer::install_skill_dir_to_destination(&source, "category", &dest).is_err());
        assert_eq!(
            std::fs::read_to_string(dest.join("precious")).unwrap(),
            "keep"
        );
    }
}
