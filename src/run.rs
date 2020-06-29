use crate::common::*;

pub fn run() -> Result<()> {
  let mut buffer = std::env::args().collect::<Vec<String>>();
  buffer.remove(0); // remove "thermo" from inputs
  let args = buffer.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
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
