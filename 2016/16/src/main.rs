fn dragon_curve(start: &str, l: usize) -> String {
    let mut res = String::from(start);
    while res.len() < l {
        let n = rev_comp(&res);
        res.push('0');
        res.push_str(&n);
    }
    res.truncate(l);
    res

}

fn rev_comp(s: &str) -> String {
    s.chars().rev().map(|c| {
        match c {
            '1' => '0',
            '0' => '1',
            _ => panic!("cannot complement")
        }
    }).collect()
}

fn checksum(s: &str) -> String {
    let cs: String = s.as_bytes().chunks(2).map(|c| {
        if c[0] == c[1] {
            '1'
        } else {
            '0'
        }
    }).collect();
    if cs.len() % 2 == 0 {
        checksum(&cs)
    } else {
        cs
    }
}

fn main() {
    let input = "01111001100111011";
    let res = dragon_curve(input, 272);
    let cs = checksum(&res);
    println!("The checksum for length 272 = {}", cs);
    let res = dragon_curve(input, 35651584);
    let cs = checksum(&res);
    println!("The checksum for length 35651584 = {}", cs);
}

#[test]
fn dragon() {
    assert_eq!(dragon_curve("1",3), "100");
    assert_eq!(dragon_curve("0",3), "001");
    assert_eq!(dragon_curve("11111",11), "11111000000");
    assert_eq!(dragon_curve("111100001010",25), "1111000010100101011110000");
}

#[test]
fn checks() {
    assert_eq!(checksum("01111001100111011"), "100");
}

#[test]
fn all() {
    let res = dragon_curve("10000",20);
    let cs = checksum(&res);
    assert_eq!(res, "10000011110010000111");
    assert_eq!(cs, "01100");
}
