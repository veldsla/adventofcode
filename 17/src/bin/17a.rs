
fn perm_fill(v: Vec<i32>, remaining: i32) -> i32 {

    let mut sum = 0;
    let mut subv = Vec::new();
    for x in &v {
        if *x == remaining {
            sum += 1;
        } else if *x < remaining {
            subv.push(*x);
        }
    }
    loop {
        match subv.pop() {
            Some(v) => sum += perm_fill(subv.clone(), remaining - v),
            None => break
        }
    }
    sum

}

fn main() {
    let v = vec![43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38];
    let nperm = perm_fill(v, 150);
    println!("There are {} ways to make 150l", nperm);
}

