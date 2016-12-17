extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::HashMap;

struct OtpIterator<'a> {
    seed: &'a str,
    n: usize,
    stretching: usize,
    stretch_cache: HashMap<String, String>,
    hasher: Md5
}

impl<'a> OtpIterator<'a> {
    fn new(s: &str, stretch: usize) -> OtpIterator {
        OtpIterator {seed: s, n :0, stretching: stretch, stretch_cache: HashMap::new(), hasher: Md5::new()}
    }
}

impl<'a> Iterator for OtpIterator<'a> {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        let mut triplet_at = 0;
        let mut triplet_char: Option<u8> = None;


        loop {

            let input_str = format!("{}{}",self.seed,self.n);
            let mut to_hash = input_str.clone();

            if self.stretch_cache.contains_key(&to_hash) {
                to_hash = self.stretch_cache.get(&to_hash).unwrap().clone();
            } else {
                let n_hashes = self.stretching + 1;
                for _ in 0..n_hashes {
                    self.hasher.reset();
                    self.hasher.input_str(&to_hash);
                    to_hash = self.hasher.result_str();
                }
                self.stretch_cache.insert(input_str, to_hash.clone()); 
            }

            self.n += 1;

            let resb = to_hash.as_bytes();

            match triplet_char {
                Some(c) => {
                    if self.n - triplet_at > 1000 {
                        //give up
                        self.n = triplet_at;
                        triplet_char = None;
                        continue;
                    } else {
                        //macth 5 c chars
                        let hit5 = resb.windows(5).any(|w| w[0] == c && w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4]);
                        if hit5 {
                            self.n = triplet_at;
                            return Some((triplet_at -1 , to_hash.clone()));
                        } else {
                            continue;
                        }
                    }
                },
                None => {
                    //match 3 chars
                    if resb.windows(3).any(|w| {
                        if w[0] == w[1] && w[1] == w[2] {
                            triplet_char = Some(w[0]);
                            true
                        } else {
                            false
                        }}) {
                        triplet_at = self.n;
                        continue;
                    }
                }
            }
        }
    }
}



fn main() {
    let mut p = OtpIterator::new("yjdafjpo", 0);
    let otp_64 = p.nth(63).unwrap();
    println!("Index {} produces the 64 password: {}", otp_64.0, otp_64.1 );
    let stretched_otp_64 = OtpIterator::new("yjdafjpo", 2016).nth(63).unwrap();
    println!("Using 2016x key stretching Index {} produces the 64 password: {}", stretched_otp_64.0, stretched_otp_64.1 );
}

#[test]
fn test() {
    let mut p = OtpIterator::new("abc", 0);
    let otp_64 = p.nth(63).unwrap();
    assert_eq!(otp_64.0, 22728);
}
#[test]
fn test_2() {
    let mut p = OtpIterator::new("abc", 2016);
    let otp_64 = p.nth(63).unwrap();
    assert_eq!(otp_64.0, 22551);
}

