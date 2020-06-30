pub(crate) use crate::common::*;

pub mod constants {
  // Time
  pub const SECOND_TO_MINUTE: f64 = 0.01666666666;
  pub const SECOND_TO_HOUR: f64 = 0.00027777;
  pub const MINUTE_TO_SECOND: f64 = 60.0;
  pub const HOUR_TO_SECOND: f64 = 3600.0;

  // SI <-> SI
  pub const INCH_IN_FOOT: f64 = 12.0;

  // SI <-> METRIC
  pub const BTU_TO_JOULE: f64 = 1055.0558526;
  pub const INCH3_IN_LITER: f64 = 61.0237;
  pub const POUND_FORCE_TO_NEWTONS: f64 = 4.4482216;
  pub const POUND_TO_GRAM: f64 = 453.592;
  pub const POUND_FORCE_PER_IN2_TO_PASCAL: f64 = 6894.76;
  pub const FOOT_TO_METER: f64 = 0.3048;

  // METRIC <-> SI
  pub const JOULE_TO_BTU: f64 = 0.000947817;
  pub const WATT_TO_FEET_POUNDS_PER_SECOND: f64 = 0.73756;
  pub const NEWTONS_TO_POUND_FORCE: f64 = 0.2248089;
  pub const GRAM_TO_POUND: f64 = 0.00220462;
  pub const PASCAL_TO_POUND_FORCE_PER_IN2: f64 = 0.000145038;
  pub const METER_TO_FOOT: f64 = 3.28084;

  // METRIC <-> METRIC
  pub const DEKA: f64 = 10.0;
  pub const HECTO: f64 = 100.0;
  pub const KILO: f64 = 1_000.0;
  pub const MEGA: f64 = 1_000_000.0;
  pub const GIGA: f64 = 1_000_000_000.0;

  pub const DECI: f64 = 0.1;
  pub const CENTI: f64 = 0.01;
  pub const MILLI: f64 = 0.001;
  pub const MICRO: f64 = 0.000_001;
  pub const NANO: f64 = 0.000_000_001;
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  BritishThermalUnit,
  CubicFoot,
  CubicInch,
  Foot,
  FootPoundsPerSecond,
  Gram,
  Inch,
  Joule,
  KiloWatt,
  Liter,
  Meter,
  Minute,
  Pascal,
  Newton,
  Pound,
  PoundForce,
  Second,
  Watt,
}

impl FromStr for Unit {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      "btu" | "Btu" | "BTU" => Ok(Unit::BritishThermalUnit),
      "ft" | "FT" => Ok(Unit::Foot),
      "in" | "IN" => Ok(Unit::Inch),
      "ftlb/s" | "ft-lb/s" | "FTLB/S" | "FT-LB/S" => Ok(Unit::FootPoundsPerSecond),
      "ft3" | "Ft3" | "FT3" => Ok(Unit::CubicFoot),
      "g" => Ok(Unit::Gram),
      "in3" | "In3" | "IN3" => Ok(Unit::CubicInch),
      "j" | "J" => Ok(Unit::Joule),
      "kw" | "kW" | "KW" => Ok(Unit::KiloWatt),
      "l" | "L" => Ok(Unit::Liter),
      "m" | "M" => Ok(Unit::Meter),
      "min" | "Min" | "MIN" => Ok(Unit::Minute),
      "n" | "N" => Ok(Unit::Newton),
      "lb" | "LB" => Ok(Unit::Pound),
      "lbf" | "lb-f" | "LBF" | "LB-F" => Ok(Unit::PoundForce),
      "pa" | "Pa" => Ok(Unit::Pascal),
      "s" | "sec" | "S" | "Sec" | "SEC" => Ok(Unit::Second),
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
      Self::Gram => write!(f, "g"),
      Self::Inch => write!(f, "in"),
      Self::Joule => write!(f, "J"),
      Self::KiloWatt => write!(f, "kW"),
      Self::Liter => write!(f, "L"),
      Self::Meter => write!(f, "m"),
      Self::Minute => write!(f, "min"),
      Self::Newton => write!(f, "N"),
      Self::Pascal => write!(f, "Pa"),
      Self::Pound => write!(f, "lb"),
      Self::PoundForce => write!(f, "lb-f"),
      Self::Second => write!(f, "s"),
      Self::Watt => write!(f, "W"),
    }
  }
}
