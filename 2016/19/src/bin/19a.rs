fn main() {
    let n = 3012210;
    let mut p = vec![true; n];

    let mut has = false;
    loop {
        let mut i = 0;
        while i < n {
            match has {
                true if p[i] => {
                    p[i] = false;
                    has = false;
                },
                true => {},
                false if p[i] => {
                    has = true;
                },
                false => {}
            }
            i+= 1;
        }

        let npresent = p.iter().fold(0, |a, &x| a + if x {1} else {0});
        println!("{} elves have presents", npresent);
        if npresent == 1 {
            println!("Happy elf is nr: {}", p.iter().position(|p| *p).unwrap()+1);
            break;
        }
    }
}
