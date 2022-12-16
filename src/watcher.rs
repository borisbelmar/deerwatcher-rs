use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::glob;

pub fn watch_and_copy(src_dir: &str, dst_dir: &str, ignore_list: &[&str]) {
  let (tx, rx) = channel();
  let mut watcher = notify::watcher(tx, Duration::from_secs(0)).unwrap();

  watcher.watch(Path::new(src_dir), RecursiveMode::Recursive).unwrap();

  loop {
    match rx.recv() {
      Ok(event) => match event {
        DebouncedEvent::Create(path) => {
          let src_path = path.to_str().unwrap();
          let dst_path = src_path.replace(src_dir, dst_dir);

          let ignored = glob::match_patterns(ignore_list, path.as_path());
          if !ignored {
            println!("Creating {} -> {}", src_path, dst_path);
            fs::create_dir_all(Path::new(dst_path.as_str()).parent().unwrap()).unwrap();
            if Path::new(src_path).is_file() {
              fs::copy(src_path, dst_path).unwrap();
            }
          }
        }
        DebouncedEvent::Write(path) => {
          let src_path = path.to_str().unwrap();
          let dst_path = src_path.replace(src_dir, dst_dir);
          
          let ignored = glob::match_patterns(ignore_list, path.as_path());

          if !ignored {
            println!("Writing {} -> {}", src_path, dst_path);
            fs::create_dir_all(Path::new(dst_path.as_str()).parent().unwrap()).unwrap();
            fs::copy(src_path, dst_path).unwrap();
          }
        }
        DebouncedEvent::Remove(path) => {
          let src_path = path.to_str().unwrap();
          let dst_path = path.to_str().unwrap().replace(src_dir, dst_dir);

          let ignored = glob::match_patterns(ignore_list, path.as_path());
          
          if !ignored {
            println!("Deleting {} -> {}", src_path, dst_path);
            std::fs::remove_dir_all(dst_path).unwrap();
          }
        }
        _ => {}
      },
      Err(e) => println!("Error: {:?}", e),
    }
  }
}
