use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("1_in.txt").unwrap();

    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data);
    let mut level = 0;
    for (i, c) in data.iter().enumerate() {
        level += match *c as char { '(' => 1, ')' => -1, _=> 0};
        if level == -1 {
            println!("Basement at char {}", i+1);
            return;
        }
    }

    println!("Santa never gets to basement");
}
