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
        x: i64
    };

    let mut money = 100i64;

    let mut ans = 0;

    loop {
        money += money/100;
        ans += 1;
        if money >= x {
            println!("{}", ans);
            return
        }
    }
}
