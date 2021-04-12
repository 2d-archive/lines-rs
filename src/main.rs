use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

use colored::Colorize;
use indoc::{printdoc};
use regex::Regex;
use walkdir::WalkDir;

use crate::cli::{Cli, parse};
use crate::config::LinesConfig;

mod config;
mod cli;

struct FileContents(i32, usize);

fn main() {
  let args = parse();

  if args.path.is_file() {
    read_file(args)
  } else if args.path.is_dir() {
    read_dir(args)
  }
}

/// Computes the contents of the provided file
fn compute_file_contents(file: &Path, config: &LinesConfig) -> Result<FileContents, Error> {
  let file_contents = match read_to_string(file) {
    Ok(str) => str,
    Err(e) => return Err(e)
  };

  if file_contents.is_empty() {
    return Ok(FileContents(0, 0));
  }

  let comment_regex = Regex::new(r"^(\*|//|/\*)").unwrap();

  // compute lines
  let mut contents = FileContents(0, 0);

  // filter lines
  let lines = file_contents.lines()
    .map(|l| l.trim())
    // filter out empty lines & lines only consisting of comments.
    .filter(|l| config.include_empty || !l.is_empty())
    .filter(|l| config.include_comments || !comment_regex.is_match(l));

  for line in lines {
    contents.0 += 1;
    contents.1 += line.len()
  }

  return Ok(contents);
}

fn read_file(args: Cli) {
  let contents = match compute_file_contents(&args.path, &args.config) {
    Ok(contents) => contents,
    Err(err) => {
      println!("Couldn't read contents of {} because {:?}", args.path.to_str().unwrap().cyan(), err);
      return;
    }
  };

  printdoc! {r#"
    Stats for {file}
      — {lines} total lines
      — {chars} total chars
    "#,
    file = args.path.to_str().unwrap().cyan(),
    lines = contents.0.to_string().as_str().yellow(),
    chars = contents.1.to_string().as_str().yellow()
  }
}

fn read_dir(args: Cli) {
  let entries = WalkDir::new(&args.path).into_iter()
    .filter_map(|e| e.ok())
    .map(|e| e.into_path())
    .filter(|p| p.is_file())
    .map(|p| compute_file_contents(&p, &args.config))
    .filter_map(|r| r.ok());

  let (mut chars, mut lines, mut files) = (0, 0, 0);
  for contents in entries {
    files += 1;
    lines += contents.0;
    chars += contents.1;
  }

  println!("Stats for {file}
    — {chars} total chars
    — {files} total files
    — {lines} total lines
    ",
           file = args.path.to_str().unwrap().cyan(),
           chars = chars.to_string().as_str().yellow(),
           files = files.to_string().as_str().yellow(),
           lines = lines.to_string().as_str().yellow())
}
