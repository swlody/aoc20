fn get_seat_id(seat: &str) -> u32 {
    let (row, column) = seat.split_at(7);
    let (mut min, mut max, mut row_id) = (0, 127, 0);
    for division in row.chars() {
        let difference = max - min;
        if division == 'F' {
            max = min + difference / 2;
            row_id = min;
        } else {
            assert_eq!('B', division);
            min += (difference + 1) / 2;
            row_id = max;
        }
    }
    let (mut min, mut max, mut column_id) = (0, 7, 0);
    for division in column.chars() {
        let difference = max - min;
        if division == 'L' {
            max = min + difference / 2;
            column_id = min;
        } else {
            assert_eq!('R', division);
            min += (difference + 1) / 2;
            column_id = max;
        }
    }
    row_id * 8 + column_id
}

pub fn solve_part1(input: &str) -> Option<u32> {
    input.lines().map(get_seat_id).max()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut seat_ids = input.lines().map(get_seat_id).collect::<Vec<_>>();
    seat_ids.sort_unstable();
    let mut last_seen = seat_ids[0];
    for &seat_id in &seat_ids[1..] {
        if seat_id != last_seen + 1 {
            return last_seen + 1;
        } else {
            last_seen = seat_id;
        }
    }
    unreachable!()
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
