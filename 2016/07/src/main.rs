use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct IPv7 {
    parts: Vec<Vec<u8>>,
    hypernet_sequences: Vec<Vec<u8>>,
}

impl<'a> From<&'a str> for IPv7 {
    fn from(s: &str) -> IPv7 {
        if !s.contains('[') {
            return IPv7::new(vec![s.bytes().collect()], Vec::new());
        }

        let mut parts = Vec::new();
        let mut hp = Vec::new();
        let mut pos = 0;
        for (open, close) in s.match_indices('[').zip(s.match_indices(']')) {
            parts.push(s.bytes().skip(pos).take(open.0 - pos).collect());
            hp.push(s.bytes().skip(open.0 + 1).take(close.0 - open.0 - 1).collect());
            pos = close.0 + 1;
        }
        if pos < s.len() -1 {
            parts.push(s.bytes().skip(pos).collect());
        }
        IPv7::new(parts, hp)
    }
}

impl IPv7 {
    fn new(parts: Vec<Vec<u8>>, hp: Vec<Vec<u8>>) -> IPv7 {
        IPv7 { parts: parts, hypernet_sequences: hp }
    }

    fn supports_tls(&self) -> bool {
        self.parts.iter().any(|p| p.windows(4).any(abba))
            && !self.hypernet_sequences.iter().any(|p| p.windows(4).any(abba))

    }

    fn supports_ssl(&self) -> bool{
        let abas: Vec<_> = self.parts.iter().flat_map(|p| {
            p.windows(3).filter_map(|s| {
                if aba(s) {
                    Some(s)
                } else {
                    None
                }
            })
        }).collect();

        abas.iter().any(|a| {
            self.hypernet_sequences.iter().flat_map(|h| {
                h.windows(3)
            }).any(|hs|{
                hs.len() == 3 && hs[0] == a[1] && hs[1] == a[0] && hs[2] == a[1]
            })
        })
    }
}

fn aba(s: &[u8]) -> bool {
    match s[0] != s[1] && s.len() == 3 {
        false => false,
        true => s[0] == s[2]
    }
}

fn abba(s: &[u8]) -> bool {
    match s[0] != s[1] && s.len() == 4 {
        false => false,
        true => s[1] == s[2] && s[3] == s[0]
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let ips: Vec<_> = BufReader::new(f).lines()
        .map(|s| IPv7::from(s.unwrap().as_ref())).collect();
    let ntls = ips.iter().fold(0, |acc, i| if i.supports_tls() { acc + 1 } else { acc } );
    println!("{} out of {} IP addresses support TLS", ntls, ips.len());
    let nssl = ips.iter().fold(0, |acc, i| if i.supports_ssl() { acc + 1 } else { acc } );
    println!("{} out of {} IP addresses support SSL", nssl, ips.len());
}

#[test]
fn test_tls() {
    assert!(IPv7::from("abba[mnop]qrst").supports_tls());
    assert!(!IPv7::from("abcd[bddb]xyyx").supports_tls());
    assert!(!IPv7::from("aaaa[qwer]tyui").supports_tls());
    assert!(IPv7::from("ioxxoj[asdfgh]zxcvbn").supports_tls());
    assert!(IPv7::from("aaaa[qwer]tyuiioxxoj[asdfgh]zxcvbn").supports_tls());
    assert!(!IPv7::from("abcd[bddb]xyyxaaaa[qwer]tyui").supports_tls());
    assert!(IPv7::from("abcd[bcdb]xyyxaaaa[qwer]tyuiazba").supports_tls());
}

#[test]
fn test_ssl() {
    assert!(IPv7::from("aba[bab]xyz").supports_ssl());
    assert!(!IPv7::from("xyx[xyx]xyx").supports_ssl());
    assert!(IPv7::from("aaa[kek]eke").supports_ssl());
    assert!(IPv7::from("zazbz[bzb]cdb").supports_ssl());
}

#[test]
fn test_aba() {
    assert!(aba(b"aba"));
    assert!(!aba(b"xyz"));
    assert!(aba(b"zaz"));
}

#[test]
fn test_abba() {
    assert!(abba(b"abba"));
    assert!(!abba(b"xcvb"));
}
