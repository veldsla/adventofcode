use std::cmp::{min, max};
use std::fs::File;
use std::io::{self, BufReader, BufRead, BufWriter};
use std::path::Path;
use std::str::FromStr;

use ndarray::{Array, Array2, s};
use png::{self, HasParameters};

struct Ground {
    data: Array2<char>,
    offset_x: usize,
    min_y: usize
}

enum Vein {
    Horizontal(usize, usize, usize),
    Vertical(usize, usize, usize),
}

fn parse_io_error(e: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
}

impl FromStr for Vein {
    type Err = io::Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let vertical = line.chars().nth(0) == Some('x');
        //vertical vein
        let mut e = line.split(|c| (c as u8) < 48 || (c as u8) > 57).filter(|s|!s.is_empty());
        let pos = e.next()
            .ok_or_else(|| parse_io_error("Error vein"))
            .and_then(|e| e.parse().map_err(|_| parse_io_error("Error in vein")))?;
        let from = e.next()
            .ok_or_else(|| parse_io_error("Error vein"))
            .and_then(|e| e.parse().map_err(|_| parse_io_error("Error in vein")))?;
        let to = e.next()
            .ok_or_else(|| parse_io_error("Error vein"))
            .and_then(|e| e.parse().map_err(|_| parse_io_error("Error in vein")))?;
        if vertical {
            Ok(Vein::Vertical(pos, from, to))
        } else {
            Ok(Vein::Horizontal(pos, from, to))
        }
    }
}


impl Ground {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Ground> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let veins = b.lines().map(|l| {
            l.and_then(|l| {
                l.parse()
            })
        }).collect::<Result<Vec<Vein>, _>>()?;
        //determine the range of the scan 500, 0 exists (spring)
        let (minx, miny, maxx, maxy) = veins.iter().fold((500, 500, 500, 0), |acc, v| {
            match v {
                Vein::Vertical(x, y0, y1) => (min(acc.0, *x), min(acc.1, *y0), max(acc.2, *x), max(acc.3, *y1)),
                Vein::Horizontal(y, x0, x1) => (min(acc.0, *x0), min(acc.1, *y), max(acc.2, *x1), max(acc.3, *y)),
            }
        });

        //create the matrix add 2 to x left and right and 1 to bottom
        let mut data = Array::from_elem((maxx - minx + 3, maxy + 2), '.');
        let offset_x = minx - 1;
        //fill in the veins
        for v in veins {
            match v {
                Vein::Vertical(x, y0, y1) => {
                    data.slice_mut(s![x-minx+1..x-minx+2, y0..y1+1]).iter_mut().for_each(|e| *e = '#');
                },
                Vein::Horizontal(y, x0, x1) => {
                    data.slice_mut(s![x0-minx+1..x1-minx+2, y..y+1]).iter_mut().for_each(|e| *e = '#');
                }
            }
        }

        data[[500-offset_x,0]] = '+';

        Ok(Ground { data, offset_x: minx - 1, min_y: miny})
    }

    /// flow the water from the spring (500,0)
    /// method
    /// flow down, push each position to source queue
    ///  - if clay
    ///     pop last source and flow rowbased
    ///         if overflows append to queue, it will flow down from there
    ///         basically resulting in a dfs
    fn flow_spring(&mut self) {
        let mut sources = Vec::new();
        sources.push((500-self.offset_x,0));
        let maxy = self.data.dim().1;

        while let Some((x, y)) = sources.pop() {
            if y < maxy -1 && self.data[[x,y+1]] == '.' {
                //flow down
                let mut downy = y+1;
                while downy < maxy &&  self.data[[x,downy]] == '.' {
                    self.data[[x,downy]] = '|';
                    sources.push((x, y));
                    sources.push((x, downy));
                    downy += 1;
                }
            } else if y < maxy -1 && self.data[[x,y+1]] != '|' {
                self.fill_row(x, y, &mut sources);
            }
        }
    }

    fn fill_row(&mut self, x: usize, y:usize, sources: &mut Vec<(usize, usize)>) {
        //if the row is contained within # we fill with ~ if not use |
        let maxx = self.data.dim().0;
        let mut left = x-1;
        loop {
            if left == 0 {
                sources.push((0,y));
                break;
            } else if self.data[[left, y+1]] == '.' {
                sources.push((left,y));
                break;
            } else if self.data[[left,y]] == '#' {
                left += 1;
                break;
            } else if self.data[[left,y]] == '|' && self.data[[left, y+1]] == '|' {
                break;
            }
            left -=1 ;
        }

        let mut right = x+1;
        loop {
            if right == maxx-1 {
                sources.push((right,y));
                break;
            } else if self.data[[right, y+1]] == '.' {
                sources.push((right,y));
                right += 1;
                break;
            } else if self.data[[right,y]] == '#' {
                break;
            } else if self.data[[right,y]] == '|' && self.data[[right, y+1]] == '|' {
                break;
            }

            right +=1 ;
        }
        
        let fill = if left > 0 && self.data[[left-1,y]] == '#' && self.data[[right,y]] == '#' { '~' } else { '|' };
        self.data.slice_mut(s![left..right, y..y+1]).iter_mut().for_each(|e| *e = fill);
    }

    fn count_water(&self) -> usize {
        let (width, height) = self.data.dim();
        self.data.t().iter().skip(self.min_y * width).take(width * (height - self.min_y - 1)).filter(|&&c| c == '~' || c == '|').count()
    }

    fn count_trapped_water(&self) -> usize {
        self.data.t().iter().filter(|&&c| c == '~').count()
    }

    fn to_png<P: AsRef<Path>>(&self, p: P) -> Result<(), png::EncodingError> {
        let file = File::create(p)?;
        let ref mut w = BufWriter::new(file); 
        let mut encoder = png::Encoder::new(w, self.data.dim().0 as u32, self.data.dim().1 as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        //this seems pretty stupid, but it works
        let data:Vec<u8> = self.data.t().iter().flat_map(|&c| {
            match c {
                '.' => vec![0,0,0, 255],
                '#' => vec![255,255,255, 255],
                '|' => vec![0,255,255, 255],
                '~' => vec![0,0,255, 255],
                '+' => vec![255,0,0, 255],
                _ => vec![0,0,0,255],
            }
        }).collect();
        writer.write_image_data(&data)
    }
}

fn main() {
    let mut ground = Ground::from_file("input.txt").unwrap();
    ground.to_png("start.png").unwrap();
    ground.flow_spring();
    ground.to_png("end.png").unwrap();
    println!("17a: Number of tiles with water is {}", ground.count_water());
    println!("17b: Number of tiles with trapped water is {}", ground.count_trapped_water());
}

#[test]
fn test() {
    let mut ground = Ground::from_file("test.txt").unwrap();
    println!("{}, {} {}", ground.data.t(), ground.offset_x, ground.min_y);
    ground.flow_spring();
    println!("{}, {}", ground.data.t(), ground.offset_x);
    assert_eq!(ground.count_water(), 57);
    assert_eq!(ground.count_trapped_water(), 29);
}
