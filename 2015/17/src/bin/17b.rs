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

    //if we don't want all permutations we could check
    //level vs min key in rescounter and avoid some calculations
    let mut minkey = 99999999;
    if let Some((key, _)) = rescounter.iter().next() {
      minkey =  *key 
    }
    if level < minkey {
    loop {
        match subv.pop() {
            Some(v)  => {
                    sum += perm_fill(subv.clone(), remaining -v, level + 1, rescounter)
            },
            None => break
        }
    }
}
    sum

}

fn main() {
    //#let v = vec![43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38];
    let v = vec![44, 23, 32, 18, 42, 29, 45, 5, 48, 31, 27, 14, 
12, 26, 46, 14, 35, 18, 18, 46, 50, 37, 46, 25, 50, 
25, 44, 22, 26, 6, 29, 43, 39, 37, 7, 31, 32, 35, 
26, 1, 11, 10, 14, 43, 29, 20, 42, 4, 16, 35, 45, 
33, 40, 47, 25, 26, 41, 16, 1, 8, 30, 48, 48, 17, 
22, 3, 18, 2, 41, 14, 28, 35, 31, 6, 27, 1, 47, 
43, 47, 49, 27, 5, 30, 26, 10, 20, 3, 43, 33, 35, 
13, 1, 7, 10, 47, 16, 26, 22, 14, 31];

    let mut count = BTreeMap::<i32,i32>::new();
    let nperm = perm_fill(v, 150, 1, &mut count);
    //println!("There are {} ways to make 150l", nperm);
    let (key, val) = count.iter().next().unwrap();
    println!("There lowest number of used containers is {}, which can be done in {} ways", key, val);
}

