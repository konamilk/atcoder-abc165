use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use itertools::Itertools;

fn main() {
    let source = AutoSource::from("3 4 3
1 3 3 100
1 2 2 10
2 3 2 10");
    input!{
        // from source,
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize, usize, usize, i32);q]
    };

    let mut max_score = 0;

    for A in (1..=m).combinations_with_replacement(n) {
        let mut score = 0;
        for (a, b, c, d) in abcd.clone() {
            if A[b-1] - A[a-1] == c {
                score += d
            }
        }
        // println!("{:?} {}",A, score);
        max_score = max(score, max_score);
    }

    println!("{}", max_score);
}
