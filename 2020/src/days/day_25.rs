use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

use nom::{
    character::complete::line_ending,
    combinator::all_consuming,
    sequence::{terminated, tuple},
    IResult
};

#[derive(Default)]
pub struct Solution {
    card_key: usize,
    lock_key: usize,
}


fn parse(i: &str) -> IResult<&str, (usize, usize)> {
    let pubkey1 = terminated(positive_integer, line_ending);
    let pubkey2 = terminated(positive_integer, line_ending);
    all_consuming(tuple((pubkey1, pubkey2)))(i)
}

fn transform(sj: usize, ls: usize) -> usize {
    (0..ls).fold(1, |mut val, _l| { val *= sj; val %= 20201227; val })
}

fn crack_ls(sj: usize, key: usize) -> usize {
    (1..).scan(1, |val, _ls| { *val *= sj; *val %= 20201227; Some(*val) })
        .position(|v| v == key).unwrap() + 1
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.card_key = result.1.0;
        self.lock_key = result.1.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let ls_card = crack_ls(7, self.card_key);
        Ok(format!("{}", transform(self.lock_key, ls_card)))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("Merry X-mas"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1() {
        let input = "5764801\n17807724\n";
        let result = parse(&input);
        assert!(result.is_ok());
        let data = result.unwrap().1;

        let card_ls = crack_ls(7, data.0);
        assert_eq!(card_ls, 8);
        let door_ls = crack_ls(7, data.1);
        assert_eq!(door_ls, 11);

        let enc_key_door = transform(data.0, 8);
        let enc_key_card = transform(data.1, 11);
        assert_eq!(enc_key_door, enc_key_card);
        assert_eq!(enc_key_door, 14897079);
    }
}
