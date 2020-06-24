use crate::common::*;
#[derive(Debug, StructOpt)]
#[structopt(
      name = "thermo",
          global_settings = &[AppSettings::NoBinaryName]
)]
pub(crate) struct Opt {
  #[structopt(long)]
  pub(crate) verbose: bool,
}
