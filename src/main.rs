mod common;
mod convert;
mod error;
mod opt;
mod units;

use crate::common::*;

fn main() -> Result<()> {
  let mut buffer = std::env::args().collect::<Vec<String>>();
  buffer.remove(0); // remove "thermo" from inputs

  let slice = buffer.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  run(&slice)?;

  Ok(())
}

fn run(args: &[&str]) -> Result<()> {
  let (flags, mut program): (Vec<&str>, Vec<&str>) = args.iter().partition(|f| f.starts_with("--"));
  let _opt = Opt::from_iter(flags);

  let cmd = program.remove(0);
  match cmd {
    "convert" => {
      let result = Convert::from_str(&program.join(" "))?;
      println!("{}", result);
    }
    _ => panic!("don't forget to put `convert` first"),
  };

  Ok(())
}
