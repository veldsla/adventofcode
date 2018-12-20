use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

struct Map(HashMap<(i32, i32), usize>);
impl Map {
    fn new() -> Map {
        let mut map = HashMap::new();
        map.insert((0, 0), 0);
        Map(map)
    }

    fn update_dist(&mut self, x: i32, y: i32, dist: usize) -> usize {
        if let Some(e) = self.0.get_mut(&(x,y)) {
            if *e > dist { 
                *e = dist;
                dist
            } else {
                *e
            }
        } else {
            self.0.insert((x, y), dist);
            dist
        }
    }

    fn dist(&self, x: i32, y: i32) -> usize {
        *self.0.get(&(x, y)).unwrap()
    }

    fn longest_route(&self) -> usize {
        *self.0.values().max().unwrap()
    }

    fn rooms_dist(&self, min: usize) -> usize {
        self.0.values().filter(|&&d| d >= min).count()
    }
}

fn make_map(s: &str) -> Map {
    let mut map = Map::new();
    make_routes(s, 0, 0, &mut map);
    map
}

fn make_routes(s: &str, mut x: i32, mut y: i32, map: &mut Map) {
    let mut chars = s.char_indices();
    let mut dist = map.dist(x, y);
    while let Some((pos, c)) = chars.next() {
        dist += 1;
        match c {
            '$'| '^' => {},
            'N' => y -=1,
            'S' => y +=1,
            'E' => x +=1,
            'W' => x -=1,
            '(' => {
                //now follows an '|' separated list of possibilities
                let mut open = 1;
                let mut start = pos+1;
                while let Some((pos, c)) = chars.next() {
                    match c {
                        '(' => open += 1,
                        ')' => open -= 1,
                        '|' if open == 1 => {
                            make_routes(&s[start..pos], x, y, map);
                            start = pos+1;
                        },
                        _ => {}
                    }
                    if open == 0 {
                        make_routes(&s[start..pos], x, y, map);
                        break;
                    }
                }
            }
            _ => panic!("main: {}", c)
        }
        dist = map.update_dist(x, y, dist);
    }
}

fn main() ->io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let s = s.trim();
    let map = make_map(s);
    println!("20a: Longest route through base is {} doors.", map.longest_route());
    println!("20b: There are {} rooms with distance > 1000.", map.rooms_dist(1000));
    Ok(())
}

#[test]
fn test() {
    assert_eq!(make_map("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").longest_route(), 18);
    assert_eq!(make_map("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").longest_route(), 23);
    assert_eq!(make_map("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").longest_route(), 31);
}
