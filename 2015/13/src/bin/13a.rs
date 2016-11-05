use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;



//I'll reuse the steinhaus johnson trotter algorithm from 9
struct TableSeater {
    people: Vec<String>,
    keys: Vec<usize>,
    dir: Vec<bool>,
    scores: HashMap<(String, String), i32>

}

impl TableSeater {
    fn new(c: Vec<String>, scores: HashMap<(String, String), i32>) -> TableSeater{
        let mut keys = Vec::new();
        let mut dirs = Vec::new();
        
        for i in  0..c.len() {
            keys.push(i+1);
            dirs.push(false);
        }

        TableSeater {people: c, keys: keys, dir:dirs, scores:scores}
    }

    fn score(&mut self) ->i32 {
        let mut score = 0;
        for i in 0..self.people.len() {
            let t1 = if i == 0 { self.people.len()-1 } else { i-1};
            let t2 = if i == self.people.len()-1 { 0 } else { i+1};
            let s1 = *self.scores.get(&(self.people[i].clone(), self.people[t1].clone())).unwrap();
            let s2 = *self.scores.get(&(self.people[i].clone(), self.people[t2].clone())).unwrap();
            //println!("{} {}", s1,s2);
            score+=s1+s2;
            //score += *self.scores.get(&(self.people[t1].clone(),self.people[i].clone())).unwrap();
            //score += *self.scores.get(&(self.people[i].clone(),self.people[t2].clone())).unwrap();
        }
        score
    }
}


//implement steinhaus johnson trotter algorithm
impl Iterator for TableSeater {
    type Item = bool;

    fn next(&mut self) -> Option<bool>{
        let len = self.people.len();
        let mut index_largest_mobile = 0;
        let mut largest_key = 0; 

        for i in 0..len {
            if self.dir[i] && i < len - 1 && self.keys[i] > self.keys[i + 1] ||
                !self.dir[i] && i > 0 && self.keys[i] > self.keys[i - 1] {
                if self.keys[i] > largest_key {
                    largest_key = self.keys[i];
                    index_largest_mobile = i;
                }
            }
        }
            
        if largest_key == 0 {
            return None;
        }

        //swap the values
        let target = if self.dir[index_largest_mobile] {
            index_largest_mobile + 1
        } else {
            index_largest_mobile - 1
        };
        self.keys.swap(index_largest_mobile, target as usize);
        //also modify the people vector
        self.people.swap(index_largest_mobile, target as usize);
        //swap self.directions
        self.dir.swap(index_largest_mobile, target as usize);

        // reverse the direction of all integers larger than k
        for i in 0..len {
            if self.keys[i] > largest_key {
                self.dir[i] = !self.dir[i];
            }
        }
        return Some(true);
    }
}




fn main() {

    let f = File::open("13_in.txt").unwrap();
    //let f = File::open("example.txt").unwrap();
    let reader = BufReader::new(f);

    let mut scores = HashMap::new();
    let mut people = HashSet::new();
    for l in reader.lines() {
        let mut line = l.ok().unwrap();
        //remove .
        line.pop();
        let d = line.split(" ").collect::<Vec<&str>>();
        let from = d[0].to_string();
        let to = d[10].to_string();
        let score = match d[2] {
            "gain" => d[3].parse::<i32>().unwrap(),
            "lose" => d[3].parse::<i32>().unwrap() * -1,
            _      => panic!("Error parsing file")
        };
        people.insert(from.clone());
        scores.insert((from, to), score);
    }

    let mut ts = TableSeater::new(people.iter().map(|s| s.clone()).collect::<Vec<String>>(), scores);

    let mut i = 0;
    let mut max = -999999;
    while ts.next().is_some() {
         i+=1;
         let score = ts.score();
         if score > max {
            max = score;
         }
         //println!("{:?}, score={}", ts.people, score)
         //hs.insert(format!("{:?}", cr.cities));
    }
    println!("nperm={}, max score = {}", i, max);

}

#[test]
fn tableseater() {
    let c = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
    ];
    let keys = (1..13).collect::<Vec<usize>>();
    let dirs = vec![false;12];


    let mut cr = TableSeater {people: c, keys: keys, dir:dirs};

    let mut i = 0;
    while cr.next().is_some() {
         i+=1;
         //hs.insert(format!("{:?}", cr.cities));
    }
    assert_eq!(i, 23);
}

