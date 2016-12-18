#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Trap,
    Safe,
}

struct Row(Vec<Tile>);

impl Row {
    fn count_safe(&self) -> usize {
        let &Row(ref f) = self;
        f.iter().fold(0, |acc, x| acc + match x { &Tile::Trap => 0, &Tile::Safe => 1}) - 2
    }
}

impl From<String> for Row {
    fn from(s: String) -> Self {
        let mut tiles = vec![Tile::Safe];
        tiles.extend(s.chars().map(|c| {
            match c {
                '.' => Tile::Safe,
                '^' => Tile::Trap,
                _ => panic!("error in input")
            }
        }));
        tiles.push(Tile::Safe);
        Row(tiles)
    }
}

struct Floor {
    rows: Vec<Row>,
}

impl Floor {
    fn new(r: Row) -> Floor {
        Floor { rows: vec![r] }
    }

    fn generate_row(&mut self) {
        let mut next = vec![Tile::Safe];
        if let Some(&Row(ref f)) = self.rows.last(){
            for i in 1..f.len() - 1 {
                let t = match (&f[i-1], &f[i], &f[i+1]) {
                    (&Tile::Trap, &Tile::Trap, &Tile::Safe) => Tile::Trap,
                    (&Tile::Safe, &Tile::Trap, &Tile::Trap) => Tile::Trap,
                    (&Tile::Safe, &Tile::Safe, &Tile::Trap) => Tile::Trap,
                    (&Tile::Trap, &Tile::Safe, &Tile::Safe) => Tile::Trap,
                    (_, _, _) => {Tile::Safe}
                };
                next.push(t);
            }
        }
        //wall
        next.push(Tile::Safe);
        self.rows.push(Row(next));
    }

    fn count_safe(&self) -> usize {
        self.rows.iter().fold(0, |acc, x| acc + x.count_safe())
    }
}


fn main() {
    let mut f = Floor::new(Row::from(".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^".to_owned()));
    for _ in 0..39 {
        f.generate_row();
    }
    println!("The 40 x 40 floor has {} safe tiles", f.count_safe());
    for _ in 40..400000 {
        f.generate_row();
    }
    println!("The 400k x 400k floor has {} safe tiles", f.count_safe());

}

#[test]
fn test_1() {
    let mut f = Floor::new(Row::from(".^^.^.^^^^".to_owned()));
    for _ in 0..9 {
        f.generate_row();
    }
    assert_eq!(f.count_safe(), 38);
}
