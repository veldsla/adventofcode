#![feature(wrapping)]

extern crate core;

use std::collections::BTreeMap;
use std::u64::MAX;
use core::num::wrapping::OverflowingOps;

fn qe(v: &Vec<u32>) -> u64 {
    let mut  mul = 1;
    for e in v {
        let (res, o) = mul.overflowing_mul(*e as u64);
        if o {
            return MAX;
        } else {
            mul = res;
        }
    }
    mul
}

fn weight(v: &Vec<u32>) -> u32 {
    v.iter().fold(0, |sum , x| sum + *x)
}

type Set=Vec<u32>;

fn perm_fill(used: Vec<u32>, avail: Vec<u32>, remaining: u32, wantlevel: usize, resset: &mut BTreeMap<u64,Vec<Set>>) {
    //end rule
    if used.len() == wantlevel {
        if remaining == 0 {
            let setqe = qe(&used);
            let entry = resset.entry(setqe).or_insert(Vec::new());
            entry.push(used);
            return;
        
        } else {
            return;
        }
    }

    let mut subv = avail.iter().cloned().filter(|v| *v <= remaining).collect::<Vec<u32>>();
    loop {
        match subv.pop() {
            Some(v)  => {
                let mut newused = used.clone();
                newused.push(v);
                perm_fill(newused, subv.clone(), remaining - v, wantlevel, resset);
            },
            None => break
        }
    }
}

fn solve(vec: Vec<u32>, ngroups: u32, currentgroup: usize) -> bool {
    //end rule
    //println!("{}, {}, {}", ngroups, currentgroup, vec.len());
    if ngroups == 1 {
        return true;
    }

    //we find the smallest set that gives us sum(vec) / ngroups weight
    let total_weight = weight(&vec);
    let per_group = total_weight / ngroups;
    assert_eq!(per_group * ngroups, total_weight);

    //use 17b like algo to get the different groups of a given size that sum to: per_group
    for size in 1..vec.len() {
        let mut sets = BTreeMap::<u64, Vec<Set>>::new();
        perm_fill(Vec::new(), vec.clone(), per_group, size, &mut sets);
        if sets.is_empty() {
            continue;
        }

        //the BTreemap is ordered by QE start solving remainder for
        //for nrgoups - 1
        for (qe, sets) in sets.iter() {
            for set in sets {
                //if we can divide the remaining values in ngroups equally sized sets were done
                //nast method to create remaing values vec
                //no dups are present in test set so a filter action does the trick
                let remain = vec.iter().cloned().filter(|x| !set.contains(&x)).collect::<Vec<u32>>();
                let res = solve(remain, ngroups - 1, currentgroup + 1);
                if res {
                    if currentgroup == 1 {
                        println!("Solution for qe: {}, Group 1: {:?}", qe, set);
                        return true
                    } else {
                        return true;
                    }

                }
            }
        }
    }
    false
}

fn main() {
    let weights = vec![1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
        59, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];

    solve(weights.clone(),3, 1);
    solve(weights.clone(),4, 1);
}

