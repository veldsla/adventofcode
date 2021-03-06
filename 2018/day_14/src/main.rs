fn recipe_scores(n: usize) -> String {
    let mut v = vec![3u8,7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while v.len() < n + 10 {
        let recipe_1 = v[elf_1];
        let recipe_2 = v[elf_2];
        let sum = recipe_1 + recipe_2;
        if sum >= 10 {
            v.push(1);
            v.push(sum - 10);
        } else {
            v.push(sum);
        }
        elf_1 = (elf_1 + (1 + recipe_1) as usize) % v.len();
        elf_2 = (elf_2 + (1 + recipe_2) as usize) % v.len();
    }
    v[n..].iter().take(10).map(|&i| char::from(b'0' + i)).collect()
}

fn recipes_before<S: AsRef<str>>(n: S) -> usize {
    let pattern: Vec<u8> = n.as_ref().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let l = pattern.len();
    if pattern.len() <= 16 {
        return match_pattern_by_number(pattern);
    }

    let mut v = vec![3u8,7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while !v.ends_with(&pattern) {
        let recipe_1 = v[elf_1];
        let recipe_2 = v[elf_2];
        let sum = recipe_1 + recipe_2;
        if sum >= 10 {
            v.push(1);
            if v.ends_with(&pattern) {
                break
            }
            v.push(sum - 10);
        } else {
            v.push(sum);
        }
        elf_1 = (elf_1 + (1 + recipe_1) as usize) % v.len();
        elf_2 = (elf_2 + (1 + recipe_2) as usize) % v.len();
    }
    v.len() - l 
}

fn match_pattern_by_number(n: Vec<u8>) -> usize {
    let mut target = 0;
    let mut mask = 0;
    let l = n.len();
    for (i, v) in n.iter().rev().enumerate() {
        target |= ( (*v as u64) << i*4);
        mask |= 15 << i*4;
    }

    let mut v = vec![3u8,7];
    let mut end = (3u64 << 4) | 7;
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while target != end {
        let recipe_1 = unsafe { *v.get_unchecked(elf_1) };
        let recipe_2 = unsafe { *v.get_unchecked(elf_2) };
        let sum = recipe_1 + recipe_2;
        if sum >= 10 {
            v.push(1);
            end = ((end << 4) & mask) | 1;
            if end == target {
                break
            }
            end = ((end << 4) & mask) | (sum as u64 - 10);
            v.push(sum - 10);
        } else {
            v.push(sum);
            end = ((end << 4) & mask) | sum as u64;
        }
        elf_1 = elf_1 + (1 + recipe_1) as usize;
        while elf_1 >= v.len() { elf_1 -= v.len() };
        elf_2 = elf_2 + (1 + recipe_2) as usize;
        while elf_2 >= v.len() { elf_2 -= v.len() };
    }
    v.len() - l 
}

fn main() {
    let input = 306281;
    println!("14a: Recipies after {} recipies is {}", input, recipe_scores(input));
    println!("14b: Recipies before input sequnce is {}", recipes_before(format!("{}", input)));
}

#[test]
fn test() {
    assert_eq!(recipe_scores(9), "5158916779");
    assert_eq!(recipe_scores(5), "0124515891");
    assert_eq!(recipe_scores(18), "9251071085");
    assert_eq!(recipe_scores(2018), "5941429882");
}

#[test]
fn test2() {
    assert_eq!(recipes_before("51589"), 9);
    assert_eq!(recipes_before("01245"), 5);
    assert_eq!(recipes_before("92510"), 18);
    assert_eq!(recipes_before("59414"), 2018);
}
