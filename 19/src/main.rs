use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Plant {
    replacements: HashMap<String, Vec<String>>,
    rev_replacements: HashMap<String, String>,
    elements: Vec<String>
}

impl Plant {
    fn new() -> Plant {
        let r = HashMap::new();
        let rv = HashMap::new();
        let e = Vec::new();
        Plant {replacements :r, rev_replacements: rv, elements: e}
    }

    fn add_replacement(&mut self, from: String, to: String) {
        let e = self.replacements.entry(from.clone()).or_insert(Vec::<String>::new());
        e.push(to.clone());
        self.rev_replacements.insert(to, from);
    }

    fn set_input(&mut self, s: String) {
        let chars = s.chars().collect::<Vec<char>>();
        let mut e = String::new();
        for i in 0..chars.len() {
            e.push(chars[i]);
            if (i < chars.len() -1 && chars[i+1].is_uppercase()) ||
               (i == chars.len() -1) {
                self.elements.push(e);
                e = String::new();
            }
        }
    }

    fn count_replacements(&mut self) -> usize {
        let mut hs = HashSet::new();
        for e in 0..self.elements.len() {
            if self.replacements.contains_key(&self.elements[e]) {
                let s1 = self.elements.iter().cloned().take(e).collect::<String>();
                let s2 = self.elements.iter().cloned().skip(e+1).collect::<String>();
                for r in self.replacements.get(&self.elements[e]).unwrap() {
                    let mut s = s1.clone();
                    s.push_str(r);
                    s.push_str(s2.as_ref());
                    hs.insert(s);
                }
            }
        }
        hs.len()
    }

    //this method occasionaly produces the correct answer
    //due to the random order in the replacement hashmap.
    //It is far too complex to run to completion
    fn decompose(&self, s: String, level: usize) ->usize {
        
        if s == "e" {
            println!("reduced to e in level {}", level-1);
            return 0;
        }

        //find all occurences of rev replacements
        //replace and call self
        for (from, to) in self.rev_replacements.iter() {
            for m in s.match_indices(from) {
                let mut news = s[..m.0].to_string();
                news.push_str(to);
                news.push_str(&s[(m.0+m.1.len())..]);
                self.decompose(news, level +1);
            }
        }

        0
    }

    fn min_decompose_steps(&mut self) -> usize{
        let s = self.elements.iter().cloned().collect::<String>();
        self.decompose(s,1)
    }
}

fn main() {
    let f = File::open("19_in.txt").unwrap();
    let reader = BufReader::new(f);

    let mut plant = Plant::new();
    let mut ri = reader.lines();
    loop {
        match ri.next().unwrap() {
            Ok(line) => {
                if line == "" {
                    let input = ri.next().unwrap().expect("error in file");
                    plant.set_input(input);
                    break;
                } else {
                    let mut it = line.split(" => ");
                    plant.add_replacement(it.next().unwrap().to_string(), it.next().unwrap().to_string());
                }
            }
            Err(e) => panic!(e)
        }

    }
    println!("{} different molecules can be created", plant.count_replacements());
    plant.min_decompose_steps();
}
