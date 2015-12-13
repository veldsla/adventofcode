struct Password {
    pw: [char; 8]
}
impl Password {
    fn new(s: &str) -> Password {
        let mut pw = ['a'; 8];
        for (i,c) in s.chars().enumerate() {
            pw[i] = c;
        }
        Password {pw: pw}
    }

    fn next(&mut self) {
        loop {
            for i in (0..8).rev() {
                let new = match self.pw[i] {
                    'z' => 'a',
                    'h' => 'j',
                    'k' => 'm',
                    'n' => 'p',
                     x  => (x as u8 + 1 ) as char
                };
                self.pw[i] = new;
                if new != 'a' {
                    break
                }
            }
            if self.is_valid() {
                break
            }
        }

    }

    fn is_valid(&self) -> bool {
        !self.has_i() && self.inc_straight() && self.two_pairs()
    }

    fn has_i(&self) -> bool {
        for i in 0 .. 8 {
            if self.pw[i] == 'i' {
                return true;
            }
        }
        false
    }

    fn inc_straight(&self) -> bool {
        for i in 0..5 {
            let v1 = self.pw[i] as u8;
            let v2 = self.pw[i+1] as u8;
            let v3 = self.pw[i+2] as u8;

            if v2 + 1 == v3 && v1 + 1  == v2 {
                return true
            }
        }
        false
    }

    fn two_pairs(&self) -> bool {
        let mut pairs = 0;
        let mut i = 0;
        while  i < 7 {
            if self.pw[i] == self.pw[i+1] {
                pairs+=1;
                i += 2;
            } else {
                i += 1;
            }
        }
        pairs >= 2
    }
}

fn main() {
    let mut pw = Password::new("hepxcrrq");
    pw.next();
    println!("Santa's new password = {}", pw.pw.iter().map(|c| {c.to_string() }).collect::<String>());
    pw.next();
    println!("Santa's newer password = {}", pw.pw.iter().map(|c| {c.to_string() }).collect::<String>());
}

#[test]
fn inc() {
    let pw = Password::new("hijklmmn");
    assert_eq!(pw.inc_straight(), true);
    assert_eq!(pw.two_pairs(), false);
    let pw2 = Password::new("abbceffg");
    assert_eq!(pw2.two_pairs(), true);
    let mut pw3 = Password::new("abcdefgh");
    pw3.next();
    assert_eq!(pw3.pw, ['a', 'b', 'c', 'd', 'f', 'f', 'a', 'a']);
    let mut pw4 = Password::new("ghjaaaaa");
    pw4.next();
    assert_eq!(pw4.pw, ['g','h','j','a','a','b','c','c']);
}
