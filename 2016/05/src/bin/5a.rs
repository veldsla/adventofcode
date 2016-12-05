extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

struct PasswordCharacterIterator<'a> {
    seed: &'a str,
    n: usize,
    hasher: Md5
}

impl<'a> PasswordCharacterIterator<'a> {
    fn new(s: &str) -> PasswordCharacterIterator {
        PasswordCharacterIterator {seed: s, n :0, hasher: Md5::new()}
    }
}

impl<'a> Iterator for PasswordCharacterIterator<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        loop {
            self.hasher.reset();
            self.hasher.input_str(format!("{}{}",self.seed,self.n).as_ref());
            self.n += 1;

            let res =  self.hasher.result_str();
            if res.chars().take(5).all(|c| c == '0') {
                return res.chars().nth(5);
            }
        }
    }
}

fn main() {
    let p = PasswordCharacterIterator::new("cxdnnyjw");
    println!("The password is: {}", p.take(8).collect::<String>());
}

#[test]
fn test() {
    let p = PasswordCharacterIterator::new("abc");
    assert_eq!(p.take(8).collect::<String>(), "18f47a30".to_string());
}
