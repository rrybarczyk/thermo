// SI <-> SI
#[cfg(test)]
const INCH_IN_FOOT: f64 = 12.0;

// SI <-> METRIC
#[cfg(test)]
const INCH3_IN_LITER: f64 = 61.0237;

#[cfg(test)]
struct Units {}

#[cfg(test)]
impl Units {
  /* Volumetric */
  /// Convert in³ to ft³
  pub fn in3_to_ft3(ft3: f64) -> f64 {
    ft3 * 1.0 / INCH_IN_FOOT.powi(3)
  }

  /// Convert ft³ to in³
  pub fn ft3_to_in3(ft3: f64) -> f64 {
    ft3 * INCH_IN_FOOT.powi(3)
  }

  /// Convert liter to in³
  pub fn l_to_in3(l: f64) -> f64 {
    l * INCH3_IN_LITER
  }

  /// Convert in³ to liter
  pub fn in3_to_l(inch: f64) -> f64 {
    inch * 1.0 / INCH3_IN_LITER
  }

  /// Convert liter to ft³
  pub fn l_to_ft3(l: f64) -> f64 {
    l * INCH3_IN_LITER / INCH_IN_FOOT.powi(3)
  }

  /// Convert ft³ to liter
  pub fn ft3_to_l(l: f64) -> f64 {
    l * INCH_IN_FOOT.powi(3) / INCH3_IN_LITER
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compare(want: f64, have: f64, tolerance: f64) -> bool {
    let abs_difference = want.abs() - have.abs();
    if abs_difference <= tolerance {
      true
    } else {
      println!(
        "want: {}, have:{}, abs_difference: {}, tolerance: {}",
        want, have, abs_difference, tolerance
      );
      false
    }
  }

  #[test]
  fn convert_cubic_foot_to_cubic_inch() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 1728.0;
    let foot = 1.0;
    let have = Units::ft3_to_in3(foot);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_inch_to_cubic_foot() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.000579;
    let inch = 1.0;
    let have = Units::in3_to_ft3(inch);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_liter_to_cubic_inch() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 61.0237;
    let liter = 1.0;
    let have = Units::l_to_in3(liter);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_inch_to_liter() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.0163871;
    let cubic_inch = 1.0;
    let have = Units::in3_to_l(cubic_inch);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_liter_to_cubic_foot() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-4);
    let want = 0.0353147;
    let liter = 1.0;
    let have = Units::l_to_ft3(liter);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_foot_to_liter() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-4);
    let want = 28.3168;
    let cubic_foot = 1.0;
    let have = Units::ft3_to_l(cubic_foot);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }
}
