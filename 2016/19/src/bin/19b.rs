struct LinkedElves {
    elves: Vec<Elf>,
    in_game: usize,
    across: usize
}

impl LinkedElves {
    fn new(n: usize) -> LinkedElves {
        let e = (0..n).map(|i| Elf {id: i+ 1, next: (i+1) % n, prev: if i> 0 {i-1} else {n-1} }).collect();
        LinkedElves {elves: e, across: n/2, in_game: n}
    }

    fn play_wep(&mut self) -> &Elf {
        let mut current_elf = 0;
        while self.in_game > 1 {
            //remove the elf marked across
            let prev = self.elves[self.across].prev;
            let next = self.elves[self.across].next;
            self.elves[prev].next = next;
            self.elves[next].prev = prev;
           
            self.in_game -= 1;

            //move across 1 node to the right
            self.across = next;
            //but if the length is now even it moves 2 places
            if self.in_game % 2 == 0 {
                self.across = self.elves[next].next;
            }
            current_elf = self.elves[current_elf].next;
        }
        &self.elves[current_elf]
    }
}

struct Elf {
    id: usize,
    prev: usize,
    next: usize
}


fn main() {
    let mut l = LinkedElves::new(3012210);

    println!("Happy elf is nr: {:?}", l.play_wep().id);
}
