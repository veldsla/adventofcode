use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_groups_rec(it: &mut std::str::Chars, group_value: u32) -> (u32, u32) {
    let mut score = 0;
    let mut garbage = 0;
    while let Some(c) = it.next() {
        match c {
            '!' => {
                let _ = it.next();
            },
            '<' => {
                //garbage until '>'
                while let Some(g) = it.next() {
                    match g {
                        '!' => { 
                            let _ = it.next();
                        },
                        '>' => break,
                        _ => {
                            garbage += 1;
                        }
                    }
                }
            },
            '{' => {
                //group open
                let (iscore, igarbage) = score_groups_rec(it, group_value + 1);
                score += iscore;
                garbage += igarbage;
            },
            '}' => {
                score += group_value;
                break;
            },
            _ => {}
        }
    }
    (score, garbage)
}

fn score_groups(s: &str) -> (u32, u32) {
    let mut it = s.chars();
    score_groups_rec(&mut it, 0)
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let s = BufReader::new(f).lines().nth(0).unwrap().unwrap();
    let (score, garbage) = score_groups(&s);
    println!("9a total score = {}, garbage removed = {}", score, garbage);
}

#[test]
fn test() {
    assert_eq!(score_groups("{}").0, 1);
    assert_eq!(score_groups("{{{}}}").0, 6);
    assert_eq!(score_groups("{{},{}}").0, 5);
    assert_eq!(score_groups("{{{},{},{{}}}}").0, 16);
    assert_eq!(score_groups("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(score_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(score_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(score_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
}

#[test]
fn test_b() {
    assert_eq!(score_groups("<>").1, 0);
    assert_eq!(score_groups("<random characters>").1, 17);
    assert_eq!(score_groups("<<<<>").1, 3);
    assert_eq!(score_groups("<{!>}>").1, 2);
    assert_eq!(score_groups("<!!>").1, 0);
    assert_eq!(score_groups("<!!!>>").1, 0);
    assert_eq!(score_groups("<{o\"i!a,<{i<a>").1, 10);

}

