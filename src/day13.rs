pub fn solve_part1(input: &str) -> u32 {
    let (earliest, ids) = input.split_once('\n').unwrap();
    let earliest = earliest.parse().unwrap();
    let ids = ids.split(',').filter_map(|x| x.parse().ok());

    let mut min = u32::MAX;
    let mut candidate = 0;

    for id in ids {
        let mut time = 0;
        while time < earliest {
            time += id;
        }
        if time < min {
            min = time;
            candidate = id;
        }
    }

    candidate * (min - earliest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(295, solve_part1("939\n7,13,x,x,59,x,31,19"));
    }
}
