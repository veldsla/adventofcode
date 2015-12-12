use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn escaped_len(s: &str) -> usize {
    let mut it = s.chars();
    let mut count = 0;
    loop {
        match it.next() {
            None => break,
            Some('\\') |  Some('"') => { count += 2},
            _   => count +=1
        }
    }
    //add two for enclosing "
    count + 2
}

fn main() {
    let f = File::open("8_in.txt").unwrap();
    let reader = BufReader::new(f);

    let total = reader.lines().map(|l| {
        let line = l.ok().unwrap();
        escaped_len(line.as_ref()) - line.len()
    }).fold(0, |sum, x| sum + x);

    println!("Difference escaped - original is {}", total);
}


#[test]
fn len() {
    assert_eq!(escaped_len("\"\""), 6);
    assert_eq!(escaped_len("\"abc\""), 9);
    assert_eq!(escaped_len("\"aaa\\\"aaa\""), 16);
    assert_eq!(escaped_len("\"\\x27\""), 11);
}
