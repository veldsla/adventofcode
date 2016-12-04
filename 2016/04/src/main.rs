use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Room {
    name: Vec<String>,
    sector_id: u32,
    checksum: Vec<char>
}

#[derive(Debug)]
enum RoomError {
    ChecksumError,
    NameError
}

impl Room {
    fn new(name_parts: Vec<String>, sector: u32, checksum: Option<Vec<char>>) -> Result<Room, RoomError> {
        let calc_checksum = Room::checksum(&name_parts);
        if let Some(sum) = checksum {
            if sum != calc_checksum {
                return Err(RoomError::ChecksumError);
            }
        }
        Ok(Room { name: name_parts, sector_id: sector, checksum: calc_checksum })
    }

    fn from_str(s: &str) -> Result<Room, RoomError> {
        if let Some(c) =  s.rfind('[') {
            if let Some(d) = s.rfind('-') {
                let checksum: Vec<char> = s[c+1..s.len()-1].chars().collect();
                let name_parts: Vec<String> = s[0..d].split("-").map(|s| s.to_string()).collect();
                let sector_id = try!(s[d+1..c].parse::<u32>().map_err(|_| RoomError::NameError));

                Room::new(name_parts, sector_id, Some(checksum))
            } else {
                Err(RoomError::NameError)
            }
        } else {
            Err(RoomError::NameError)
        }
    }

    fn decrypt_name(&self) -> String {
        let decrypted_parts: Vec<String> = self.name.iter().map(|p| {
            p.chars().map(|c| {
                let c = c as u8;
                (((c - 'a' as u8 + (self.sector_id % 26) as u8) % 26) + 'a' as u8 ) as char
            }).collect::<String>()
        }).collect();

        let mut s = String::new();
        for (i, p) in decrypted_parts.iter().enumerate() {
            if i>0 {
                s.push(' ');
            }
            s.push_str(&p);
        }
        s
    }

    fn checksum(name: &Vec<String>) -> Vec<char> {
        let mut count = HashMap::new();
        for c in name.iter().flat_map(|w| w.chars()) {
            let e = count.entry(c).or_insert(0);
            *e += 1;
        }
        let mut lt: Vec<(u32, char)> = count.iter().map(|v| (*v.1, *v.0)).collect();
        lt.sort_by(|a, b| {
            match a.0.cmp(&b.0).reverse() {
                Ordering::Equal => a.1.cmp(&b.1),
                o => o
            }
        });

        lt.iter().take(5).map(|t| t.1).collect()
    }
}

fn main() {
    let r = File::open("input.txt").and_then(|f| Ok(BufReader::new(f)))
        .unwrap_or_else(|_| panic!("Error opening input.txt"));
    let rooms: Vec<Room> = r.lines().map(|l| Room::from_str(l.unwrap().as_ref()))
        .filter(|r| r.is_ok()).map(|r| r.unwrap()).collect();
    
    println!("The room list contains {} valid entries, and the sector ids sum to {}", rooms.len(),
        rooms.iter().map(|r| r.sector_id).sum::<u32>());

    println!("Rooms matching north:");
    let fr = rooms.iter().filter(|r| r.decrypt_name().contains("north"));
    for f in fr {
        println!("\t{}, {}",f.sector_id, f.decrypt_name());
    }
}

#[test]
fn test() {
    assert!(Room::from_str("aaaaa-bbb-z-y-x-123[abxyz]").is_ok());
    assert!(Room::from_str("a-b-c-d-e-f-g-h-987[abcde]").is_ok());
    assert!(Room::from_str("not-a-real-room-404[oarel]").is_ok());
    assert!(Room::from_str("totally-real-room-200[decoy]").is_err());
}

#[test]
fn decrypt() {
    let r = Room::new(vec!["qzmt".to_string(),"zixmtkozy".to_string(),"ivhz".to_string()], 343, None).unwrap();
    assert_eq!(r.decrypt_name(), "very encrypted name");

}
