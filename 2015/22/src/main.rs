#[derive(Clone, Eq, Debug)]
struct Spell {
    cost: i32,
    effect: Effect,
    duration: i8,
    immediate: bool
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Effect {
        damage: i32,
        armor: i32,
        heal: i32,
        mana: i32
}

impl PartialEq for Spell {
    fn eq(&self, other: &Spell) -> bool {
        self.cost == other.cost && self.effect == other.effect
    }
}

#[derive(Clone, Debug)]
struct Player {
    mana: i32,
    hit_points: i32,
    armor: i32,
    damage: i32,
    spells: Vec<Spell>,
}

impl Player {
    fn new(m: i32, hp: i32, a: i32, d: i32) -> Player {
        Player {mana: m, hit_points: hp, armor:a, damage: d, spells: Vec::new()}
    }

    fn buy_spell(&mut self, s: Spell, opponent: &mut Player) {
        if s.cost > self.mana {
            panic!("Cannot buy this spell");
        }
        self.mana -= s.cost;
        if s.immediate {
            self.mana += s.effect.mana;
            self.hit_points += s.effect.heal;
            opponent.hit_points -= s.effect.damage;
        }
        self.spells.push(s);
    }

    fn apply_spells(&mut self, opponent: &mut Player) {
        self.armor = 0;
        for s in self.spells.iter_mut() {
            if !s.immediate && s.duration > 0 {
                s.duration -= 1;
                self.mana += s.effect.mana;
                self.armor += s.effect.armor;
                self.hit_points += s.effect.heal;
                opponent.hit_points -= s.effect.damage;
            }
        }
    }

    fn total_spell_cost(&self) -> i32 {
        let mut sum = 0;
        for s in &self.spells {
            sum += s.cost;
        }
        sum
    }

    //This is of course a terrible idea and I should really keep
    //the used spells in a separate list
    fn has_active_spell(&self, s: &Spell) -> bool {
        self.spells.iter().any(|sp| { *sp == *s  && sp.duration > 0} )
    }

    fn receive_damage(&mut self, opponent: &Player) {
        if opponent.damage <= self.armor {
            self.hit_points -= 1;
        } else {
            self.hit_points -= opponent.damage - self.armor;
        };
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
}

//this recursive function consumes the player
//this lets the borrow checker make sure my recursive calls do not
//overwrite other players
fn rec(mut player: Player, mut opponent: Player, spells: &Vec<Spell>, minspentwin: &mut i32, hard: u8) {
    if player.total_spell_cost() > *minspentwin {
        //early return if already spent too much mana
        return;
    }
    if hard == 1 {
        player.hit_points -= 1;
        if player.is_dead() {
            return;
        }
    }

    player.apply_spells(&mut opponent);
    if opponent.is_dead() {
        if *minspentwin > player.total_spell_cost() {
            *minspentwin = player.total_spell_cost();
        }
        return;
    }

    //filter spells based on if in use and cost
    let available_spells = spells.iter().cloned()
        .filter(|s| {s.cost <= player.mana && !player.has_active_spell(s)}).collect::<Vec<Spell>>();
    if available_spells.is_empty() {
        //cannot buy => dead
        return;
    }

    for s in available_spells.iter() {
        let mut newplayer = player.clone();
        let mut newopponent = opponent.clone();

        newplayer.buy_spell(s.clone(), &mut newopponent);
        //boss' turn
        newplayer.apply_spells(&mut newopponent);
        //boss dead?
        if newopponent.is_dead() {
            if *minspentwin > newplayer.total_spell_cost() {
                *minspentwin = newplayer.total_spell_cost();
            }
            return;
        }
        newplayer.receive_damage(&newopponent);
        if newplayer.is_dead() {
            return;
        }
        rec(newplayer, newopponent, spells, minspentwin, hard);
    }

}

fn main() {
    // the spells
    let de = Effect {damage: 0,armor: 0, heal: 0, mana: 0};
    let spells = vec![
        Spell {cost: 53, effect: Effect {damage: 4, ..de}, duration: 0, immediate: true},
        Spell {cost: 73, effect: Effect {damage: 2, heal: 2, ..de}, duration: 0, immediate: true},
        Spell {cost: 113, effect: Effect {armor: 7, ..de}, duration: 6, immediate: false},
        Spell {cost: 173, effect: Effect {damage: 3, ..de}, duration: 6, immediate: false},
        Spell {cost: 229, effect: Effect {mana: 101, ..de}, duration: 5, immediate: false},
    ];

    //22a
    let player = Player::new(500,50,0,0);
    let boss = Player::new(0,58,0,9);
    let mut lowest = 99999999;
    rec(player, boss, &spells, &mut lowest, 0);
    println!("Lowest spent mana to victory is {}", lowest);

    //22b
    let player2 = Player::new(500,50,0,0);
    let boss2 = Player::new(0,58,0,9);
    lowest = 99999999;
    rec(player2, boss2, &spells, &mut lowest, 1);
    println!("Lowest spent mana at level hard = to victory is {}", lowest);
}

#[test]
fn test_1() {
    let de = Effect {damage: 0,armor: 0, heal: 0, mana: 0};
    let spells = vec![
        Spell {cost: 53, effect: Effect {damage: 4, ..de}, duration: 0, immediate: true}, //Magic missile
        Spell {cost: 73, effect: Effect {damage: 2, heal: 2, ..de}, duration: 0, immediate: true}, // Drain
        Spell {cost: 113, effect: Effect {armor: 7, ..de}, duration: 6, immediate: false}, // Shield
        Spell {cost: 173, effect: Effect {damage: 3, ..de}, duration: 6, immediate: false}, //Posion
        Spell {cost: 229, effect: Effect {mana: 101, ..de}, duration: 5, immediate: false}, //recharge
    ];

    let mut player = Player::new(250,10,0,0);
    let mut boss = Player::new(0,13,0,8);
    //turn 1
    player.apply_spells(&mut boss);
    player.buy_spell(spells[3].clone(), &mut boss);
    //oponent turn
    player.apply_spells(&mut boss);
    player.receive_damage(&boss);

    
    //turn 2
    player.apply_spells(&mut boss);
    player.buy_spell(spells[0].clone(), &mut boss);
    //oponent turn
    player.apply_spells(&mut boss);
    assert!(boss.is_dead());
    assert!(!player.is_dead());

}

#[test]
fn test_2() {
    let de = Effect {damage: 0,armor: 0, heal: 0, mana: 0};
    let spells = vec![
        Spell {cost: 53, effect: Effect {damage: 4, ..de}, duration: 0, immediate: true}, //Magic missile
        Spell {cost: 73, effect: Effect {damage: 2, heal: 2, ..de}, duration: 0, immediate: true}, // Drain
        Spell {cost: 113, effect: Effect {armor: 7, ..de}, duration: 6, immediate: false}, // Shield
        Spell {cost: 173, effect: Effect {damage: 3, ..de}, duration: 6, immediate: false}, //Posion
        Spell {cost: 229, effect: Effect {mana: 101, ..de}, duration: 5, immediate: false}, //recharge
    ];

    let mut player = Player::new(250,10,0,0);
    let mut boss = Player::new(0,14,0,8);
    //turn 1
    player.apply_spells(&mut boss);
    player.buy_spell(spells[4].clone(), &mut boss);
    //oponent turn
    player.apply_spells(&mut boss);
    player.receive_damage(&boss);
    assert_eq!(player.hit_points, 2);
    assert_eq!(boss.hit_points, 14);

    
    //turn 2
    player.apply_spells(&mut boss);
    player.buy_spell(spells[2].clone(), &mut boss);
    //oponent turn
    player.apply_spells(&mut boss);
    player.receive_damage(&boss);
    assert_eq!(player.hit_points, 1);
    assert_eq!(player.armor, 7);
    assert_eq!(player.mana, 211);

 
    //turn 3
    player.apply_spells(&mut boss);
    player.buy_spell(spells[1].clone(), &mut boss);
    //oponent turn
    assert_eq!(player.hit_points, 3);
    assert_eq!(player.mana, 239);
    assert_eq!(boss.hit_points, 12);
    player.apply_spells(&mut boss);
    player.receive_damage(&boss);

 
    //turn 4
    player.apply_spells(&mut boss);
    player.buy_spell(spells[3].clone(), &mut boss);
    //oponent turn
    assert_eq!(player.hit_points, 2);
    assert_eq!(boss.hit_points, 12);
    player.apply_spells(&mut boss);
    assert_eq!(player.armor, 7);
    player.receive_damage(&boss);

    //turn 5
    player.apply_spells(&mut boss);

    player.buy_spell(spells[0].clone(), &mut boss);
    //oponent turn
    assert_eq!(player.hit_points, 1);
    assert_eq!(player.mana, 114);
    assert_eq!(boss.hit_points, 2);
    player.apply_spells(&mut boss);
    assert_eq!(player.armor, 0);

    assert!(boss.is_dead());
    assert!(!player.is_dead());

}
