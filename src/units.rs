pub(crate) use crate::common::*;

// SI <-> SI
pub(crate) const INCH_IN_FOOT: f64 = 12.0;

// SI <-> METRIC
pub(crate) const INCH3_IN_LITER: f64 = 61.0237;

#[derive(Debug, PartialEq)]
pub enum Unit {
  CubicFoot,
  CubicInch,
  Foot,
  Inch,
  Liter,
}

impl FromStr for Unit {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      "ft" | "FT" => Ok(Unit::Foot),
      "in" | "IN" => Ok(Unit::Inch),
      "ft3" | "FT3" => Ok(Unit::CubicFoot),
      "in3" | "IN3" => Ok(Unit::CubicInch),
      "L" | "l" => Ok(Unit::Liter),
      _ => Err(Error::UnrecognizedUnit {
        unit: text.to_string(),
      }),
    }
  }
}

impl Display for Unit {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::CubicFoot => write!(f, "ft³"),
      Self::CubicInch => write!(f, "in³"),
      Self::Foot => write!(f, "ft"),
      Self::Inch => write!(f, "in"),
      Self::Liter => write!(f, "L"),
    }
  }
}
