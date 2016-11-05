struct LookAndSay {
    string: String
}

impl LookAndSay {
    fn new(s: &str) -> LookAndSay {
        LookAndSay {string: s.to_string()}
    }

    fn next(&mut self) {
        let mut newstring = String::new();
        //read the string out load
        //take all same items
        {
            let mut it = self.string.chars();
        let mut c = it.next().unwrap();
        let mut count = 1;
        loop {
            let c2 = it.next();
            if c2.is_none() {
                newstring.push_str(self.say(c, count).as_ref());
                break;
            } else if c == c2.unwrap() {
                count += 1;
            } else {
                newstring.push_str(self.say(c, count).as_ref());
                c = c2.unwrap();
                count = 1;
            }
        }
        }
        self.string = newstring;
    }

    fn say(&self, c: char, n: usize) -> String {
        let mut s = n.to_string();
        s.push(c);
        s
    }
}


fn main() {
    let mut l = LookAndSay::new("1113222113");
    for i in 0..50 {
        l.next();
        println!("i={}, len={}", i+1, l.string.len());
    }
}

#[test]
fn looksay() {
    let mut l = LookAndSay::new("1");
    l.next();
    assert_eq!(l.string, "11".to_string());
    l.next();
    assert_eq!(l.string, "21".to_string());
    l.next();
    assert_eq!(l.string, "1211".to_string());
    l.next();
    assert_eq!(l.string, "111221".to_string());
    l.next();
    assert_eq!(l.string, "312211".to_string());
}
