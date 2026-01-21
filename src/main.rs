use excel_image_generator::config::Config;
use std::{
  env,
  io::{self, Read, Write},
  process,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|error| {
    eprintln!("Error parsing arguments: {error}");
    pause();
    process::exit(1);
  });

  println!("{:?}", config);

  Ok(())
}

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press any key to exit").unwrap();
  stdout.flush().unwrap();

  let _ = stdin.read(&mut [0u8]).unwrap();
}
