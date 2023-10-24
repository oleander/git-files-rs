extern crate git2;

use std::collections::BTreeMap;
use std::env;
use git2::Repository;
use std::path::Path;
use anyhow::{Context, Result};

fn main() -> Result<()> {
  let flags = git2::RepositoryOpenFlags::empty();
  let ceiling_dirs: &[&str] = &[];
  let current_dir = std::env::current_dir().unwrap();
  let repo = Repository::open_ext(current_dir, flags, ceiling_dirs).context("Failed to open repo")?;

  // Step 2: Diff
  let head = repo.revparse_single("origin/main").unwrap();
  let tree = head.peel_to_tree().unwrap();
  let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), None).unwrap();

  // Step 3: Process diff and sort
  let mut map = BTreeMap::new();

  diff
    .foreach(
      &mut |delta, _| {
        if let Some(path) = delta.new_file().path() {
          let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();
          map.entry(extension.to_owned()).or_insert_with(Vec::new).push(path.to_owned());
        }
        true
      },
      None,
      None,
      None,
    )
    .unwrap();

  // Step 4: Print
  let mut last_ext = "";

  for (ext, paths) in &map {
    for path in paths {
      if last_ext != *ext {
        println!();
        last_ext = ext;
      }
      println!("{}", path.display());
    }
  }

  Ok(())
}
