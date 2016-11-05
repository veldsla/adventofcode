use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64
}

impl Position {
    fn move_by_c(&mut self, c: char) {
        match c {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => {}
        };
    }
}

fn main() {

    //3a or 3b
    let both = true;

    let mut visited = HashSet::<Position>::new();

    let mut f = File::open("3_in.txt").unwrap();

    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data);


    let mut santa = Position {x: 0, y: 0};
    let mut robosanta = Position {x: 0, y: 0};

    visited.insert(santa);
    //no need to insert robosanta pos = same as santa

    let mut iter = data.iter();
    let mut pos = 1;
    loop {
        match iter.next() {
            Some(c) if both && pos % 2  == 0 => {
                robosanta.move_by_c(*c as char);
                visited.insert(robosanta);
            },
            Some(c) => {
                santa.move_by_c(*c as char);
                visited.insert(santa);
            },
            None => break
        };
        pos += 1;
    }

    println!("visited {} houses", visited.len());

}


