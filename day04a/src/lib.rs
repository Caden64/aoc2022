pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn program_a(input: &str) -> u32 {
    let temp = input.replace(",", "-");
    let mut total: u32 = 0;
    for line in temp.lines() {
        let mut split = line.split("-");
        let values = (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap());
        if (values.0 <=values.2 && values.1 >=values.3) || (values.2 <=values.0 && values.3 >=values.1) {
            total += 1
        }
    }
    total
}

pub fn program_b(input: &str) -> u32 {
    let temp = input.replace(",", "-");
    let mut total: u32 = 0;
    for line in temp.lines() {
        let mut split = line.split("-");
        let values = (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap());
        if (values.0 >= values.2 && values.0 <= values.3) || (values.1 >= values.2 && values.1 <= values.3) || (values.0 <=values.2 && values.1 >=values.3) || (values.2 <=values.0 && values.3 >=values.1) {
            total += 1
        }
    }
    total
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
        let result = program_a("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(result, 2)
    }
    #[test]
    fn test_program_b() {
        let result = program_b("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(result, 4)
    }
}
