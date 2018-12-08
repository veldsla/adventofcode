use std::fs::File;
use std::io::{self, Read};

fn sum_meta<I: Iterator<Item=usize>>(it: &mut I) -> usize {
    let mut sum = 0;
    if let Some(num_child) = it.next() {
        let num_meta = it.next().unwrap();
        for _ in 0..num_child {
            sum += sum_meta(it);
        }
        sum += it.take(num_meta).sum::<usize>();
    }
    sum
}

fn sum_meta_b<I: Iterator<Item=usize>>(it: &mut I) -> usize {
    let mut sum = 0;
    if let Some(num_child) = it.next() {
        let num_meta = it.next().unwrap();
        if num_child > 0 {
            let child_sums: Vec<usize> = (0..num_child).map(|_| sum_meta_b(it)).collect();
            sum += it.take(num_meta)
                .map(|meta| {
                    match meta {
                        0 => 0,
                        n if n > child_sums.len() => 0,
                        n => child_sums[n-1]
                    }
                }).sum::<usize>();
        } else {
            sum += it.take(num_meta).sum::<usize>();
        }
    }
    sum
}

fn main() -> io::Result<()> {
        let mut f = File::open("input.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let data = s.split_whitespace()
            .map(|s| s.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData,format!("{}",e))))
            .collect::<Result<Vec<usize>, _>>()?;

        let mut it = data.clone().into_iter();
        println!("8a: Sum meta is {}", sum_meta(&mut it));

        let mut it = data.into_iter();
        println!("8b: Sum meta complex is {}", sum_meta_b(&mut it));

        Ok(())
}

#[test]
fn test() {
    let v = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(sum_meta(&mut v.clone().into_iter()), 138);
    assert_eq!(sum_meta_b(&mut v.into_iter()), 66);
}
