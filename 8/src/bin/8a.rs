use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn nchar(s: &str) -> usize {
    let mut it = s.chars();
    let mut count = 0;
    loop {
        match it.next() {
            None => break,

            Some('\\') => {
                //single escape
                match it.next() {
                    Some('x') => {
                        //discard the 2 following count 1
                        it.next(); it.next();
                        count += 1;
                    },
                    Some('\\') | Some('"') => {
                        //escaped backslash or " count 1
                        count += 1;
                    },
                    _ => panic!("Unknown escape sequence")
                }
            }
            Some('"') => {},
            _   => count +=1
        }
    }
    count
}

fn main() {
    let f = File::open("8_in.txt").unwrap();
    let reader = BufReader::new(f);

    let total = reader.lines().map(|l| {
        let line = l.ok().unwrap();
        line.len() - nchar(line.as_ref())
    }).fold(0, |sum, x| sum + x);

    println!("Difference code - memory is {}", total);
}


#[test]
fn len() {
    assert_eq!(nchar("\"\""), 0);
    assert_eq!(nchar("abc"), 3);
    assert_eq!(nchar("aaa\\\"aaa"), 7);
    assert_eq!(nchar("hello\\x27there"), 11);
}
