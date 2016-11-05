#![feature(step_by)]
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Aunt {
    name: u16,
    kv: HashMap<String, u8>
}

impl Aunt {
    fn from_line(s: String) -> Aunt {
        let d = s.split(|x| x == ':' || x == ' ' || x == ',').collect::<Vec<&str>>();
        let mut hv = HashMap::<String, u8>::new();

        let n = d[1].parse::<u16>().unwrap();
        for i in (3..d.len()).step_by(4) {
            let k = d[i].to_string();
            let v = d[i+2].parse::<u8>().unwrap();

            hv.insert(k,v);
        }

        Aunt {name: n, kv: hv}
    }

    fn match_attr(&self, k: &str, value: u8) -> bool {
        match self.kv.get(&k.to_string()) {
            Some(x) => *x == value,
            None    => true
        }
    }

    fn match_attr_less(&self, k: &str, value: u8) -> bool {
        match self.kv.get(&k.to_string()) {
            Some(x) => *x < value,
            None    => true
        }
    }

    fn match_attr_more(&self, k: &str, value: u8) -> bool {
        match self.kv.get(&k.to_string()) {
            Some(x) => *x > value,
            None    => true
        }
    }
}

fn main() {
    let f = File::open("16_in.txt").unwrap();
    let reader = BufReader::new(f);

    let mut v = Vec::new();
    for l in reader.lines() {
        let line = l.ok().unwrap();
        let a = Aunt::from_line(line);
        v.push(a);
    }

    let f = v.iter().filter(|a| {
        a.match_attr("children", 3) &&
        a.match_attr("cats", 7) &&
        a.match_attr("samoyeds", 2) &&
        a.match_attr("pomeranians", 3) &&
        a.match_attr("akitas", 0) &&
        a.match_attr("vizslas", 0) &&
        a.match_attr("goldfish", 5) &&
        a.match_attr("trees", 3) &&
        a.match_attr("cars", 2) &&
        a.match_attr("perfumes", 1) }).collect::<Vec<_>>();
    assert_eq!(f.len(),1);

    println!("Aunt nr {} is a match", f[0].name);

    let f = v.iter().filter(|a| {
        a.match_attr("children", 3) &&
        a.match_attr_more("cats", 7) &&
        a.match_attr("samoyeds", 2) &&
        a.match_attr_less("pomeranians", 3) &&
        a.match_attr("akitas", 0) &&
        a.match_attr("vizslas", 0) &&
        a.match_attr_less("goldfish", 5) &&
        a.match_attr_more("trees", 3) &&
        a.match_attr("cars", 2) &&
        a.match_attr("perfumes", 1) }).collect::<Vec<_>>();
    assert_eq!(f.len(),1);

    println!("Aunt nr {} is a match with borked MFCSAM", f[0].name);


}
