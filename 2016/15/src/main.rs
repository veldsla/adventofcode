#[derive(Clone)]
struct Disc {
    n_pos: usize,
    start: usize,
    i: usize
}

impl Disc {
    fn pos_after(&self, t: usize) -> usize {
        (self.start + t) % self.n_pos
    }

    fn first_aligned_after(&self) -> usize {
        let aligned_at = self.n_pos - (self.i  % self.n_pos) - 1;

        if aligned_at >= self.start {
            aligned_at - self.start
        } else {
            aligned_at + self.n_pos -  self.start
        }
    }

    fn is_aligned(&self, t: usize) ->  bool {
        self.pos_after(t) == 0
    }
}

struct Sculpture {
    discs: Vec<Disc>,
}

impl Sculpture {
    fn new() -> Sculpture {
        Sculpture { discs: Vec::new() }
    }

    fn add_disc(&mut self, npos: usize, start: usize) {
        let last = self.discs.len();
        self.discs.push(Disc { n_pos: npos, start: start, i: last });
    }

    fn press_at(&mut self) -> usize {
        let mut d = self.discs.clone();

        d.sort_by(|a, b| a.n_pos.cmp(&b.n_pos).reverse());
        //determine the time when the largest disc would be aligned for complete fall-through
        let mut time = d[0].first_aligned_after();
        loop {
            if aligned(&d, time, 1) {
                return time;
            } else {
                time += d[0].n_pos;
            }
        }
    }

}

fn aligned(discs: &Vec<Disc>, time: usize, current_disc: usize) -> bool {
    if current_disc == discs.len() {
        false
    } else {
        match discs[current_disc].is_aligned(time + discs[current_disc].i +1) {
            true if current_disc == discs.len()-1 => true,
            true => aligned(discs, time, current_disc+1),
            false => false
        }
    }
}

fn main() {
    let mut s = Sculpture::new();
    s.add_disc(7,0);
    s.add_disc(13,0);
    s.add_disc(3,2);
    s.add_disc(5,2);
    s.add_disc(17,0);
    s.add_disc(19,7);
    println!("Press the button at time = {}", s.press_at());
    s.add_disc(11,0);
    println!("Press the button at time = {}", s.press_at());
}

#[test]
fn test() {
    let mut s = Sculpture::new();
    s.add_disc(5,4);
    s.add_disc(2,1);

    assert_eq!(s.press_at(), 5);
}
