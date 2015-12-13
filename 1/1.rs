use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("1_in.txt").unwrap();

    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data);
    let sum = data.iter().map(|u| match *u as char { '(' => 1, ')' => -1, _=> 0}).fold(0, |sum, x| sum + x); 
    println!("Santa is now at floor {}", sum);
}
