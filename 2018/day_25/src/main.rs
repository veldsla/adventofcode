use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn dist(p1: &(i32, i32, i32, i32), p2: &(i32, i32, i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() +
        (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
}

fn constellations(mut v: Vec<(i32, i32, i32, i32)>) -> usize {
    let mut cst = 0;;
    while let Some(start) = v.pop() {
        let mut ncst = vec![start];
        loop {
            let oldv = v.len();
            v.retain(|c| {
                if ncst.iter().map(|t| dist(c,t)).min().unwrap() < 4 {
                    ncst.push(c.clone());
                    false
                } else {
                    true
                }
            });
            if oldv == v.len() { break; }
        }
        cst += 1;
    }
    cst
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let points: Vec<_> = BufReader::new(f).lines()
        .map(|l| {
            l.and_then(|l| {
                let mut n = l.split(',');
                let x = n.next().unwrap().parse().unwrap();
                let y = n.next().unwrap().parse().unwrap();
                let z = n.next().unwrap().parse().unwrap();
                let t = n.next().unwrap().parse().unwrap();
                Ok((x, y, z, t))
            })
        }).collect::<Result<Vec<_>, _>>()?;
    println!("The number of constellations is {}", constellations(points));

    Ok(())
}

#[test]
fn test() {
    let v = vec![
        (-1,2,2,0),
        (0,0,2,-2),
        (0,0,0,-2),
        (-1,2,0,0),
        (-2,-2,-2,2),
        (3,0,2,-1),
        (-1,3,2,2),
        (-1,0,-1,0),
        (0,2,1,-2),
        (3,0,0,0)];
    assert_eq!(constellations(v), 4);

    let v = vec![
        (1,-1,0,1),
        (2,0,-1,0),
        (3,2,-1,0),
        (0,0,3,1),
        (0,0,-1,-1),
        (2,3,-2,0),
        (-2,2,0,0),
        (2,-2,0,-1),
        (1,-1,0,-1),
        (3,2,0,2)];
    assert_eq!(constellations(v), 3);


    let v = vec![
        (1,-1,-1,-2),
        (-2,-2,0,1),
        (0,2,1,3),
        (-2,3,-2,1),
        (0,2,3,-2),
        (-1,-1,1,-2),
        (0,-2,-1,0),
        (-2,2,3,-1),
        (1,2,2,0),
        (-1,-2,0,-2)];
    assert_eq!(constellations(v), 8);


}
