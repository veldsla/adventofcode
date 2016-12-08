use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;

struct Screen {
    pixels: Vec<bool>,
    width: usize,
    height: usize
}

enum Command {
    RotateRow((usize, usize)),
    RotateCol((usize, usize)),
    Rect((usize,usize))
}

impl From<String> for Command {
    fn from(s: String) -> Command {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "rotate" => {
                let rc = parts[2][2..].parse::<usize>().unwrap();
                let amount = parts[4].parse::<usize>().unwrap();
                match parts[1] {
                    "row" => Command::RotateRow((rc, amount)),
                    "column" => Command::RotateCol((rc, amount)),
                    _ => panic!("Error parsing command")
                }
            },
            "rect" => {
                let ab: Vec<_> = parts[1].split('x').map(|i| i.parse::<usize>().unwrap()).collect();
                Command::Rect((ab[0], ab[1]))
            },
            _ => panic!("Error parsing command")
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pixels.iter().enumerate().map(|p| {
            let n = p.0 > 0 && (p.0 + 1) % self.width == 0;
            match *p.1 {
                true if n => "#\n",
                false if n => ".\n",
                true  => "#",
                false => ".",
            }
        }).collect::<String>())
    }
}

impl Screen {
    fn new(w: usize, h: usize) -> Screen {
        Screen { width: w, height: h, pixels: vec![false; w * h] }
    }

    fn set_pixel(&mut self, x: usize, y: usize, to: bool) {
        self.pixels[y*self.width + x] = to;
    }

    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..a {
            for y in 0 ..b {
                self.set_pixel(x,y, true);
            }
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        let pos = row * self.width;
        assert!(row < self.height);
        let amount = self.width - amount % self.width;

        self.pixels[pos..pos+amount].reverse();
        self.pixels[pos+amount..pos+self.width].reverse();
        self.pixels[pos..pos+self.width].reverse();
    }

    fn rotate_col(&mut self, col: usize, amount: usize) {
        assert!(col < self.width);

        let amount = amount % self.height;
        let mut col_coords: Vec<usize> = (0..self.height).map(|r| r*self.width + col).collect();
        let coord_values: Vec<bool> = col_coords.iter().map(|&c| self.pixels[c]).collect();

        col_coords[..amount].reverse();
        col_coords[amount..].reverse();
        col_coords.reverse();

        for (c, v) in col_coords.into_iter().zip(coord_values.into_iter()) {
            self.pixels[c] = v;
        }
    }

    fn do_command<C: Into<Command>>(&mut self, command: C) {
        match command.into() {
            Command::RotateRow((row, n)) => self.rotate_row(row, n),
            Command::RotateCol((col, n)) => self.rotate_col(col, n),
            Command::Rect((a,b)) => self.rect(a, b),
        }
    }

    fn sum_lit(&self) -> usize{
        self.pixels.iter().fold(0, |acc, p| acc + if *p {1} else {0})
    }
}


fn main() {
    let mut s = Screen::new(50,6);

    let f = File::open("input.txt").unwrap();
    for l in BufReader::new(f).lines() {
        s.do_command(l.unwrap());
    }
    println!("Final screen:\n{}", s);
    println!("There are {} pixels lit", s.sum_lit());
}
