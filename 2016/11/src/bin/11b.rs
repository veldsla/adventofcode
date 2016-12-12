/// There are 5 elements in my puzzel
/// we have 2 types (chips/generators) that can be on or off (2 bits)
///
///                allocated bits
/// cobalt      0,1
/// polonium    2,3
/// promethium  4,5
/// ruthenium   6,7
/// thulium     8,9
/// elerium     10,11
/// dilithium   12,13
/// 
/// Chips go to the lowest bit, generators the highest
/// 
/// The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, 
///    a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator,
///    and a cobalt-compatible microchip.
/// The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
/// The third floor contains nothing relevant.
/// The fourth floor contains nothing relevant.
///
/// 11b:
/// first floor add: elerium/dilithim chip gennie pair
/// 
/// So floor 1: 11 11 11 11 10 10 11 (16363)
///          2: 00 00 01 01 00 (20)
///          3: 00 00 00 00 00 (0)
///          4: 00 00 00 00 00 (0)
///          
/// Using bit shift operators to expose at risk microchips all valid floor configurations can be
/// precalculated (there are 2314 valid floors and we want floor four the be 16383.
/// 
/// other bit operations:
/// e.g. take a polonium generator from floor 1: ( 00 00 10 00 00) from (11 11 10 10 11)
/// use xor:  11 11 10 10 11 ^ 00 00 10 00 00 = 11 11 00 10 11
/// 
/// add a  promethium chip to floor 1
/// use or: 11 11 10 10 11 | 00 00 00 01 00 = 11 11 10 11 11
/// 
/// 
/// We can also precalculate a simple hashmap<from, Vec<to> > that gives the possible outcomes for
/// a given floor and all types a Floor can accept
/// 

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Floor(u16);

struct GameData {
    out_moves: HashMap<Floor, Vec<(u16, Floor)>>,
    in_moves: HashMap<Floor, Vec<u16>>,
}

impl Floor {
    fn is_valid(&self) -> bool {
        let Floor(mut n) = *self;
        
        //a chip is at risk if there is no matching generator and another generator
        let mut at_risk = false;
        let mut dangerous = false;
        for _ in 0..7 {
            match n & 3 {
                3 => dangerous = true, // protected, but dangerous
                2 => dangerous = true, // dangerous
                1 => at_risk = true,   // at risk
                0 => {},               // no item
                _ => unreachable!()
            }
            n >>= 2;
        }
        (at_risk && !dangerous) || (!at_risk)
    }

    //calculate what you can take from a floor and what that would leave
    fn next_floors(&self) -> Vec<(u16, Floor)> {
        let Floor(n) = *self;

        let mut res = Vec::new();

        //take one item:
        for s in 0..14 {
            let m =  1 << s;
            if n & m == m {
                //bit is set, we can take it
                let f = Floor(n ^ m);
                if f.is_valid() {
                    res.push((m, f));
                }
            }
        }

        //take two items a matched generator-chip
        for s in 0..7 {
            let m = 3 << (s*2);
            if  n & m  == m {
                //both are present taking this should always be safe
                let f = Floor(n ^ m);
                if f.is_valid() {
                    res.push((m, f));
                } else {
                    panic!("why you cannot take both???");
                }
            }
        }

        //or any combination of chip-chip / gennie-gennie
        for s in 0..12 {
            let m = 5 << s;
            if  n & m  == m {
                let f = Floor(n ^ m);
                if f.is_valid() {
                    res.push((m, f));
                }
            }
        }
        res
    }
}

impl GameData {
    fn new() -> GameData {
        //generate all valid floors  and possible moves
        let valid_floors: Vec<_> = (0..16384).map(|i| Floor(i)).filter(|f| f.is_valid()).collect();
        println!("Game has {} valid floors", valid_floors.len());
        let out_moves: HashMap<_, _> = valid_floors.into_iter().map(|f| (f, f.next_floors())).collect();
        let mut in_moves = HashMap::new();

        for (_, v) in &out_moves {
            for f in v.iter() {
                let e = in_moves.entry(f.1).or_insert(Vec::new());
                e.push(f.0);
            }
        }

        GameData { out_moves: out_moves, in_moves: in_moves }
    }
}

#[derive(Clone, Copy)]
struct GameState {
    floors: [Floor; 4],
    elevator_pos: usize,
    step_count: usize,
}

impl GameState {
    fn do_move(&mut self, item: u16, remains: Floor, to: usize) {
        self.floors[self.elevator_pos] = remains;
        self.elevator_pos = to;
        let Floor(mut target) = self.floors[to];
        assert_eq!(target & item, 0);
        target |= item;
        self.floors[to] = Floor(target);
        self.step_count += 1;
    }

    fn to_int(&self) -> usize {
        let mut res: usize = 0;
        for i in 0..4 {
            let Floor(n) = self.floors[i];
            res |= (n as usize) << (i*14)
        
        }
        //add elevator pos:
        res |= self.elevator_pos << 56;
        res
    }

    fn solved(&self) -> bool {
        self.floors[3] == Floor(16383)
    }
}


fn solve(game: GameState, data: &GameData) -> usize {

    //this should really be a min heap...but we'll just waste some memory and
    //keep on adding states.....
    let mut game_queue = Vec::with_capacity(20000000);
    game_queue.push(game);
    let mut seen = HashSet::with_capacity(1000000);
    let mut vec_pos = 0;

    let mut last_depth = 0;
    loop {
        while game_queue.len() > 0 {
            let g = game_queue[vec_pos];
            vec_pos += 1;
            if g.step_count > last_depth {
                println!("Depth: {}, queue size: {}", g.step_count, game_queue.len());
                last_depth = g.step_count;
            }
            if !seen.contains(&g.to_int()) {
                seen.insert(g.to_int());
                //check the current game state and return all possible moves
                let next = gamestate_next(g, data);
                // any solutions
                if let Some(min_solved) = next.iter().filter(|s| s.solved()).map(|s| s.step_count).min() {
                    //game solved
                    return min_solved;
                } else {
                    game_queue.extend(next.into_iter());
                }
            }
        }
    }
}

fn gamestate_next(game: GameState, data: &GameData) -> Vec<GameState> {

    let possible_floors = match game.elevator_pos {
        2 => vec![3,1],
        1 => vec![2,0],
        0 => vec![1],
        3 => vec![2],
        _ => unreachable!()
    };

    let from_floor = &game.floors[game.elevator_pos];
    let mut next_states = Vec::new();
    if let Some(m) = data.out_moves.get(from_floor) {
        for floor in possible_floors.iter() {
            let to_floor = &game.floors[*floor];
            for mv in m.iter() {
                if let Some(valid_to) = data.in_moves.get(to_floor) {
                    if valid_to.contains(&mv.0) {
                        //do move
                        let mut state = game.clone();
                        state.do_move(mv.0, mv.1, *floor);
                        next_states.push(state);
                    }
                }
            } 
        }
    }
    next_states
}

fn main() {
    let game = GameState {floors: [Floor(16363), Floor(20), Floor(0), Floor(0)], elevator_pos: 0, step_count: 0 };
    let data = GameData::new();
    let solved_at = solve(game, &data);
    println!("Solved after {} steps", solved_at);
}

#[test]
fn gameint() {
    let game = GameState {floors: [Floor(235), Floor(20), Floor(0), Floor(0)], elevator_pos: 0, step_count: 0 };
    assert_eq!(game.to_int(), 235 + (20 << 14));

}

