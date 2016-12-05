struct Keypad {
    number: u8
}

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'L'=> Direction::Left,
            'U'=> Direction::Up,
            'R'=> Direction::Right,
            'D'=> Direction::Down,
            n => panic!(format!("Unknown direction in input: '{}'", n))
        }
    }
}

impl Keypad {
    fn new() -> Keypad {
        Keypad { number: 5 }
    }

    fn go<D: Into<Direction>>(&mut self, di: D) {
        let d = di.into();
        match d {
            Direction::Left if self.can_left() => self.number -=1,
            Direction::Up if self.can_up() => self.number -= if self.number < 4 || self.number == 13 {2} else {4},
            Direction::Right if self.can_right() => self.number += 1,
            Direction::Down if self.can_down() => self.number += if self.number > 9 || self.number == 1 {2} else {4},
            _ => {}
        }
    }

    fn can_left(&self) -> bool {
        let n = self.number;
        n != 1 && n != 2 && n != 5 && n != 10 && n != 13
    }

    fn can_up(&self) -> bool {
        let n = self.number;
        n != 1 && n != 2 && n != 4 && n != 5 && n != 9
    }

    fn can_right(&self) -> bool {
        let n = self.number;
        n != 1 && n != 4 && n != 9 && n != 12 && n != 13
    }

    fn can_down(&self) -> bool {
        let n = self.number;
        n != 5 && n != 9 && n != 10 && n != 12 && n != 13
    }

    fn enter_sequence<S>(&mut self, sequence: S) where S: Iterator<Item=char> {
        for m in sequence {
            self.go(m);
        }
    }

    fn get_number_char(&self) -> char {
        match self.number {
            n if n < 10 => (n + 48) as char,
            n => (n + 55) as char
        }
    }
}

fn main() {
    let input = "\
RLRLLLULULULUUDUULULRDDLURURDDLDUUDDLRDDUUUDDRUDLRRDDUDUUDULUDRDULRUDRULRDRUDLDDULRRDLDRLUDDLLDRDDDUDDLUDUDULDRLLDRLULRLURDLULRUUUDRULLUUDLRDLDDUDRRRLDLRUUURRLDDRRRURLLULDUULLDRLRDLLDURDLDDULLDDLDLUURRRURLRURLLRRDURLDUDDLULUUULULLLDRRRRRLULRDUDURURLULRURRRLLUURDURULRRUULDRDLULDLLUDLUDRLUDLRRLDLLDLDUDDLULLDRULRLRULDURRDLDLLUDRLLDRRDLDUDUURUURDUUDDDLDLDDRDLUDLDUUUUDLDRLRURDLURURDLLLUURURDRDLUDLLRUDULLLDLULLULLDLDDRDRRRUDDDUDDDDRULLLLRLDDLLRDRLLLRRLDRRUDRUUURLLLRULRRDURDLDRLDDUUDUUURRLRRUDLDLDDRUDLULLUUDUUUDLUDDRUULLLURUDDDDLRUDDLLLRUR
LDLRLDDDLUDRDRRUDUURLRULLUDDRLURLUULDLLRLLUDLRLRUDLULRLRRLRURLDDDURUDUUURDRLDDLUUUDRUDUDDDLLURLLULRUULLUDRULUDDULDUDUDULLDRUUUULRDUUDLUDURDLLRLLRLUUDUUDRLLLRULUURUDLDRLLDUDLDDRULDULDURRLDDDUDUDDRUDUDRDURLLLLLULDRDDLLUDULLLUDRURLDLDLDULLDDRURRLUDDRLURLULRLDDDUUUURLRDLRURDDURLDLRRLLRLRLUURRLLDDLDRLRDUDDLLDDDURUUDURLRRDUULRRDDRRUULDRLRUDRRLDDRLDRULLDLDURRULDURRRDLRRLRLLLRLDRLLULRRLLLLLDLDDULDLLDLLDUUDDRLURUUUUULRDDLRDLRDRDRDLUDDLDDRULLUDDRLDLLUDRLUURRLUDURURLLRURRURRLRLLRLURURDDDDRRLURDUULLUU
LLRRDURRDLDULRDUDLRDRDRURULDURUDRRURDDDRLDLDRDRDRDRULDUURLULDDUURUULUDULLDUDLLLLDLLLDRLUUULLULDDRRUDDULLLULRDRULDDULDUDRDDLUUURULDLLUDUUUUURUDLLDRDULLRULLDURDRLLDLDRDDURUULUDURRRUULLDUUDDURDURLDLRRLLDURDDLRRRUDLRRRDLDRLUDLUDRDRLDDLLLRLLRURDLRDUUUURRLULDDLDLLLUDRDRLRRDURDDLURDLDDDULLLRRLDDDRULDDDLRRDULUUUDRRULDDLLLURDRRLLLUULDRRRUURRDDLULDRLULDDDLDULDRRRULRULLURLURULLLLRUDRRRDRDRDLDULURLRRRRLRUDDRRRUURUURLLRURURUURRURRDLDLLUDRRRDUDDRDURLLRLRRULD
DULRRDRLRLUDLLURURLLRLRDLLDLLDRDUURLRUUUDLLDUUDDUULDUULLRUDRURLUDRDLRUDDDLULUDLLDRULULLLDRRULDLLUURLRRRLDRDLDRURRRRDLRUUDULLRLLLDLRUDLDUUDRLDLRDRLRDLDDDUDLRUDLDDLLLDRLLRRUUDRDDUUURURRRUUDLRRDDRUDLDDULULDLRRLRDDUDRUURRUULURLURUDRRURRRULDDDDURDLUUULUULULRDLRRRRRURURRLRUULDUUURRDRRDLDUUUULLULLLLUDLUUDUURRDLDLRRRLUUURULDULDLDRLLURDRUULLLLLULLLDRURURRUDRRRRUDUDUDRUDUDRDRULUUDRURDDUUDLDLDUURUDURLRLRRDRDRDLLDUDDULLRDLDDRLLDLRDURDDULLLDLLLULDLUUUDLDRDLURUURDDLRDLLLLLRLURDLLLULLRRLU
DUULULUUDUDLLRLRURULLDLRRLURDLLDUDUDDRURRLUDULULDRRDRLUULUDDLUURURDLDDDRDRUDURLDDLUDUURULRRUUDRLURRLRLDURRRULRLDDDRUDDDDDUDDULLLRRLLDULDRULUDLRRDLLUDRDLDULRLLLUULLRULRLLLLUDDRRDRLULDLDLURDDRUDDLDLDLDRULDLLDDUUDULUULULLURDURRLLUDRULLRDUDRDRURDRDRDURUUDULDDRURUDLLUUDUUDURDLRDRURUDRUURLUUURLRLUDRUDRUURLLUDRLURDDURRUDRDRLRRLDDDRDDLUUUDDLULDUURUDUDLLDRURDURRDULRLURRDLDDRLUDRLDLRLDDUURRULDDLDUDDLRDULLDDDLDUUUUDLRUDUDLDRDLRDDLDLRLLUDDRRLUDLDUUULLDDRLRRDLRRRRUDDLRLLULRLRDURDUDDRRULLDDLDLRRDLLULDURURDDURLRLULULURRUDUDRDLURULDUDLUULDUUURLLRUDLLRDLRUDRLULDUDRRDUUDUUULUUUDDRUD
";

    let mut k = Keypad::new();

    let code: String = 
    input.lines().map(|l| {
        k.enter_sequence(l.chars());
        k.get_number_char()
    }).collect();

    
    println!("The code is {}.", code);
}

#[test]
fn test() {
    let input = "\
ULL
RRDDD
LURDL
UUUUD
";
    let mut k = Keypad::new();
    let code: String = 
    input.lines().map(|l| {
        k.enter_sequence(l.chars());
        k.get_number_char()
    }).collect();

    assert_eq!(code,"5DB3");
}