use std::collections::VecDeque;

/// Naive vec edit operation
fn part_one(nplayers: usize, points_last: u32) -> u32 {
    let mut board = Vec::with_capacity(points_last as usize);
    let mut marbles = 0 ..= points_last;
    board.push(marbles.next().unwrap());

    let mut players: Vec<_> = (0..nplayers).map(|_| 0).collect();
    let mut iteration = 0;
    let mut current = 0;
    loop {
        let player = iteration % nplayers;
        if let Some(marble) = marbles.next() {
            if marble > 0 && marble % 23 != 0 {
                let position = (current + 2) % board.len();
                board.insert(position+1, marble);
                current = position;
            } else {
                players[player] += marble;
                let take = if current >= 7 {
                    current - 7
                } else {
                    (board.len() as isize + (current as isize - 7) % board.len() as isize) as usize
                };
                let m =  board.remove((take + 1) % board.len());
                players[player] += m;
                current = take  % board.len();
            }
        } else {
            return *players.iter().max().unwrap();
        }
        iteration += 1;
    }
}

/// optimized to two queue's that split the data between the current
/// cursor position and juggle elements left or right
fn part_two(nplayers: usize, points_last: u32) -> u32 {
    let mut board_left = VecDeque::new();
    let mut board_right = VecDeque::new();
    let mut marbles = 0 ..= points_last;

    board_left.push_back(marbles.next().unwrap());

    let mut players: Vec<_> = (0..nplayers).map(|_| 0).collect();
    let mut iteration = 0;
    loop {
        let player = iteration % nplayers;
        if let Some(marble) = marbles.next() {
            if marble > 0 && marble % 23 != 0 {
                //can we shift a marble right to left?
                if let Some(e) = board_right.pop_front() {
                    board_left.push_back(e);
                    board_left.push_back(marble);
                } else {
                    //board right is empty
                    std::mem::swap(&mut board_left, &mut board_right);
                    board_left.push_back(board_right.pop_front().unwrap());
                    board_left.push_back(marble);
                }
            } else {
                players[player] += marble;
                // try to move 7 elements from left to right
                let mut moved = 0;
                for _ in 0 .. 7 {
                    if let Some(e) = board_left.pop_back() {
                        board_right.push_front(e);
                        moved += 1;
                    }
                }
                if board_left.is_empty() {
                    std::mem::swap(&mut board_left, &mut board_right);
                    //place the remaing items from left to right
                    let pos = board_left.len() - 7 + moved;
                    board_right.extend(board_left.drain(pos..))
                }

                let take = board_left.pop_back().unwrap();
                //move cursor 1 to the right
                if let Some(n) = board_right.pop_front() {
                    board_left.push_back(n);
                } else {
                    std::mem::swap(&mut board_left, &mut board_right);
                }

                players[player] += take;
            }
        } else {
            return *players.iter().max().unwrap();
        }
        iteration += 1;
    }
}

fn main() {
    // 426 players; last marble is worth 72058 points
    // use naive algorithm
    println!("9a: Winner has {} points", part_one(426, 72058));

    // more marbles
    // 426 players; last marble is worth 7205800 points
    // use optimized juggler
    println!("9b: Winner has {} points", part_two(426, 7205800));
}

#[test]
fn test_one() {
    assert_eq!(part_one(9, 25), 32);
    assert_eq!(part_one(10, 1618), 8317);
    assert_eq!(part_one(17, 1104), 2764);
    assert_eq!(part_one(21, 6111), 54718);
    assert_eq!(part_one(30, 5807), 37305);
}

#[test]
fn test_two() {
    assert_eq!(part_two(9, 25), 32);
    assert_eq!(part_two(10, 1618), 8317);
    assert_eq!(part_two(17, 1104), 2764);
    assert_eq!(part_two(21, 6111), 54718);
    assert_eq!(part_two(30, 5807), 37305);
}
