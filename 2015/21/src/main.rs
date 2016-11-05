use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Weapon {
    name: String,
    cost: i16,
    damage: i16,
    armor: i16
}

type Armor = Weapon;
type Ring = Weapon;

#[derive(Debug)]
struct Player {
    hit_points: i16,
    damage: i16,
    armor: i16,
}

impl Player {
    fn boss() -> Player {
        Player {hit_points: 103, damage: 9, armor: 2}
    }

    fn with_items(w: Weapon, a: Armor, 
                  ring1: Ring, ring2: Ring) -> Player {
        let damage = w.damage + ring1.damage + ring2.damage;
        let armor = a.armor + ring1.armor + ring2.armor;

        Player {hit_points: 100, damage: damage, armor:armor}
        
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }

    fn win_figth(&mut self, opponent: &mut Player) -> bool {
        loop {
            //println!("player: {:?}, opponent: {:?}", self, opponent);
            //player hits
            let mut damage = self.damage - opponent.armor;
            if damage < 0 {
                damage = 1;
            }
            opponent.hit_points -= damage;
            //opponent still alive?
            if opponent.is_dead() {
                return true;
            }
            //opponents hits back
            damage = opponent.damage - self.armor;
            if damage < 0 {
                damage = 1;
            }
            self.hit_points -= damage;
            //am I still alive?
            if self.is_dead() {
                return false;
            }
        }
    }
}

fn main() {
    let weapons = vec![
        Weapon {name: "Dagger".to_string(), cost: 8, damage: 4, armor: 0},
        Weapon {name: "Shortsword".to_string(), cost: 10, damage: 5, armor: 0},
        Weapon {name: "Warhammer".to_string(), cost: 25, damage: 6, armor: 0},
        Weapon {name: "Longsword".to_string(), cost: 40, damage: 7, armor: 0},
        Weapon {name: "Greataxe".to_string(), cost: 74, damage: 8, armor: 0}];
    let armors = vec![
        Armor {name: "None".to_string(), cost: 0, damage: 0, armor: 0},
        Armor {name: "Leather".to_string(), cost: 13, damage: 0, armor: 1},
        Armor {name: "Chainmail".to_string(), cost: 31, damage: 0, armor: 2},
        Armor {name: "Splintmail".to_string(), cost: 53, damage: 0, armor: 3},
        Armor {name: "Bandedmail".to_string(), cost: 75, damage: 0, armor: 4},
        Armor {name: "Platemail".to_string(), cost: 102, damage: 0, armor: 5}];
    let rings = vec![
        Ring {name: "None".to_string(), cost: 0, damage: 0, armor: 0},
        Ring {name: "Damage +1".to_string(), cost: 25, damage: 1, armor: 0},
        Ring {name: "Damage +2".to_string(), cost: 50, damage: 2, armor: 0},
        Ring {name: "Damage +3".to_string(), cost: 100, damage: 3, armor: 0},
        Ring {name: "Defense +1".to_string(), cost: 20, damage: 0, armor: 1},
        Ring {name: "Defense +2".to_string(), cost: 40, damage: 0, armor: 2},
        Ring {name: "Defense +3".to_string(), cost: 80, damage: 0, armor: 3}];

    //generate all players sum costs an check outcome
    let mut cost_map_win = BTreeMap::new();
    let mut cost_map_lose = BTreeMap::new();
    for w in &weapons {
        for a in &armors {
            for r1 in &rings {
                for r2 in &rings {
                    if *r1 == *r2 {
                        continue;
                    }
                    let cost = w.cost + a.cost + r1.cost + r2.cost;
                    let mut player = Player::with_items(w.clone(),a.clone(),r1.clone(),r2.clone());
                    let mut boss = Player::boss();
                    if player.win_figth(&mut boss) {
                        cost_map_win.insert(cost, player);
                    } else {
                        cost_map_lose.insert(cost, player);
                    }
                }
            }
        }
    }
    let (mincost, _) = cost_map_win.iter().next().unwrap();
    let (maxcost, _) = cost_map_lose.iter().last().unwrap();
    println!("Minimum costs to win is {}", mincost);
    println!("Maximum costs and still lose is {}", maxcost);


}


#[test]
fn testfight() {
    let mut player = Player {hit_points: 8, damage: 5, armor: 5};
    let mut boss = Player {hit_points: 12, damage: 7, armor: 2};
    assert!(player.win_figth(&mut boss))
}
