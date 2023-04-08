use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub fn save_files(content: String, dir: &str) {
  create_dir_all(dir).unwrap_or_else(|err| {
    if err.kind() == std::io::ErrorKind::AlreadyExists {
      return;
    }
    panic!("Failed to create directory.");
  });
  save_html(&content, dir);
  save_css(&content, dir);
  save_js(&content, dir);
}

fn save_html(content: &str, dir: &str) {
  let html_content = content
    .split("```")
    .find(|str| str.starts_with("html"))
    .expect("Failed to get html.")
    .strip_prefix("html")
    .expect("Failed to get html.");

  let mut file =
    File::create(Path::new(dir).join("index.html")).expect("Failed to create html file.");
  write!(file, "{}", html_content).unwrap_or_else(|_| panic!("Failed to save html."));
}

fn save_css(content: &str, dir: &str) {
  let css_content = content
    .split("```")
    .find(|str| str.starts_with("css"))
    .expect("Failed to get css.")
    .strip_prefix("css")
    .expect("Failed to get css.");

  let mut file =
    File::create(Path::new(dir).join("index.css")).expect("Failed to create css file.");
  write!(file, "{}", css_content).unwrap_or_else(|_| panic!("Failed to save css."));
}

fn save_js(content: &str, dir: &str) {
  let js_content = content
    .split("```")
    .find(|str| str.starts_with("js"))
    .and_then(|str| str.strip_prefix("js"));

  if let Some(js_content) = js_content {
    let mut file =
      File::create(Path::new(dir).join("index.js")).expect("Failed to create js file.");
    write!(file, "{}", js_content).unwrap_or_else(|_| panic!("Failed to save js."));
  }
}

#[cfg(test)]
mod save_files_test {
  use std::fs::read_to_string;

  use super::*;

  const OUTPUT_DIR: &str = "test-output";

  fn clean_up() {
    std::fs::remove_dir_all(OUTPUT_DIR).unwrap_or_else(|err| {
      if err.kind() == std::io::ErrorKind::NotFound {
        return;
      }
      panic!("Failed to remove directory.");
    });
  }

  #[test]
  fn input() {
    let content = read_to_string("test/input.txt").unwrap();
    save_files(content, OUTPUT_DIR);
    clean_up()
  }
}
