extern crate itertools;

use itertools::Itertools;

#[derive(Debug)]
struct FuelCells {
    cells: Vec<i32>,
    dim: usize
}

fn make_power(serial: i32) -> impl Fn(usize, usize) -> i32 {
    move |x, y| {
        let rack_id = x as i32 + 10;
        let hp = (rack_id * y as i32 + serial) * rack_id;
        let h = hp / 100;
        h - (h/10) * 10 - 5
    }
}


impl FuelCells {
    fn new(dim: usize, serial: i32) -> FuelCells {
        let power = make_power(serial);
        let cells = (1..=dim)
            .cartesian_product(1..=dim)
            .map(|(y, x)| power(x, y))
            .collect();

        FuelCells { cells, dim }
    }

    fn highest_square(&self, size: usize) -> (i32, usize, usize) {
        //group by column of size values
        if size == self.dim {
            return (self.cells.iter().sum::<i32>(), 1, 1);
        }
        let end = self.dim - size;
        let colsums: Vec<i32> = (0..end)
            .cartesian_product(0..self.dim)
            .map(|(y, x)| {
                //zero based x/y
                (y..y+size).map(|row| self.cells[x + row*self.dim]).sum()
            }).collect();

        colsums.windows(size)
            .enumerate()
            .map(|(pos, v)| {
                let sum = v.iter().sum();
                let x = pos % self.dim;
                let y = pos / self.dim;
                (sum, x+1, y+1)
            })
            .max_by_key(|e| e.0)
            .unwrap()
    }

    fn maximum_power_square(&self) -> (i32, usize, usize, usize) {
        (1..=self.dim).map(|size| {
            let (pow, x, y) = self.highest_square(size);
            (pow, size, x, y)
        }).max_by_key(|e| e.0).unwrap()
    }
}


fn main() {
    let input = 8561;
    let f = FuelCells::new(300, input);
    let (pow, x, y) = f.highest_square(3);
    println!("11a: Highest power({}) at coordinate {},{}", pow, x, y);
    let (pow, size, x, y) = f.maximum_power_square();
    println!("11b: Maximum power({pow}) at coordinate {x},{y} with a size {size}, id {x},{y},{size}",
        pow=pow, size=size, x=x, y=y);

}

mod test {
    use super::*;

    #[test]
    fn power() {
        let p = make_power(8);
        assert_eq!(p(3,5), 4);
        let p = make_power(57);
        assert_eq!(p(122,79), -5);
        let p = make_power(39);
        assert_eq!(p(217,196), 0);
        let p = make_power(71);
        assert_eq!(p(101,153), 4);
    }

    #[test]
    fn three_by_three() {
        let f = FuelCells::new(300, 18);
        assert_eq!(f.highest_square(3), (29, 33, 45));

        let f = FuelCells::new(300, 42);
        assert_eq!(f.highest_square(3), (30, 21, 61));
    }

    #[test]
    fn max_square() {
        let f = FuelCells::new(300, 18);
        assert_eq!(f.maximum_power_square(), (113, 16, 90, 269));

        let f = FuelCells::new(300, 42);
        assert_eq!(f.maximum_power_square(), (119, 12, 232, 251));
    }
}
