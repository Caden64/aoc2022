use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::Hash;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn priority(b :u8) -> usize {
    if b >=b'A' && b <=b'Z' {
        (b-b'A'+27) as usize
    } else {
        (b-b'a'+1) as usize
    }
}

pub fn program_b(input:&str) -> usize {
    let mut total: u32 = 0;
    let mut sets: Vec<HashSet<u8, RandomState>> = Vec::new();
    for line in input.lines() {
        sets.push(HashSet::from_iter(line.bytes()))
    }

    for set in sets.chunks(3) {
        let (a,b, c) = (&set[0],&set[1],&set[2]);
        let ab: HashSet<u8, RandomState> = HashSet::from_iter(a.intersection(b).cloned());
        let abc = ab.intersection(c).next().unwrap();
        total += priority(*abc) as u32
    }

    total as usize
}

pub fn program_a(input:&str) -> usize{
    let mut total: u32 = 0;
    for line in input.lines() {
        let v: Vec<u8> = line.bytes().collect();
        let (a,b) = v.split_at(v.len()/2);
        let a: HashSet<u8, RandomState> = HashSet::from_iter(a.iter().cloned());
        let b = HashSet::from_iter(b.iter().cloned());
        let both = a.intersection(&b).next().unwrap();
        total += priority(*both) as u32

    }
    total as usize

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn program() {
        program_a("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw")
    }
}
