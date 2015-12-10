use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

type Word = str;


trait SantaSaysB {
    fn is_nice(&self) -> bool;
    fn has_dup_pair(&self) -> bool;
    fn has_xnx(&self) -> bool;
}

impl SantaSaysB for Word {
    fn is_nice(&self) -> bool {
        self.has_dup_pair() &&
            self.has_xnx()
    }

    fn has_dup_pair(&self) -> bool {

        for pos in 0 .. self.len()-1 {
            let pattern :String = self.chars().skip(pos).take(2).collect();
            //we can safely unwrap as we always find the source
            //pos + 1 because the patterns may not overlap
            if self.rfind(&pattern).unwrap() > pos + 1 {
                return true
            }
        }
        false
    }

    fn has_xnx(&self) -> bool {
        let b = self.chars().collect::<Vec<char>>();
        for i in 0 .. self.len()-2 {
            if b[i] == b[i+2] {
                return true;
            }
        }
        false
    }
}

fn main() {
    //5a

    let f = File::open("5_in.txt").unwrap();
    let reader = BufReader::new(f);

    let total = reader.lines().map(|l| {
        let line = l.ok().unwrap();
        match line.is_nice() {
            true => 1,
            false => 0
        }
    }).fold(0, |sum, x| sum + x);

    println!("Santa found {} nice words", total);


}

#[test]
fn nice() {
    assert!("qjhvhtzxzqqjkmpb".is_nice());
    assert!("xxyxx".is_nice());
    assert_eq!("uurcxstgmygtbstg".is_nice(), false);
    assert_eq!("ieodomkazucvgmuy".is_nice(), false);
}
    
