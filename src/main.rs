extern crate git2;

use anyhow::Result;
use anyhow::bail;
use git2::Repository;
use std::collections::BTreeMap;

fn main() -> Result<()> {
  let flags = git2::RepositoryOpenFlags::empty();
  let ceiling_dirs: &[&str] = &[];
  let current_dir = std::env::current_dir()?;
  let repo = Repository::open_ext(current_dir, flags, ceiling_dirs)?;

  let head = repo.head()?;
  let tree = head.peel_to_tree()?;
  let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), None)?;

  let mut map = BTreeMap::new();

  diff.foreach(
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
  )?;

  if map.is_empty() {
    bail!("No files found");
  }

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
