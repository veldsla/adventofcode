extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

struct PasswordGenerator<'a> {
    seed: &'a str,
    hasher: Md5,
}

impl<'a> PasswordGenerator<'a> {
    fn new(seed: &str) -> PasswordGenerator {
        PasswordGenerator {seed: seed, hasher: Md5::new()}
    }

    fn generate(&mut self) -> String {
       let mut n = 0;
       let mut password = vec![None; 8];
        loop {
            self.hasher.reset();
            self.hasher.input_str(format!("{}{}",self.seed,n).as_ref());
            n += 1;

            let rs =  self.hasher.result_str();
            if rs.chars().take(5).all(|c| c == '0') {
                let c = rs.chars().nth(5).unwrap();
                let pos = (c as u8 - 48) as usize;
                if c >= '0' && c <= '7' && password[pos].is_none() {
                    password[pos] = rs.chars().nth(6);
                }
            }
            if password.iter().all(|c| c.is_some()) {
                return password.iter().map(|c| c.unwrap()).collect();
            }
        }
    }
}

fn main() {
    let mut p = PasswordGenerator::new("cxdnnyjw");
    println!("The password is: {}", p.generate());
}

#[test]
fn test() {
    let mut p = PasswordGenerator::new("abc");
    assert_eq!(p.generate(), "05ace8e3".to_string());
}
