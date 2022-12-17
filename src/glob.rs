use std::path::Path;
use wax::{Glob, Pattern};

pub fn match_patterns (patterns: &Vec<String>, path: &Path) -> bool {
  let patterns: &Vec<&str> = &patterns.iter().map(|s| s.as_str()).collect();
  let any = wax::any::<Glob, _>(patterns.to_vec()).unwrap();

  if path.is_dir() {
    let mut ignore = false;
    let parent = path.parent().unwrap().to_str().unwrap();
    if parent == "" {
      return false;
    }
    for pattern in patterns {
      if pattern.ends_with("**/*") && pattern.starts_with("**/") {
        let pattern = pattern
          .replace("/**/*", "")
          .replace("**/", "");
        let path_without_parent = path
          .to_str()
          .unwrap()
          .replace(format!("{}/", parent).as_str(), "");
        if path_without_parent.contains(pattern.as_str()) {
          ignore = true;
        }
      }
    }
    return ignore;
  }
  
  any.is_match(path)
}
