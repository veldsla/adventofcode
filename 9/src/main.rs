use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct CityRouter {
    cities: Vec<String>,
    keys: Vec<usize>,
    dir: Vec<bool>,
}

impl CityRouter {
    fn new(c: Vec<String>) -> CityRouter{
        let mut keys = Vec::new();
        let mut dirs = Vec::new();
        
        for i in  0..c.len() {
            keys.push(i+1);
            dirs.push(false);
        }

        CityRouter {cities: c, keys: keys, dir:dirs}
    }
}

//implement steinhaus johnson trotter algorithm
impl Iterator for CityRouter {
    type Item = bool;

    fn next(&mut self) -> Option<bool>{
        println!("{:?}", self);
        let len = self.cities.len();
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
        //also modify the cities vector
        self.cities.swap(index_largest_mobile, target as usize);
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

fn dist(cities: &Vec<String>, distances: &HashMap<(String, String), u64>) -> u64 {
    let mut sumdist:u64 = 0;
    println!("{:?}", cities);
    for i in 0..cities.len() -1 {
        let d = distances.get(&(cities[i].clone(), cities[i+1].clone()));
        if d.is_some() {
            sumdist += *d.unwrap();
        } else {
            sumdist += *distances.get(&(cities[i+1].clone(), cities[i].clone())).unwrap();
        }
    }
    sumdist
}

fn main() {
    let mut distances = HashMap::<(String, String), u64>::new();
    let mut cities = HashSet::<String>::new();

    let f = File::open("9_in.txt").unwrap();
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let line = l.ok().unwrap();
        let parts = line.split(" ").collect::<Vec<&str>>();
        let from = parts[0].to_string();
        let to = parts[2].to_string();
        let distance = parts[4].parse::<u64>().unwrap();

        distances.insert((from.clone(), to.clone()), distance);
        cities.insert(from);
        cities.insert(to);
    }
    
    //permutation generator doesn't return original....start with original order
    
    let cv = cities.iter().map(|s| {s.to_string()}).collect::<Vec<String>>();
    let mut mindist = dist(&cv, &distances);
    let mut maxdist = mindist;
    let mut cityrouter = CityRouter::new(cv);
    while cityrouter.next().is_some() {
        let newdist = dist(&cityrouter.cities, &distances);
        println!("{:?}", newdist);
        if newdist < mindist {
            mindist = newdist;
        }

        if newdist > maxdist {
            maxdist = newdist;
        }
    }

    println!("The minimum distance is {}, maximum {}", mindist, maxdist);

}

#[test]
fn cityrouter() {
    let c = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()];
    let keys = vec![1,2,3,4];
    let dirs = vec![false;4];


    let mut cr = CityRouter {cities: c, keys: keys, dir:dirs};
    let mut hs = HashSet::new();

    let mut i = 0;
    while cr.next().is_some() {
         i+=1;
         hs.insert(format!("{:?}", cr.cities));
    }
    assert_eq!(i, 23);
    assert_eq!(hs.len(), 23);
}
