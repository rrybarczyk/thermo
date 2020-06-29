pub fn main() {
  let in3 = thermo::convert::Convert::l_to_in3(1.0);
  let btu = thermo::convert::Convert::j_to_btu(650.0);
  let kw = thermo::convert::Convert::kw_to_ft_lb_per_s(0.135);
  println!("1.0 Liter = {} in³", in3);
  println!("650.0 J = {} BTU", btu);
  println!("0.135 ft-lb/s= {} KW", kw);
}
