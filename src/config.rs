#[derive(Debug)]
pub enum Mode {
  Encode(),
  Decode(),
}

#[derive(Debug)]
pub struct Config {
  pub mode: Mode,
  pub path: String,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Self, String> {
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

    let path = args[2].clone();

    Ok(Self { mode, path })
  }
}
