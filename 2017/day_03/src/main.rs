#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Cell(u32);

impl Cell {
    fn neighbours(&self) -> Vec<Cell> {
        //generate neighbours assuming self is the last appended cell
        //previous is always there
        let p = self.0;
        let mut v = vec![Cell(p - 1)];

        let g = self.grid_size();
        let (x, y) = self.x_y();
        //println!("Get neigh for {}at {},{}, g={}", p, x, y, g);
        
        //Every spiral added to the grid has an offset of 8 added to the previous offset.
        //This offset starts specific for each side. (1, 3, 5, 7, the mid numbers from spiral around 1)
        //Corners are special
        //Distance 1 from corner also
        //
        //I'll write this out, but there are probably more clever ways
        
        if y == 0 {
            //top row
            let sub = ((g - 1) / 2 - 1) * 8 + 3;
            match x {
                0 => v.push(Cell(p - sub - 1)),
                1 => {
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub - 1));
                },
                xv if xv == g - 2 => {
                    v.push(Cell(p - 2));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                },
                xv if xv == g - 1 => {
                    v.push(Cell(p - sub + 1));
                },
                _ => {
                    v.push(Cell(p - sub - 1));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                }
            }
        } else if y == g - 1 {
            //bottom row
            let sub = ((g - 1) / 2 - 1) * 8 + 7;
            match x {
                0 => v.push(Cell(p - sub + 1)),
                1 => {
                    v.push(Cell(p - 2));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                },
               /* xv if xv == g - 2 => {
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub - 1));
                },*/
                xv if xv == g - 1 => {
                    v.push(Cell(p - sub - 1));
                    v.push(Cell(p - sub));
                },
                _ => {
                    v.push(Cell(p - sub - 1));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                }
            }
        } else if x == 0 {
            //left col excluding corners (handled above)
            let sub = ((g - 1) / 2 - 1) * 8 + 5;
            match y {
                1 => {
                    v.push(Cell(p - 2));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                },
                xv if xv == g - 2 => {
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub - 1));
                },
                _ => {
                    v.push(Cell(p - sub - 1));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                }
            }
        } else if x == g - 1 {
            //right col excluding corners (handled above)
            let sub = ((g - 1) / 2 - 1) * 8 + 1;
            match y {
                1 => {
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub - 1));
                },
                xv if xv == g - 3 => {
                    v.push(Cell(p - 2));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                },
                xv if xv == g - 2 => {
                    v.push(Cell(p - sub + 1));
                },
                _ => {
                    v.push(Cell(p - sub - 1));
                    v.push(Cell(p - sub));
                    v.push(Cell(p - sub + 1));
                }
            }
        }
        //println!("neigh: {:?}", v);
        v
    }

    fn grid_size(&self) -> u32 {
        let mut g = (self.0 as f64).sqrt().ceil() as u32;
        if g % 2 == 0 { g += 1; }
        g
    }

    fn x_y(&self) -> (u32, u32) {
        let p = self.0;
        let g = self.grid_size();

        let empty = g*g - p;
        //println!("p={}, g={}, empty={}", p, g, empty);
        match empty  {
            v if v < g => (g-empty-1, g-1),                             //botom row
            v if v < g+g-1 => (0, g - 1 - (empty - g + 1)),             //left column
            v if v < g+g+g-2 => (1 + (empty - (g + g - 1)), 0),         //top row 
            v if v < g+g+g+g => (g-1, 1 + (empty - (g + g -1 + g - 1))),//right column
            _ => unreachable!()
        }
    }
    
    fn dist_to_center(&self) -> u32 {
        let (x, y) = self.x_y();
        let g = self.grid_size();

        let mid = (g as i32 - 1) / 2;
        ((x as i32 - mid).abs() + (y as i32- mid).abs()) as u32
    }

    fn value(&self) -> Option<u32> {
        match self.0 {
            1 => Some(1),
            2 => Some(1),
            3 => Some(2),
            4 => Some(4),
            5 => Some(5),
            6 => Some(10),
            7 => Some(11),
            8 => Some(23),
            9 => Some(25),
            10 => Some(26),
            11 => Some(54),
            _ => None
        }
    }
}

fn get_cell_value(c: &Cell, mut cache: &mut Vec<Option<u32>>) -> u32 {
    let i = c.0 as usize;
    if let Some(v) = cache[i] {
        v
    } else if let Some(v) = c.value() {
        v
    } else {
        let v = c.neighbours().iter().map(|cell| {
            get_cell_value(cell, &mut cache)
        }).sum();
        cache[i] = Some(v);
        v
    }
}

fn main() {
    let target = 347991;
    //3a
    println!("Distance from cell {} to 1 = {}", target, Cell(target).dist_to_center());

    //3b in a messed up way. Because I cheated using the OEIS I made myself write the backwards
    //recursive solver for the spiral sum given a position. This is totally inconvenient to answer
    //the question (caching would make it a lot more effective) but who cares :-)
    //okoke stupid cache added.
    let s: Vec<Cell> = (0..100).map(|v| Cell(v)).collect();
    let mut cache = vec![None; 100];
    let p = s.binary_search_by(|cell| get_cell_value(cell, &mut cache).cmp(&target));
    match p {
        Err(i) if i == s.len() => println!("Not found, increase search space"),
        Ok(i) | Err(i) => println!("Next val is in spiral > {} = {}, cell={}", target, cache[i].unwrap(), i),
    }
}

#[test]
fn test_xy() {
    assert_eq!(Cell(25).x_y(), (4,4));
    assert_eq!(Cell(24).x_y(), (3,4));
    assert_eq!(Cell(22).x_y(), (1,4));
    assert_eq!(Cell(21).x_y(), (0,4));
    assert_eq!(Cell(20).x_y(), (0,3));
    assert_eq!(Cell(19).x_y(), (0,2));
    assert_eq!(Cell(17).x_y(), (0,0));
    assert_eq!(Cell(16).x_y(), (1,0));
    assert_eq!(Cell(15).x_y(), (2,0));
    assert_eq!(Cell(14).x_y(), (3,0));
    assert_eq!(Cell(13).x_y(), (4,0));
    assert_eq!(Cell(12).x_y(), (4,1));
    assert_eq!(Cell(11).x_y(), (4,2));
    assert_eq!(Cell(10).x_y(), (4,3));

    assert_eq!(Cell(9).x_y(), (2,2));
    assert_eq!(Cell(8).x_y(), (1,2));
    assert_eq!(Cell(7).x_y(), (0,2));
    assert_eq!(Cell(6).x_y(), (0,1));
    assert_eq!(Cell(5).x_y(), (0,0));
    assert_eq!(Cell(4).x_y(), (1,0));
    assert_eq!(Cell(3).x_y(), (2,0));
    assert_eq!(Cell(2).x_y(), (2,1));

    assert_eq!(Cell(1).x_y(), (0,0));
}

#[test]
fn test_dist() {
    assert_eq!(Cell(1).dist_to_center(), 0);
    assert_eq!(Cell(12).dist_to_center(), 3);
    assert_eq!(Cell(23).dist_to_center(), 2);
    assert_eq!(Cell(1024).dist_to_center(), 31);
}

#[test]
fn test_neighbours() {
    assert_eq!(Cell(65).neighbours(), vec![Cell(64), Cell(37)]);
    assert_eq!(Cell(77).neighbours(), vec![Cell(76), Cell(45), Cell(46), Cell(47)]);
    assert_eq!(Cell(23).neighbours(), vec![Cell(22), Cell(7), Cell(8), Cell(9)]);
    assert_eq!(Cell(14).neighbours(), vec![Cell(13), Cell(12), Cell(3), Cell(4)]);
    assert_eq!(Cell(24).neighbours(), vec![Cell(23), Cell(8), Cell(9), Cell(10)]);
    assert_eq!(Cell(21).neighbours(), vec![Cell(20), Cell(7)]);
    assert_eq!(Cell(25).neighbours(), vec![Cell(24), Cell(9), Cell(10)]);
    assert_eq!(Cell(26).neighbours(), vec![Cell(25), Cell(10)]);
    assert_eq!(Cell(27).neighbours(), vec![Cell(26), Cell(25), Cell(10), Cell(11)]);
}

#[test]
fn test_cellvalues() {
    let mut cache = vec![None; 100];
    assert_eq!(get_cell_value(&Cell(10), &mut cache), 26);
    assert_eq!(get_cell_value(&Cell(11), &mut cache), 54);
    assert_eq!(get_cell_value(&Cell(17), &mut cache), 147);
    assert_eq!(get_cell_value(&Cell(23), &mut cache), 806);
    assert_eq!(get_cell_value(&Cell(24), &mut cache), 880);
    assert_eq!(get_cell_value(&Cell(25), &mut cache), 931);
    assert_eq!(get_cell_value(&Cell(26), &mut cache), 957);
    assert_eq!(get_cell_value(&Cell(27), &mut cache), 1968);
    assert_eq!(get_cell_value(&Cell(30), &mut cache), 2391);
}

