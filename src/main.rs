mod copy;
mod watcher;
mod glob;

fn main() {
  let src_dir = "/Users/borisbelmar/Proyectos/Deerwatcher/test1";
  let dst_dir = "/Users/borisbelmar/Proyectos/Deerwatcher/test2";
  let ignored_list = &[".git", "**/target/**/*", "**/src/**/*"];

  copy::copy_recursive(src_dir, dst_dir, ignored_list).unwrap();

  watcher::watch_and_copy(src_dir, dst_dir, ignored_list)
}
