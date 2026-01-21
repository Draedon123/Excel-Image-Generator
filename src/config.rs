use std::path;

#[derive(Debug)]
pub enum Mode {
  Encode(),
  Decode(),
}

#[derive(Debug)]
pub struct Config<'a> {
  pub mode: Mode,
  pub path: &'a path::Path,
}

impl<'a> Config<'a> {
  pub fn new(args: &'a Vec<String>) -> Result<Self, String> {
    if args.len() < 3 {
      return Err(String::from("Not enough arguments (expected 2)"));
    };

    let mode = if args[1] == "-e" {
      Mode::Encode()
    } else if args[1] == "-d" {
      Mode::Decode()
    } else {
      return Err(format!(
        "Invalid mode '{}' (expected -e or -d for encode and decode respectively",
        args[1]
      ));
    };

    let path: &'a path::Path = path::Path::new(&args[2]);

    if !path.exists() {
      return Err(format!("Path {} does not exist", args[2]));
    }

    Ok(Self { mode, path })
  }
}
