
use std::collections::{HashMap, HashSet, BTreeMap};
use std::str;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    combinator::all_consuming,
    sequence::{delimited, separated_pair, terminated},
    IResult
};

#[derive(Default)]
pub struct Solution {
    input: Option<Ingredients>,
}

type AllergenMap = (HashSet<String>, HashSet<String>);
#[derive(Debug)]
struct Ingredients(Vec<AllergenMap>);

fn decode(i :&Ingredients) -> (usize, String) {
    let mut alg_map: HashMap<&String, HashSet<&String>> = i.0.iter().fold(HashMap::new(), |mut acc, (ingr, alg)| {
        for a in alg {
            let e = acc.entry(a).or_insert_with(HashSet::new);
            e.extend(ingr.iter());
        }
        acc
    });

    for (&alg, ingr) in alg_map.iter_mut() {
        //find products listing this allergen
        //remove any ingredient not specified by this product
        for (oneof, _) in  i.0.iter().filter(|(_ing, algs)| algs.contains(alg)) {
            ingr.retain(|alg_ingr| oneof.contains(alg_ingr.as_str()));
        }
    }

    let maybe_alg: HashSet<&String> = alg_map.values().flat_map(|s| s.iter().map(|&s| s)).collect();
    let all_ing: HashSet<&String> = i.0.iter().flat_map(|(ing,  _alg)| ing.iter()).collect();
    let not_ing: HashSet<String> = all_ing.difference(&maybe_alg).map(|s| s.to_string()).collect();
    let part1 = i.0.iter().map(|(ing, _alg)| ing.intersection(&not_ing).count()).sum();


    //solve part 2 using the alg_map
    let mut known = BTreeMap::new();
    let mut known_ingredients = HashSet::new();
    while !alg_map.is_empty() {
        //remove ingredients that have been identified from still ambigious algs
        //also allergens that have more than 1 candidate, but one candidates is unique in the map
        //are known
        for (k, v) in &alg_map {
            if v.len() == 1 {
                known.insert(k.to_owned(), v.iter().next().unwrap().to_owned());
                known_ingredients.insert(k.to_owned());
            } else if let Some(ingr) = v.iter().find(|&&ingr| alg_map.iter().filter(|(_, i)| i.contains(ingr)).count() == 1) {
                known.insert(k.to_owned(), ingr.to_owned());
                known_ingredients.insert(k.to_owned());
            }
        }
        //might have jumped a hoop to many to avoid some borrow issues......
        alg_map.retain(|k, _v| !known.contains_key(k));
        alg_map.iter_mut().for_each(|(_k, v)| v.retain(|i| !known_ingredients.contains(i)));
    }
    let part2 = known.values().join(",");

    (part1, part2)
}

fn parse(i: &str) -> IResult<&str, Ingredients> {
    let ingredients = map(separated_list1(space1, alpha1), 
        |v: Vec<&str>| v.into_iter().map(|s| s.to_owned()).collect());
    let allergens = map(separated_list1(terminated(char(','), space0), alpha1),
        |v: Vec<&str>| v.into_iter().map(|s| s.to_owned()).collect());
    let product = separated_pair(
        ingredients,
        space1,
        delimited(tag("(contains "), allergens, char(')')));
    all_consuming(map(many1(terminated(product, line_ending)), Ingredients))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.input = Some(result.1);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("See part2"))
    }

    fn part2(&self) -> Result<String> {
        let data = self.input.as_ref().ok_or_else(|| anyhow!("Not parsed"))?;
        let (p1, p2) = decode(data);
        Ok(format!("{}\t{}", p1, p2))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
    #[test]
    fn p1() {
        let result = parse(TEST);
        println!("{:?}", result);
        assert!(result.is_ok());
        let data = result.unwrap().1;
        let (p1, p2) = decode(&data);
        assert_eq!(p1, 5);
        assert_eq!(p2, "mxmxvkd,sqjhc,fvjkl");
    }

}

