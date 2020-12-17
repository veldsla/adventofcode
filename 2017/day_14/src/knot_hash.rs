//From day 10

const SALT: [u8; 5] = [17,31,73,47,23];

fn rearrange(lengths: &[u8], loops: usize) -> (u16, Vec<u8>) {
    let mut v: Vec<u8> = (0..).take(256).collect();

    let mut skip = 0;
    let mut pos = 0;
    
    for _ in 0..loops {
        for l in lengths.iter().map(|l| *l as usize) {
            reverse_range(&mut v, pos, l);
            skip += 1;
            pos += l + skip - 1;
        }
    }

    (v[0] as u16 * v[1] as u16, v)
}

fn parse_input(s: &str) -> Vec<u8> {
     s.bytes()
         .chain(SALT.iter().cloned())
         .collect()
}

fn pack_hash(v: &[u8]) -> Vec<u8> {
    v.chunks(16)
        .map(|s| s.iter().fold(0, |acc, x| acc ^ x))
        .collect()
}

fn to_hex(v: &[u8]) -> String {
    v.iter().map(|e| format!("{:02x}", e)).collect()
}

pub fn knot_hash(s: &str) -> String {
    let seq = parse_input(s);
    let (_, sparse_hash) = rearrange(&seq, 64);
    let packed_hash = pack_hash(&sparse_hash);
    to_hex(&packed_hash)
}

fn reverse_range(v: &mut [u8], pos: usize, l: usize) {
    let len = v.len();
    (pos..(pos + l / 2))                                        //loop over half the positions
        .map(|p| (p % len, ((pos + l - 1) - (p - pos)) % len))  //calc mate and wrap if necessary
        .for_each(|(from, to)| v.swap(from, to));               //swap the elements
}

#[test]
fn reverse() {
    let mut v: Vec<_> = (0..10).collect();
    reverse_range(&mut v, 3, 3);
    assert_eq!(v, vec![0,1,2,5,4,3,6,7,8,9]);
    reverse_range(&mut v, 8, 4);
    assert_eq!(v, vec![9,8,2,5,4,3,6,7,1,0]);
    reverse_range(&mut v, 0, 0);
    assert_eq!(v, vec![9,8,2,5,4,3,6,7,1,0]);
    reverse_range(&mut v, 5, 1);
    assert_eq!(v, vec![9,8,2,5,4,3,6,7,1,0]);
    reverse_range(&mut v, 0, 10);
    assert_eq!(v, vec![0,1,7,6,3,4,5,2,8,9]);
    reverse_range(&mut v, 5, 10);
    assert_eq!(v, vec![9,8,2,5,4,3,6,7,1,0]);
    reverse_range(&mut v, 5, 9);
    assert_eq!(v, vec![1,7,6,3,4,5,2,8,9,0]);
}

#[test]
fn knot() {
    assert_eq!([65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22].iter().fold(0, |acc, x| acc ^ x), 64);
    assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
