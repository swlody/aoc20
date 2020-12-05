fn get_seat_id(seat: &str) -> u32 {
    seat.chars().rev().enumerate().fold(0, |id, (idx, c)| {
        id | match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("Invalid character in input"),
        } << idx
    })
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(get_seat_id).max().unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut seat_ids = input.lines().map(get_seat_id).collect::<Vec<_>>();
    seat_ids.sort_unstable();
    seat_ids.windows(2).find(|w| w[0] + 1 != w[1]).unwrap()[0] + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_id() {
        assert_eq!(357, get_seat_id("FBFBBFFRLR"));
        assert_eq!(567, get_seat_id("BFFFBBFRRR"));
        assert_eq!(119, get_seat_id("FFFBBBFRRR"));
        assert_eq!(820, get_seat_id("BBFFBBFRLL"));
    }
}
