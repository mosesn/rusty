use std::os;
use std::from_str;

fn comb(a: ~fn(int) -> int, b: ~fn(int) -> int) -> ~fn(int) -> int { |x|
  a(b(x))
}

fn more(fns: &[~fn(int) -> int]) -> ~fn(int) -> int {
    let mut tmp: ~fn(int) -> int = |x| x;
    for &f in fns.iter() {
        tmp = comb(tmp, f);
    }
    tmp
}

fn main() {
    let num = 3;
//    let quadruple = ntimes(double, 2);
//    print(fmt!("quadruple %?", quadruple(num)));
}
