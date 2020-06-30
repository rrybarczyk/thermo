pub(crate) use crate::common::*;

#[derive(Debug, PartialEq)]
pub struct Convert {
  original_unit: Unit,
  original_value: f64,
  target_unit: Unit,
  target_value: f64,
}

impl FromStr for Convert {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let mut vec: Vec<&str> = text.split_whitespace().collect();

    let original_value = vec.remove(0).parse::<f64>().unwrap();
    let original_unit = Unit::from_str(vec.remove(0))?;
    vec.remove(0);
    let target_unit = Unit::from_str(vec.remove(0))?;
    Convert::new(original_value, original_unit, target_unit)
  }
}

impl Display for Convert {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{} {} = {} {}",
      self.original_value, self.original_unit, self.target_value, self.target_unit
    )
  }
}

impl Convert {
  pub fn new(original_value: f64, original_unit: Unit, target_unit: Unit) -> Result<Convert> {
    if original_unit == Unit::Foot && target_unit == Unit::Inch {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::ft_to_in(original_value),
      });
    } else if original_unit == Unit::Inch && target_unit == Unit::Foot {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::in_to_ft(original_value),
      });
    } else if original_unit == Unit::CubicFoot && target_unit == Unit::CubicInch {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::ft3_to_in3(original_value),
      });
    } else if original_unit == Unit::CubicInch && target_unit == Unit::CubicFoot {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::in3_to_ft3(original_value),
      });
    } else if original_unit == Unit::CubicFoot && target_unit == Unit::Liter {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::ft3_to_l(original_value),
      });
    } else if original_unit == Unit::CubicInch && target_unit == Unit::Liter {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::in3_to_l(original_value),
      });
    } else if original_unit == Unit::Liter && target_unit == Unit::CubicFoot {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::l_to_ft3(original_value),
      });
    } else if original_unit == Unit::Liter && target_unit == Unit::CubicInch {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::l_to_in3(original_value),
      });
    } else if original_unit == Unit::Joule && target_unit == Unit::BritishThermalUnit {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::j_to_btu(original_value),
      });
    } else if original_unit == Unit::BritishThermalUnit && target_unit == Unit::Joule {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::btu_to_j(original_value),
      });
    } else if original_unit == Unit::Watt && target_unit == Unit::KiloWatt {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::w_to_kw(original_value),
      });
    } else if original_unit == Unit::KiloWatt && target_unit == Unit::Watt {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::kw_to_w(original_value),
      });
    } else if original_unit == Unit::KiloWatt && target_unit == Unit::Watt {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::kw_to_w(original_value),
      });
    } else if original_unit == Unit::Watt && target_unit == Unit::FootPoundsPerSecond {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::w_to_ft_lb_per_s(original_value),
      });
    } else if original_unit == Unit::KiloWatt && target_unit == Unit::FootPoundsPerSecond {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::kw_to_ft_lb_per_s(original_value),
      });
    } else if original_unit == Unit::FootPoundsPerSecond && target_unit == Unit::Watt {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::ft_lb_per_s_to_w(original_value),
      });
    } else if original_unit == Unit::FootPoundsPerSecond && target_unit == Unit::KiloWatt {
      return Ok(Convert {
        original_unit,
        original_value,
        target_unit,
        target_value: Convert::ft_lb_per_s_to_kw(original_value),
      });
    } else {
      return Err(Error::IrreconcilableUnits {
        original_unit,
        target_unit,
      });
    }
  }

  /// Convert in to ft
  pub fn in_to_ft(inch: f64) -> f64 {
    inch * INCH_IN_FOOT.powi(-1)
  }

  /// Convert ft to in
  pub fn ft_to_in(ft: f64) -> f64 {
    ft * INCH_IN_FOOT
  }
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

  /// Convert joule to British thermal unit
  pub fn j_to_btu(j: f64) -> f64 {
    j * JOULE_TO_BTU
  }

  /// Convert British thermal unit to joule
  pub fn btu_to_j(j: f64) -> f64 {
    j * BTU_TO_JOULE
  }

  /// Convert Watt to KiloWatt
  pub fn w_to_kw(w: f64) -> f64 {
    w * MILLI
  }

  /// Convert KiloWatt to Watt
  pub fn kw_to_w(kw: f64) -> f64 {
    kw * KILO
  }

  /// Convert W to foot pounds per second (ft-lb/sec)
  pub fn ft_lb_per_s_to_w(ftlb_per_s: f64) -> f64 {
    ftlb_per_s * 1.0 / WATT_TO_FEET_POUNDS_PER_SECOND
  }

  /// Convert W to foot pounds per second (ft-lb/sec)
  pub fn ft_lb_per_s_to_kw(ftlb_per_s: f64) -> f64 {
    ftlb_per_s * 1.0 / (WATT_TO_FEET_POUNDS_PER_SECOND * KILO)
  }

  /// Convert W to foot pounds per second (ft-lb/sec)
  pub fn w_to_ft_lb_per_s(w: f64) -> f64 {
    w * WATT_TO_FEET_POUNDS_PER_SECOND
  }

  /// Convert kW to foot pounds per second (ft-lb/sec)
  pub fn kw_to_ft_lb_per_s(kw: f64) -> f64 {
    kw * WATT_TO_FEET_POUNDS_PER_SECOND * KILO
  }

  /// Convert Pound Force to Newton
  pub fn lbf_to_n(lbf: f64) -> f64 {
    lbf * POUND_FORCE_TO_NEWTONS
  }

  /// Convert Newton to Pound Force
  pub fn n_to_lbf(newtons: f64) -> f64 {
    newtons * NEWTONS_TO_POUND_FORCE
  }

  /// Convert gram to pound
  pub fn g_to_lb(gram: f64) -> f64 {
    gram * GRAM_TO_POUND
  }

  /// Convert pound to gram
  pub fn lb_to_g(lb: f64) -> f64 {
    lb * POUND_TO_GRAM
  }

  /// Convert second to minute
  pub fn s_to_min(s: f64) -> f64 {
    s * SECOND_TO_MINUTE
  }

  /// Convert minute to second
  pub fn min_to_s(s: f64) -> f64 {
    s * MINUTE_TO_SECOND
  }

  /// Convert grams/sec to pound/minute
  pub fn g_per_s_to_lb_per_min(g_per_s: f64) -> f64 {
    g_per_s * 1.0 / SECOND_TO_MINUTE * GRAM_TO_POUND
  }

  /// Convert lbf/in² to Pascal
  pub fn lbf_per_in2_to_pascal(lbf_per_in2: f64) -> f64 {
    lbf_per_in2 * POUND_FORCE_PER_IN2_TO_PASCAL
  }

  /// Convert Pascal to lbf/in²
  pub fn pascal_to_lbf_per_in2(pascal: f64) -> f64 {
    pascal * PASCAL_TO_POUND_FORCE_PER_IN2
  }

  /// Convert meter to feet
  pub fn m_to_ft(m: f64) -> f64 {
    m * METER_TO_FOOT
  }

  /// Convert foot to meter
  pub fn ft_to_m(m: f64) -> f64 {
    m * FOOT_TO_METER
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // INCOMPLETE TEST
  #[test]
  fn convert_units_from_str() -> Result<(), Error> {
    let want = Convert {
      original_value: 1.0,
      original_unit: Unit::Foot,
      target_value: INCH_IN_FOOT,
      target_unit: Unit::Inch,
    };

    let text = format!("{} ft to in", 1.0);
    let have = Convert::from_str(&text)?;

    assert_eq!(have, want);
    Ok(())
  }
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
    let have = Convert::ft3_to_in3(foot);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_inch_to_cubic_foot() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.000579;
    let inch = 1.0;
    let have = Convert::in3_to_ft3(inch);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_liter_to_cubic_inch() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 61.0237;
    let liter = 1.0;
    let have = Convert::l_to_in3(liter);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_inch_to_liter() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.0163871;
    let cubic_inch = 1.0;
    let have = Convert::in3_to_l(cubic_inch);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_liter_to_cubic_foot() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-4);
    let want = 0.0353147;
    let liter = 1.0;
    let have = Convert::l_to_ft3(liter);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_cubic_foot_to_liter() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-4);
    let want = 28.3168;
    let cubic_foot = 1.0;
    let have = Convert::ft3_to_l(cubic_foot);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_joule_to_btu() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-4);
    let want = 0.000947817;
    let joule = 1.0;
    let have = Convert::j_to_btu(joule);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_btu_to_joule() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-6);
    let want = 1055.0558526;
    let btu = 1.0;
    let have = Convert::btu_to_j(btu);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_w_to_kw() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 1.0;
    let w = 1000.0;
    let have = Convert::w_to_kw(w);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_kw_to_w() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 1000.0;
    let kw = 1.0;
    let have = Convert::kw_to_w(kw);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_w_to_ft_pound_per_s() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.73756;
    let w = 1.0;
    let have = Convert::w_to_ft_lb_per_s(w);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_kw_to_ft_pound_per_s() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 99.57089;
    let kw = 0.135;
    let have = Convert::kw_to_ft_lb_per_s(kw);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_ft_lb_per_s_to_w() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 1.35582;
    let ft_lb_per_s = 1.0;
    let have = Convert::ft_lb_per_s_to_w(ft_lb_per_s);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_ft_lb_per_s_to_kw() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 0.00135582;
    let ft_lb_per_s = 1.0;
    let have = Convert::ft_lb_per_s_to_kw(ft_lb_per_s);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_lbf_to_n() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 4.4482216;
    let lbf = 1.0;
    let have = Convert::lbf_to_n(lbf);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_n_to_lbf() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 0.2248089;
    let newton = 1.0;
    let have = Convert::n_to_lbf(newton);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_g_to_lb() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.00220462;
    let g = 1.0;
    let have = Convert::g_to_lb(g);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_s_to_min() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 0.01666666;
    let s = 1.0;
    let have = Convert::s_to_min(s);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_min_to_s() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 60.0;
    let min = 1.0;
    let have = Convert::min_to_s(min);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  ///   g    60 s       lb
  /// 1 - * ----- *  ---------
  ///   s    min     453.592 g
  #[test]
  fn convert_g_per_s_to_lb_per_min() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-3);
    let want = 0.132277;
    let g_per_s = 1.0;
    let have = Convert::g_per_s_to_lb_per_min(g_per_s);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_lbf_per_in2_to_pascal() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-2);
    let want = 6894.76;
    let lbf_per_in2 = 1.0;
    let have = Convert::lbf_per_in2_to_pascal(lbf_per_in2);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_pascal_to_lbf_per_in2() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.000145038;
    let pascal = 1.0;
    let have = Convert::pascal_to_lbf_per_in2(pascal);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_m_to_ft() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 3.28084;
    let m = 1.0;
    let have = Convert::m_to_ft(m);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }

  #[test]
  fn convert_ft_to_m() -> Result<(), ()> {
    let tolerance = 10.0_f64.powi(-5);
    let want = 0.3048;
    let ft = 1.0;
    let have = Convert::ft_to_m(ft);

    assert_eq!(compare(want, have, tolerance), true);
    Ok(())
  }
}
