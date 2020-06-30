pub fn main() {
  let in3 = thermo::convert::Convert::l_to_in3(1.0);
  let btu = thermo::convert::Convert::j_to_btu(650.0);
  let ft_lb_per_s = thermo::convert::Convert::kw_to_ft_lb_per_s(0.135);
  let lbf = thermo::convert::Convert::n_to_lbf(5.0);
  let lb_per_min = thermo::convert::Convert::g_per_s_to_lb_per_min(378.0);

  // 304 kPa -> lbf/in²
  let lbf_per_in2 =
    thermo::convert::Convert::pascal_to_lbf_per_in2(304.0) * thermo::units::constants::KILO;

  let ft3_per_s = 55.0
    * thermo::units::constants::METER_TO_FOOT.powi(3)
    * thermo::units::constants::SECOND_TO_HOUR;

  println!("1.0 Liter = {:4} in³", in3);
  println!("650.0 J = {:.4} BTU", btu);
  println!("0.135 kW = {:.4} ft-lb/s", ft_lb_per_s);
  println!("5.0 N = {:.4} lbf", lbf);
  println!("378.0 g/s = {:.4} lb/min", lb_per_min);
  println!("304.0 kPa = {:.4} lbf/in²", lbf_per_in2);
  println!("55.0 m³/hr = {:.4} ft³/s", ft3_per_s);
}
