pub fn solve_part1(input: &str) -> u32 {
    let (earliest, ids) = input.split_once('\n').unwrap();
    let earliest = earliest.parse::<u32>().unwrap();
    let ids = ids.split(',').filter_map(|x| x.parse().ok());

    let mut min_wait_time = u32::MAX;
    let mut candidate_id = 0;

    for id in ids {
        let wait_time = id - (earliest % id);
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            candidate_id = id;
        }
    }

    candidate_id * min_wait_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(295, solve_part1("939\n7,13,x,x,59,x,31,19"));
    }
}
