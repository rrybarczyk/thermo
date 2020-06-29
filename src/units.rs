pub(crate) use crate::common::*;

pub(crate) mod constants {
  // SI <-> SI
  pub(crate) const INCH_IN_FOOT: f64 = 12.0;

  // SI <-> METRIC
  pub(crate) const BTU_TO_JOULE: f64 = 1055.0558526;
  pub(crate) const INCH3_IN_LITER: f64 = 61.0237;

  // METRIC <-> SI
  pub(crate) const JOULE_TO_BTU: f64 = 0.000947817;
  pub(crate) const WATT_TO_FEET_POUNDS_PER_SECOND: f64 = 0.73756;

  // METRIC <-> METRIC
  pub(crate) const KILOWATT_TO_WATT: f64 = 1000.0;
  pub(crate) const WATT_TO_KILOWATT: f64 = 0.001;
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  BritishThermalUnit,
  CubicFoot,
  CubicInch,
  Foot,
  FootPoundsPerSecond,
  Inch,
  Liter,
  Joule,
  KiloWatt,
  Watt,
}

impl FromStr for Unit {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      "btu" | "Btu" | "BTU" => Ok(Unit::BritishThermalUnit),
      "ft" | "FT" => Ok(Unit::Foot),
      "in" | "IN" => Ok(Unit::Inch),
      "ftlb/s" | "ft-lb/s" | "ft lb/s" | "ft lb / s" | "FTLB/S" | "FT-LB/S" | "FT LB/S" => {
        Ok(Unit::FootPoundsPerSecond)
      }
      "ft3" | "Ft3" | "FT3" => Ok(Unit::CubicFoot),
      "in3" | "In3" | "IN3" => Ok(Unit::CubicInch),
      "l" | "L" => Ok(Unit::Liter),
      "j" | "J" => Ok(Unit::Joule),
      "kw" | "kW" | "KW" => Ok(Unit::KiloWatt),
      "w" | "W" => Ok(Unit::Watt),
      _ => Err(Error::UnrecognizedUnit {
        unit: text.to_string(),
      }),
    }
  }
}

impl Display for Unit {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::BritishThermalUnit => write!(f, "Btu"),
      Self::CubicFoot => write!(f, "ft³"),
      Self::CubicInch => write!(f, "in³"),
      Self::Foot => write!(f, "ft"),
      Self::FootPoundsPerSecond => write!(f, "ft-lb/s"),
      Self::Inch => write!(f, "in"),
      Self::Liter => write!(f, "L"),
      Self::Joule => write!(f, "J"),
      Self::Watt => write!(f, "W"),
      Self::KiloWatt => write!(f, "kW"),
    }
  }
}
