use std::borrow::ToOwned;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::iter::repeat;
use std::path::Path;

#[derive(Debug)]
struct Graph(HashMap<Node, Vec<Node>>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Node(char);

struct Worker(Vec<char>);

impl ToOwned for Graph {
    type Owned = Graph;
    fn to_owned(&self) -> Self::Owned {
        Graph(self.0.iter().map(|(k, v)| (*k, v.clone())).collect())
    }
}

impl Graph {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Graph> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let mut map = HashMap::new();
        for line in b.lines() {
            let line = line?;
            let e: Vec<_> = line.split_whitespace().collect();
            let from = Node(e[1].parse().unwrap());
            let to = Node(e[7].parse().unwrap());
            let list = map.entry(from).or_insert_with(Vec::new);
            list.push(to);
        }

        Ok(Graph(map))
    }

    fn topo_visit(&self) -> String {
        let mut s = String::new();
        let mut queue: Vec<_> = self.roots().into_iter().collect();

        //I need a graph to modify
        let mut graph = self.to_owned();

        queue.sort_by(|a, b| a.cmp(b).reverse());
        while let Some(node) = queue.pop() {
            s.push(node.0);
            // iterate over node targets
            if let Some(mut targets) = graph.0.remove(&node) {
                while let Some(target) = targets.pop() {
                    // any other node still points here?
                    if !graph.has_incoming(target) {
                        queue.push(target)
                    }
                }
            }
            queue.sort_by(|a, b| a.cmp(b).reverse());
        }
        s
    }

    fn worker_time(&self, num_workers: usize, char_score_diff: u8) -> usize {
        let mut workers: Vec<_> = (0..num_workers).map(|_| Worker(Vec::new())).collect();
        let mut queue: Vec<_> = self.roots().into_iter().collect();
        let mut next = Vec::new();

        //I need a graph to modify
        let mut graph = self.to_owned();
    
        queue.sort_by(|a, b| a.cmp(b).reverse());
        while let Some(node) = queue.pop() {
            // check the time the workers finish the prereqs
            let finished = self.incoming(node)
                .into_iter()
                .map(|pre| workers.iter().map(|w| w.last_index_of(pre)).max().unwrap())
                .max()
                .unwrap_or(0);

            // get the first available worker
            let w = workers.iter_mut().min_by_key(|w| w.len()).unwrap();
            // calc the wait and add the task
            let wait = finished.saturating_sub(w.len());
            w.add_task(node, (node.0 as u8 - char_score_diff) as usize, wait as usize);

            // iterate over node targets
            next.clear();
            if let Some(mut targets) = graph.0.remove(&node) {
                while let Some(target) = targets.pop() {
                    // any other node still points here?
                    if !graph.has_incoming(target) {
                        next.push(target)
                    }
                }
            }
            next.sort_by(|a, b| a.cmp(b).reverse());
            next.extend(queue.iter());
            std::mem::swap(&mut queue, &mut next);
        }

        workers.iter().map(|w| w.len()).max().unwrap_or(0)
    }

    fn roots(&self) -> Vec<Node> {
        let mut roots: HashSet<Node> = self.0.keys().cloned().collect();
        for list in self.0.values() {
            for node in list {
                roots.remove(node);
            }
        }
        roots.into_iter().collect()
    }

    fn has_incoming(&self, n: Node) -> bool {
        self.0.values().any(|list| {
            list.contains(&n)
        })
    }

    fn incoming(&self, n: Node) -> Vec<Node>{
         self.0.iter().filter_map(|(k, v)| {
             if v.contains(&n) {
                 Some(*k)
            } else {
                None
            }
         }).collect()
     }
}

impl Worker {
    fn last_index_of(&self, node: Node) -> usize {
        self.0.iter().rposition(|&i| i == node.0)
            .map_or(0, |i| i + 1)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn add_task(&mut self, task: Node, duration: usize, wait: usize) {
        self.0.extend(repeat('.').take(wait).chain(repeat(task.0).take(duration)));
    }
}

fn main() -> io::Result<()> {
    let g = Graph::from_file("input.txt")?;
    println!("7a:  Instruction order {}", g.topo_visit());
    println!("7b:  Total worker time for {} workers isr {}", 5, g.worker_time(5, 4));
    Ok(())
}

#[test]
fn test() {
    let g = Graph::from_file("test.txt").unwrap();
    assert_eq!(g.topo_visit(), "CABDFE".to_owned());
    assert_eq!(g.worker_time(2, 64), 15);
}
