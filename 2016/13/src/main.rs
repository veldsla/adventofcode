use std::collections::HashSet;
use std::mem;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    x: u32,
    y: u32,
    magic: u32,
}

impl Position {
    fn directions(&self) -> Vec<Position> {
        [self.left(), self.right(), self.up(), self.down()].into_iter()
            .filter(|d| d.is_some()).map(|d| d.unwrap()).filter(|d| !d.is_wall() && d != self).collect()
    }

    fn left(&self) -> Option<Position>{
        if self.x > 0 {
            let mut new = *self;
            new.x -= 1;
            Some(new)
        } else {
            None
        }
    }

    fn up(&self) -> Option<Position>{
        if self.y > 0 {
            let mut new = *self;
            new.y -= 1;
            Some(new)
        } else {
            None
        }
    }

    fn down(&self) -> Option<Position>{
            let mut new = *self;
            new.y += 1;
            Some(new)
    }

    fn right(&self) -> Option<Position>{
            let mut new = *self;
            new.x += 1;
            Some(new)
    }

    fn is_wall(&self) -> bool {
        let x = self.x;
        let y = self.y;
        let value  = x*x + 3*x + 2*x*y + y + y*y;
        (value + self.magic).count_ones() % 2 != 0
    }

}

fn find_path(start: Position, destination: Position) -> u32 {
    let mut queue = Vec::new();
    let mut next_queue = Vec::new();
    let mut seen = HashSet::new();
    queue.push(start);
    let mut moves = 0;
    loop {
        while let Some(pos) = queue.pop() {
            if pos == destination {
                return moves;
            }
            seen.insert((pos.x, pos.y));
            let next = pos.directions().into_iter().filter(|p| !seen.contains(&(p.x, p.y)));
            next_queue.extend(next);
        }
        moves += 1;
        mem::swap(&mut queue, &mut next_queue)
    }
}

fn count_steps(start: Position) -> usize {
    let mut queue = Vec::new();
    let mut next_queue = Vec::new();
    let mut seen = HashSet::new();
    seen.insert((start.x, start.y));
    queue.push(start);
    let mut moves = 0;
    loop {
        if moves == 51 {
            break;
        }
        while let Some(pos) = queue.pop() {
            seen.insert((pos.x, pos.y));
            let next = pos.directions().into_iter().filter(|p| !seen.contains(&(p.x, p.y)));
            next_queue.extend(next);
        }
        moves += 1;
        mem::swap(&mut queue, &mut next_queue)
    }
    seen.len()
}

fn main() {
    let start = Position {x: 1, y: 1, magic: 1362};
    let destination = Position {x: 31, y: 39, magic: 1362};
    let moves = find_path(start, destination);
    println!("Going to x: 31, y: 39 took {} moves", moves);
    println!("We can reach {} positions with at most 50 steps", count_steps(start));

}

#[test]
fn wall() {
    let p = Position { x: 0, y: 0, magic: 10};
    assert!(!p.is_wall());
    let p = Position { x: 9, y: 9, magic: 10};
    assert!(p.is_wall());
}

#[test]
fn directions() {
    let p = Position { x: 0, y: 0, magic: 10};
    assert_eq!(p.directions(), vec![Position { x: 0, y: 1, magic: 10 }] );
    let p = Position { x: 3, y: 3, magic: 10};
    assert_eq!(p.directions(),vec![Position { x: 3, y: 2, magic: 10 }, Position { x: 3, y: 4, magic: 10 }]);
}

#[test]
fn path() {
    assert_eq!(find_path( Position {x: 1, y: 1, magic: 10}, Position {x: 7, y: 4, magic: 10}), 11)
}

