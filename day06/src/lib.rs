use std::collections::HashSet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn program_a(input: &str) {
    for (idx, char) in input.chars().step_by(4).enumerate() {
        println!("{}, {}", idx, char);
        if idx != 0 {
            for x in input.chars().into_iter().enumerate() {
                
            }
        }
    }

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
    fn test_program_a() {
        program_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    }
}
