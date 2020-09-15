#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let res = 1_000_000_007;
    let mut ans = 1;
    let mut count = vec![0; 26];
    for &c in &s {
        count[c as usize - 'a' as usize] += 1;
    }
    for i in 0..26 {
        if count[i] != 0 {
            ans = ans * (count[i] + 1) % res as u64;
        }
    }
    ans -= 1;
    println!("{}", ans);
}
