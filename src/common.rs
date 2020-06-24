pub(crate) use crate::{
  convert::Convert,
  error::Error,
  opt::Opt,
  units::{Unit, INCH3_IN_LITER, INCH_IN_FOOT},
};
pub(crate) use std::{
  fmt::{self, Display, Formatter},
  str::FromStr,
};
pub(crate) use structopt::{clap::AppSettings, StructOpt};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
