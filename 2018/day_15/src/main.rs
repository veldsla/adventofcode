use std::collections::{VecDeque, HashSet, HashMap};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum MobType {
    Elf,
    Goblin,
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Mob {
    pos: usize,
    mobtype: MobType,
    hp: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    //refers to mob index in mobs Vec
    Occupied(MobType, usize),
}

#[derive(Debug)]
struct Cave {
    map: Vec<Cell>,
    width: usize,
    height: usize,
    mobs: Vec<Mob>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.map.chunks(self.width) {
            writeln!(f, "{}", c.iter().map(|e| {
                match e {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Occupied(mobtype, _) => {
                        match mobtype {
                            MobType::Elf => 'E',
                            MobType::Goblin => 'G',
                        }
                    }
                }
            }).collect::<String>())?
        }
        Ok(())
    }
}
impl fmt::Display for MobType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MobType::Elf => write!(f, "Elves"),
            MobType::Goblin => write!(f, "Goblins")
        }
    }
}

impl Cave {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Cave> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let mut map = Vec::new();
        let mut height = 0;
        let mut width = 0;
        let mut mobs = Vec::new();
        for l in b.lines() {
            let l = l?;
            let l = l.trim_end_matches('\n');
            width = l.len();
            map.extend(
                l.chars().enumerate().map(|(x, c)| {
                    match c {
                        'G' => {
                            mobs.push( Mob {pos: height*width + x, hp: 200, mobtype: MobType::Goblin });
                            Cell::Occupied(MobType::Goblin, mobs.len() - 1)
                        },
                        'E' => {
                            mobs.push( Mob {pos: height*width + x, hp: 200, mobtype: MobType::Elf });
                            Cell::Occupied(MobType::Elf, mobs.len() - 1)
                        },
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        _ => panic!("error in maze")
                    }
                })
            );
            height += 1;
        }
        Ok(Cave{ map, mobs, width, height })
    }

    fn fight(&mut self, elf_damage: u32) -> bool {
        // order players on board position
        let mut players: Vec<_> = self.map.iter()
            .rev()
            .cloned()
            .filter_map(|m| {
                if let Cell::Occupied(_, pos) = m {
                    if self.mobs[pos].is_dead() {
                        None
                    } else {
                       Some(pos)
                    }
                } else {
                    None
                }
            })
            .collect();

        //each player takes turn
        let mut moved = false;
        while let Some(player) = players.pop() {
            let mytype = self.mobs[player].mobtype;
            let mypos = self.mobs[player].pos;
            if let Some((attack_pos, target_pos, dist, target_mob, hp, to)) = self.mobs[player].route(self) {
                //println!("do move attack pos = {}, target at {}, mobidx {} hp: {}, mov = {}, ", attack_pos, target_pos, target_mob, hp, to);
                moved = true;
                if dist >= 1 {
                    self.map[mypos] = Cell::Empty;
                    self.map[to] = Cell::Occupied(mytype, player);
                    self.mobs[player].pos = to;
                }
                if dist <=1 {
                    // fight the mob
                    self.mobs[target_mob].damage(if mytype == MobType::Elf { elf_damage } else { 3 });
                    if self.mobs[target_mob].is_dead() {
                        //println!("player {} died", target_mob);
                        players.retain(|&p| p!= target_mob);
                        self.map[target_pos] = Cell::Empty;
                        //add test that if a winner is present and this is not the last player to
                        //not count this round
                        if !players.is_empty() && (
                            self.mobs.iter().filter(|m| m.mobtype == MobType::Elf).all(|elf| elf.is_dead()) || 
                            self.mobs.iter().filter(|m| m.mobtype == MobType::Goblin).all(|gobl| gobl.is_dead()))
                        {
                            return false;
                        }
                    }
                }
            } else {
                //println!("player {} cannot move", player);
            }
        }
        moved
    }

    fn to_the_death(&mut self, elf_damage: u32) -> (usize, MobType, u32) {
        let mut round = 0;
        while self.fight(elf_damage) {
            round += 1;
            //println!("round {}", round);
            println!("{}", self);
        }
        //println!("cave:{} {:?}", round, self.mobs);
        let winners = self.mobs.iter().find(|m| !m.is_dead()).unwrap().mobtype;
        let sum_hp = self.mobs.iter()
            .filter(|m| !m.is_dead())
            .map(|m| m.hp).sum::<u32>();

        (round, winners, sum_hp)
    }


    fn moves(&self, pos: usize) -> Vec<usize> {
        // reading order, Up Left Right Down
        let mut directions = Vec::new();
        if pos >= self.width {
            directions.push(pos - self.width);
        }
        if pos > 0 && pos % self.width != 0 {
            directions.push(pos - 1);
        }
        if pos < self.map.len() && (pos + 1) % self.width != 0 {
            directions.push(pos + 1);
        }
        if pos < self.map.len() - self.width {
            directions.push(pos + self.width);
        }
        directions
    }
}

impl Mob {
    fn is_dead(&self) -> bool {
        self.hp == 0
    }

    fn damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }

    fn enemy(&self) -> MobType {
        match self.mobtype {
            MobType::Elf => MobType::Goblin,
            MobType::Goblin => MobType::Elf
        }
    }

    fn is_enemy(&self, other: &Mob) -> bool {
        self.mobtype == MobType::Elf && other.mobtype == MobType::Goblin ||
            self.mobtype == MobType::Goblin && other.mobtype == MobType::Elf
    }

    fn route(&self, map: &Cave) -> Option<(usize, usize, usize, usize, u32, usize)> {
        //simple bfs
        //visited doubles as a distance matrix
        let mut visited = vec![None; map.map.len()];
        
        let mut queue = VecDeque::new();
        let mut next_queue = VecDeque::new();
        let mut routes = HashMap::new();

        //start at the source position
        //println!("start at {}", self.pos);
        next_queue.push_back((self.pos, 0));

        let mut reachable = Vec::new();

        while !next_queue.is_empty() && reachable.is_empty() {
            std::mem::swap(&mut next_queue, &mut queue);
            while let Some((pos, d)) = queue.pop_front() {
                visited[pos] = Some(d);
                //get the moves in reading order
                for to in map.moves(pos).into_iter() {
                    match map.map[to] {
                        Cell::Occupied(_, mobidx) => {
                            if map.mobs[mobidx].is_enemy(self) {
                                reachable.push((pos, to, d + 1, mobidx, map.mobs[mobidx].hp));
                            }
                        },
                        Cell::Empty => {
                            if visited[to].is_none() {
                                next_queue.push_back((to, d+1));
                                visited[to] = Some(d + 1);
                                routes.insert(to, pos);
                            }
                        },
                        Cell::Wall => {}
                    }
                }
            }
        }
        //println!("reachable {:?}", reachable);
        //println!("visited {:?}", visited);
        //println!("routes {:?}", routes);
        if reachable.is_empty() {
            return None
        }
        //all are reachable by the same distance
        //select the lowest target by position
        let mut target_pos = reachable.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let dist = visited[target_pos].unwrap();
        //println!("go to {} dist = {}", target_pos, dist);

        if dist == 0 {
            //don't move, attack the lowest hp;
            let enemy = map.moves(self.pos).into_iter().filter_map(|p| {
                if let Cell::Occupied(_, mobidx) = map.map[p] {
                    if map.mobs[mobidx].is_enemy(self) {
                        Some((mobidx, &map.mobs[mobidx]))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).min_by_key(|mob| mob.1.hp).unwrap();

            Some((target_pos, enemy.1.pos, 0, enemy.0, enemy.1.hp, self.pos))
        } else if dist == 1{
            //make a move
            //now take a step that reduces the distance, break ties by position
            while visited[target_pos] != Some(1) {
                target_pos = *routes.get(&target_pos).expect("barf");
            }
            // find the attack
            let enemy = map.moves(target_pos).into_iter().filter_map(|p| {
                if let Cell::Occupied(_, mobidx) = map.map[p] {
                    if map.mobs[mobidx].is_enemy(self) {
                        Some((mobidx, &map.mobs[mobidx]))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).min_by_key(|mob| mob.1.hp).unwrap();
            Some((enemy.1.pos, target_pos, 1, enemy.0, enemy.1.hp, target_pos))
        } else {
            while visited[target_pos] != Some(1) {
                target_pos = *routes.get(&target_pos).expect("barf");
            }
            let closest = reachable.into_iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
            Some((closest.0, closest.1, closest.2, closest.3, closest.4, target_pos))

        }
    }
}


fn main() {
    let mut cave = Cave::from_file("input.txt").unwrap();
    let outcome = cave.to_the_death(3);
    println!("15a: Cave fight to the death, won by {}, in {} rounds with {} hp remaining, outcome: {}",
             outcome.1, outcome.0, outcome.2, outcome.0 * outcome.2 as usize);

    for damage in 4.. {
        let mut cave = Cave::from_file("input.txt").unwrap();
        let outcome = cave.to_the_death(damage);
        if cave.mobs.iter().filter(|m| m.mobtype == MobType::Elf).all(|elf| !elf.is_dead()) {
            println!("{}", cave);
            println!("15b: Cave fight glorious Elf victory requires {} attack points, victory in {} rounds with {} hp remaining, outcome: {}",
                damage, outcome.0, outcome.2, outcome.0 * outcome.2 as usize);
            break
        }
    }
}

#[test]
fn test() {
    let mut cave = Cave::from_file("test1.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (47, MobType::Goblin, 590));
    let mut cave = Cave::from_file("test6.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (20, MobType::Goblin, 937));
    let mut cave = Cave::from_file("test5.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (54, MobType::Goblin, 536));
    let mut cave = Cave::from_file("test4.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (35, MobType::Goblin, 793));
    let mut cave = Cave::from_file("test3.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (46, MobType::Elf, 859));
    let mut cave = Cave::from_file("test2.txt").unwrap();
    assert_eq!(cave.to_the_death(3), (37, MobType::Elf, 982));
}

#[test]
fn test2() {
    let mut cave = Cave::from_file("test1.txt").unwrap();
    assert_eq!(cave.to_the_death(15), (29, MobType::Elf, 172));
    let mut cave = Cave::from_file("test3.txt").unwrap();
    assert_eq!(cave.to_the_death(4), (33, MobType::Elf, 948));
    let mut cave = Cave::from_file("test4.txt").unwrap();
    assert_eq!(cave.to_the_death(15), (37, MobType::Elf, 94));
    let mut cave = Cave::from_file("test5.txt").unwrap();
    assert_eq!(cave.to_the_death(12), (39, MobType::Elf, 166));
    let mut cave = Cave::from_file("test6.txt").unwrap();
    assert_eq!(cave.to_the_death(34), (30, MobType::Elf, 38));
    
}
