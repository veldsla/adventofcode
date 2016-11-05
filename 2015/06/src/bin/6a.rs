use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Clone, Copy)]
enum Status {
    On,
    Off
}

#[derive(Clone, Copy)]
enum ChangeLight {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Clone, Copy)]
struct Light {
    status: Status
}

impl Light {
    fn new() ->Light {
        Light {status: Status::Off }
    }

    fn is_on(&self) -> bool {
        match self.status {
            Status::On => true,
            Status::Off => false
        }
    }

    fn turn_on(&mut self) {
        self.status = Status::On;
    }

    fn turn_off(&mut self) {
        self.status = Status::Off;
    }

    fn toggle(&mut self) {
        match self.status {
            Status::On => self.status = Status::Off,
            Status::Off => self.status = Status::On
        }
    }

    fn change(&mut self, change: ChangeLight) {
        match change {
            ChangeLight::TurnOn  => self.turn_on(),
            ChangeLight::TurnOff => self.turn_off(),
            ChangeLight::Toggle  => self.toggle()
        };
    }
}

struct Grid {
    lights: [ [Light; 1000]; 1000]
}

impl Grid {
    fn new() -> Grid {
        Grid {lights: [[Light::new(); 1000]; 1000]}
    }

    fn num_on(&self) -> u64 {
        let mut sum =0;
        for r in 0..1000 {
            for l in 0..1000 {
                if self.lights[r][l].is_on() {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn change(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, change: ChangeLight) {
        for r in x1..x2 +1 {
            for c in y1..y2+1 {
                self.lights[r][c].change(change);
            }
        }
    }

    fn process_command(&mut self, s: &str) {
        //command structure:
        //turn <on|off> x1,y1 through x2,y2
        let v = s.split(" ").collect::<Vec<&str>>();
        let mut it = v.iter();
        let change = match it.next() {
            Some(&"toggle") => ChangeLight::Toggle,
            Some(&"turn")   => {
                match it.next() {
                    Some(&"on") => ChangeLight::TurnOn,
                    Some(&"off") => ChangeLight::TurnOff,
                    _ => panic!("Error parsing command")
                }
            },
            _ => panic!("Error parsing command")
        };

        let x1y1 = it.next().unwrap();
        assert_eq!(it.next(), Some(&"through"));
        let x2y2 = it.next().unwrap();
        let xy1 = x1y1.split(",").map(|s| { s.parse::<usize>().unwrap() }).collect::<Vec<usize>>();
        let xy2 = x2y2.split(",").map(|s| { s.parse::<usize>().unwrap() }).collect::<Vec<usize>>();

        self.change(xy1[0],xy1[1],xy2[0],xy2[1], change);
    }
}


fn main() {
    let f = File::open("6_in.txt").unwrap();
    let reader = BufReader::new(f);

    let mut g = Grid::new();
    for l in reader.lines() {
        let line = l.ok().unwrap();
        g.process_command(line.as_ref());
    }

    println!("There are {} lights on", g.num_on());
}

#[test]
fn test_grid() {
    let mut g = Grid::new();
    assert_eq!(g.num_on(), 0);
    g.change(499,499,500,500, ChangeLight::TurnOn);
    assert_eq!(g.num_on(), 4);
    g.change(0,0,0,999, ChangeLight::Toggle);
    assert_eq!(g.num_on(), 1004);
    g.change(0,600,999,600, ChangeLight::TurnOff);
    assert_eq!(g.num_on(), 1003);
}

#[test]
fn test_commands() {
    let mut g = Grid::new();

    g.process_command("turn on 499,499 through 500,500");
    assert_eq!(g.num_on(), 4);
    g.process_command("turn on 599,989 through 806,993");
    g.process_command("turn off 370,39 through 425,839");
}

