fn main() {
    let input = 369;

    //Actually doing the insertions:
    let mut buffer = vec![0];
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + input) % buffer.len() + 1;
        buffer.insert(pos, i);
    }
    println!("16a: The number after the newly inserted 2017 is: {}", buffer[(pos+1)%buffer.len()]);

    //Only looking at pos == 0
    let mut pos = 0;
    let i = (1..50_000_001)
        .filter(|&i| { 
            pos = (pos + input) % i + 1;
            pos == 1
        }).last().unwrap();
    println!("16b: The last number inserted at pos=0 after 50M rounds = {}", i);
}

#[test]
fn test() {
    let input = 3;
    let mut buffer = vec![0];
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + input) % buffer.len() + 1;
        buffer.insert(pos, i);
        if i <= 10 {
            println!("{:?}", buffer);
        }
    }
    println!("{:?}", buffer);
    assert_eq!(buffer[(pos+1)%buffer.len()], 638);
}
