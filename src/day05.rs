fn traverse_bsp(input: &str, initial_max: u32) -> u32 {
    let (mut min, mut max, mut id) = (0, initial_max, 0);
    for division in input.chars() {
        let difference = max - min;
        id = match division {
            'F' | 'L' => {
                max = min + difference / 2;
                min
            }
            'B' | 'R' => {
                min += (difference + 1) / 2;
                max
            }
            _ => panic!("Invalid character in input"),
        };
    }

    id
}

fn get_seat_id(seat: &str) -> u32 {
    let (rows, columns) = seat.split_at(7);
    let row_id = traverse_bsp(rows, 127);
    let column_id = traverse_bsp(columns, 7);

    row_id * 8 + column_id
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(get_seat_id).max().unwrap()
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
