
struct Numgen {
    n: u64
}

impl Iterator for Numgen {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.n = (self.n * 252533) % 33554393;
        Some(self.n)
    }
}

impl Numgen {
    fn coord_to_pos(&self, row: usize, col: usize) -> usize {
        let mut add = 1;
        let mut pos = 1;
        for i in 1..row {
            pos += add;
            add += 1;
        }
        add+=1;
        for i in 1..col {
            pos +=add;
            add += 1;
        }
        pos
    }
}

fn main() {
    //code at 1 is 20151125
    let mut n = Numgen {n : 20151125};
    //To continue, please consult the code grid in the manual.  Enter the code at row 2947, column 3029.
    let pos = n.coord_to_pos(2947,3029);
    //use pos - 2  because the first number is already generated and nth skips n iterations
    let pw = n.nth(pos-2).unwrap();
    println!("Your code is: {}", pw);
}

#[test]
fn test() {
    let mut n = Numgen {n : 20151125};
    assert_eq!(n.coord_to_pos(6,1), 16);
    assert_eq!(n.coord_to_pos(1,6), 21);
    assert_eq!(n.coord_to_pos(3,3), 13);
    assert_eq!(n.coord_to_pos(2,4), 14);
    assert_eq!(n.coord_to_pos(3,4), 19);

    assert_eq!(n.nth(19-2), Some(7981243));
}

#[test]
fn test_it() {
    let mut n = Numgen {n : 20151125};
    assert_eq!(n.next(), Some(31916031));
    assert_eq!(n.next(), Some(18749137));
    assert_eq!(n.next(), Some(16080970));
    assert_eq!(n.next(), Some(21629792));
}

