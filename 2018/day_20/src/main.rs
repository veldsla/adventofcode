use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Map(HashMap<(i32, i32), usize>);
impl Map {
    fn new() -> Map {
        let map = HashMap::new();
        Map(map)
    }

    fn update_dist(&mut self, x: i32, y: i32, dist: usize) -> usize {
        if let Some(e) = self.0.get_mut(&(x,y)) {
            if *e >= dist { 
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

    fn dist(&self, x: i32, y: i32) -> Option<usize> {
        self.0.get(&(x, y)).cloned()
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
    make_routes(s, &mut map);
    map
}

fn make_routes(s: &str, map: &mut Map) {
    let mut current = VecDeque::new();
    current.push_back((0, 0, 0, s, ""));
    let mut visited = HashSet::new();

    while let Some((mut x, mut y, mut dist , s, remainder)) = current.pop_front() {
        println!("Make routes for {} starting at {},{} current dist = {}, remaining: {:?}", s, x, y, dist, remainder);
        let mut chars = s.char_indices();
        while let Some((pos, c)) = chars.next() {
            println!("{}", c);
            match c {
                '$'| '^' => {},
                'N' => {dist += 1; y -=1; dist = map.update_dist(x, y, dist);},
                'S' => {dist += 1; y +=1; dist = map.update_dist(x, y, dist);},
                'E' => {dist += 1; x +=1; dist = map.update_dist(x, y, dist);},
                'W' => {dist += 1; x -=1; dist = map.update_dist(x, y, dist);},
                '(' => {
                    //now follows an '|' separated list of possibilities
                    let mut open = 1;
                    let mut start = pos+1;
                    let mut alts = Vec::new();
                    while let Some((pos, c)) = chars.next() {
                        //println!("{}", c);
                        match c {
                            '(' => open += 1,
                            ')' => open -= 1,
                            '|' if open == 1 => {
                                alts.push(&s[start..pos]);
                                start = pos+1;
                            },
                            _ => {}
                        }
                        if open == 0 {
                            alts.push(&s[start..pos]);
                            break;
                        }
                    }
                    //now queue all the alts and attach the remainder
                    println!("founf alts: {:?}", alts);
                    alts.iter().for_each(|s| {
                        if !visited.contains(&(x, y)) || map.dist(x, y).unwrap() > dist {
                            println!("going alt {:?} from {},{}", s,x,y);
                            if s.is_empty() {
                            current.push_back((x, y, dist, chars.as_str(), ""));
                            } else {
                            current.push_back((x, y, dist, s, chars.as_str()));
                            }
                        }
                    });
                    break;
                }
                _ => panic!("main: {}", c)
            }
        }
        println!("before processing remainder {:?}, dist = {}, at {},{}", remainder, dist, x, y);
        if !remainder.is_empty() {
                println!("processing remainder {:?}, dist = {}, at {},{}", remainder, dist, x, y);
                current.push_back((x, y, dist, remainder, ""))
        } else {
        println!("Marking done {},{}", x, y);
        visited.insert((x,y));
        }
    }
}

fn main() ->io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let s = s.trim().trim_end_matches('$');
    let map = make_map(s);
    println!("20a: Longest route through base is {} doors.", map.longest_route());
    println!("20b: There are {} rooms with distance > 1000.", map.rooms_dist(1000));
    Ok(())
}

#[test]
fn test() {
    assert_eq!(make_map("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").longest_route(), 18);
    assert!(false);
    assert_eq!(make_map("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").longest_route(), 23);
    assert_eq!(make_map("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").longest_route(), 31);
    assert_eq!(make_map("^(N|E)(N|E)$").longest_route(), 2);
    assert_eq!(make_map("^(N|S|E|W)(N|S|E|W)(N|S|E|W)(N|S|E|W)(N|S|E|W)(N|S|E|W)(N|S|E|W)(N|S|E|W)$").longest_route(), 8);
}
