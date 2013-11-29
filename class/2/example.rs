fn outer<'l>(num: ~int) -> ~fn(int) -> int { |x|
//    *inner(num) + x
    *num + x
}

fn inner(num: ~int) -> ~int {
    num
}

fn add(num: ~int) -> ~fn(int) -> int { |x|
    *num + x
}

fn main() {
    let arg = ~3;
    let quadruple = outer(arg);
    print(fmt!("quadruple %?", quadruple));
}
