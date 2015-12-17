use std::collections::BTreeMap;

fn perm_fill(v: Vec<i32>, remaining: i32, level: i32, rescounter: &mut BTreeMap<i32,i32>) -> i32 {

    let mut sum = 0;
    let mut subv = Vec::new();
    for x in &v {
        if *x == remaining {
            //match with length level
            let mut c = rescounter.entry(level).or_insert(0);
            *c += 1;
            sum += 1;
        } else if *x < remaining {
            subv.push(*x);
        }
    }
    loop {
        match subv.pop() {
            Some(v) => sum += perm_fill(subv.clone(), remaining -v, level + 1, rescounter),
            None => break
        }
    }
    sum

}

fn main() {
    let v = vec![43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38];

    let mut count = BTreeMap::<i32,i32>::new();
    let nperm = perm_fill(v, 150, 1, &mut count);
    println!("There are {} ways to make 150l", nperm);
    let (key, val) = count.iter().next().unwrap();
    println!("There lowest number of used containers is {}, which can be done in {} ways", key, val);
}

