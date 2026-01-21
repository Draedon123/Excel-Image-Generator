use crate::config::{Config, Mode};

pub mod config;
mod encode;

pub fn run(config: &Config) {
  let _ = match config.mode {
    Mode::Encode() => encode::encode(config.path).unwrap(),
    _ => (),
  };
}
