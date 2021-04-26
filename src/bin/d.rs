use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        a: i64,
        b: i64,
        n: i64
    };

    // (0..9).for_each(|x| println!("{} {}", x, f(5,7,x)));

    if b == 1{
        println!("0");
        return
    }

    if n < b {
        println!("{}", f(a,b,n));
        return
    }
    else {
        println!("{}", f(a,b,b-1));
        return
    }

}

fn f(a:i64, b:i64, x:i64) -> i64{
    (a * x) / b - a * (x / b)
}