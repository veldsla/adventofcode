use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

type Word = str;


trait SantaSays {
    fn is_nice(&self, blacklist: &Vec<&str>) -> bool;
    fn has_three_vowels(&self) -> bool;
    fn has_double_letter(&self) -> bool;
    fn has_no_forbidden(&self, blacklist: &Vec<&str>) -> bool;
}

impl SantaSays for Word {
    fn is_nice(&self, blacklist: &Vec<&str>) -> bool {
        self.has_three_vowels() &&
            self.has_double_letter() &&
            self.has_no_forbidden(blacklist)
    }

    fn has_three_vowels(&self) -> bool {
        let n_vowels = self.chars().map(|b| match b {
            'a'|'e'|'i'|'o'|'u' => 1,
            _ => 0
        }).fold(0, |sum, x| sum + x);

        n_vowels >= 3
    }

    fn has_double_letter(&self) -> bool {
        let b = self.chars().collect::<Vec<char>>();
        for i in 0 .. self.len()-1 {
            if b[i] == b[i+1] {
                return true;
            }
        }
        return false;
    }

    fn has_no_forbidden(&self, blacklist: &Vec<&str>) -> bool {
        for w in blacklist {
            if self.contains(w) {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    //5a
    let forbidden = vec!["ab", "cd", "pq", "xy"];

    let f = File::open("5_in.txt").unwrap();
    let reader = BufReader::new(f);

    let total = reader.lines().map(|l| {
        let line = l.ok().unwrap();
        match line.is_nice(&forbidden) {
            true => 1,
            false => 0
        }
    }).fold(0, |sum, x| sum + x);

    println!("Santa found {} nice words", total);


}

#[test]
fn three_vowels() {
    let nice = "ugknbfddgicrmopn";
    let naughty = "dvszwmarrgswjxmb";

    assert!(nice.has_three_vowels());
    assert_eq!(naughty.has_three_vowels(), false);
}

#[test]
fn double_letter() {
    let nice = "ugknbfddgicrmopn";
    let naughty = "jchzalrnumimnmhp";

    assert!(nice.has_double_letter());
    assert_eq!(naughty.has_double_letter(), false);
}

#[test]
fn forbidden() {
    let nice = "ugknbfddgicrmopn".to_string();
    let naughty = "haegwjzuvuyypxyu".to_string();

    let forbidden = vec!["ab", "cd", "pq", "xy"];

    assert!(nice.has_no_forbidden(&forbidden));
    assert_eq!(naughty.has_no_forbidden(&forbidden), false);
}
#[test]
fn nice() {
    let forbidden = vec!["ab", "cd", "pq", "xy"];

    assert!("ugknbfddgicrmopn".is_nice(&forbidden));
    assert!("aaa".is_nice(&forbidden));
    assert_eq!("jchzalrnumimnmhp".is_nice(&forbidden), false);
    assert_eq!("haegwjzuvuyypxyu".is_nice(&forbidden), false);
    assert_eq!("dvszwmarrgswjxmb".is_nice(&forbidden), false);
}
    
