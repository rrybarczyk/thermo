use crate::common::*;

#[derive(Debug)]
pub enum Error {
  IrreconcilableUnits {
    original_unit: Unit,
    target_unit: Unit,
  },
  UnrecognizedUnit {
    unit: String,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::IrreconcilableUnits {
        original_unit,
        target_unit,
      } => write!(
        f,
        "Units do not convert: `{}`≠ `{}`",
        original_unit, target_unit
      ),
      Self::UnrecognizedUnit { unit } => write!(f, "Unrecognized unit: {}", unit),
    }
  }
}
