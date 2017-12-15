struct Generator {
    value: u64,
    factor: u64
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.value *= self.factor;
        self.value %= 2147483647;
        Some(self.value)
    }
}

impl Generator {
    fn into_picky(self, m: u64) -> PickyGenerator {
        PickyGenerator { generator: self, multiple: m }
    }
}

struct PickyGenerator {
    generator: Generator,
    multiple: u64,
}

impl Iterator for PickyGenerator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        while let Some(v) = self.generator.next() {
            if v % self.multiple == 0 {
                return Some(v)
            }
        }
        None
    }
}

fn main() {
    let gen_a = Generator { value : 516, factor: 16807};
    let gen_b = Generator { value : 190, factor: 48271};
    let matches: u32 = gen_a.take(40_000_000).zip(gen_b)
        .map(|(a, b)| {
            if a ^ b == 0 || (a ^ b) % 65536 == 0 {
                1
            } else {
                0
            }
        }).sum();
    println!("15a: Generators produce {} matches in 40M iterations", matches);

    let gen_a = Generator { value : 516, factor: 16807};
    let gen_b = Generator { value : 190, factor: 48271};
    let matches: u32 = gen_a.into_picky(4).take(5_000_000).zip(gen_b.into_picky(8))
        .map(|(a, b)| {
            if a ^ b == 0 || (a ^ b) % 65536 == 0 {
                1
            } else {
                0
            }
        }).sum();
    println!("15b: Picky generators produce {} matches in 5M iterations", matches);
}

#[test]
fn test() {
    let gen_a = Generator { value : 65, factor: 16807};
    let gen_b = Generator { value : 8921, factor: 48271};
    assert_eq!(gen_a.take(5).collect::<Vec<_>>(), vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]);
    assert_eq!(gen_b.take(5).collect::<Vec<_>>(), vec![430625591, 1233683848, 1431495498, 137874439, 285222916]);
}

#[test]
fn test_b() {
    let gen_a = Generator { value : 65, factor: 16807};
    let gen_b = Generator { value : 8921, factor: 48271};
    assert_eq!(gen_a.into_picky(4).take(5).collect::<Vec<_>>(), vec![1352636452, 1992081072, 530830436, 1980017072, 740335192]);
    assert_eq!(gen_b.into_picky(8).take(5).collect::<Vec<_>>(), vec![1233683848, 862516352, 1159784568, 1616057672, 412269392]);
}
