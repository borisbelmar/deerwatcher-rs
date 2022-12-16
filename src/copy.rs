use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::convert::TryInto;

use crate::glob::match_patterns;

pub fn copy_recursive(
  src_dir: &str,
  dst_dir: &str,
  ignored_list: &[&str],
) -> Result<(), std::io::Error> {
  for entry in WalkDir::new(src_dir)
    .into_iter()
    .filter_entry(|e| {
      let path = e.path();
      !match_patterns(ignored_list, path)
    }) {
      let entry: walkdir::DirEntry = entry?.try_into().unwrap();
      let src_path = entry.path();
      let dst_path = Path::new(dst_dir).join(src_path.strip_prefix(src_dir).unwrap());

      if entry.file_type().is_file() {
        fs::copy(src_path, dst_path)?;
      } else if entry.file_type().is_dir() {
        fs::create_dir_all(dst_path)?;
      }
    }

  Ok(())
}
