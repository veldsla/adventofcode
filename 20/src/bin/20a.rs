#![feature(step_by)]
use std::process::exit;
fn main() {

    let n = 33100000;
    //worst case elve 3_310_000 delivers 33_100_000 presents at house 3_310_000
    let mut v = vec![10; 3_310_000];
    loop {
        for e in 2 .. 3_310_001 {
            for p in (e .. 3_310_000).step_by(e) {
                v[p-1] += e*10;
            }
            if v[e-1] > n {
                println!("House {} has more than {} presents({})", e, n, v[e-1]);
                exit(0);
            }
        }
    }
}
