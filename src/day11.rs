#[derive(Copy, Clone, PartialEq)]
enum Spot {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Spot {
    fn from(spot: char) -> Self {
        match spot {
            'L' => Spot::Empty,
            '.' => Spot::Floor,
            '#' => Spot::Occupied,
            c => panic!("Invalid character in input: {}", c),
        }
    }
}

#[derive(Clone)]
pub struct Matrix {
    rows: Vec<Vec<Spot>>,
    num_rows: usize,
    num_columns: usize,
}

pub fn input_generator(input: &str) -> Matrix {
    let rows = input
        .lines()
        .map(|row| row.chars().map(Spot::from).collect())
        .collect::<Vec<Vec<_>>>();
    let num_rows = rows.len();
    let num_columns = rows[0].len();

    Matrix {
        rows,
        num_rows,
        num_columns,
    }
}

impl Matrix {
    fn apply_rules<F>(&mut self, count_occupied: F, max_adjacent: u32) -> Option<u32>
    where
        F: Fn(&Self, usize, usize) -> u32,
    {
        let mut new_rows = Vec::with_capacity(self.num_rows);

        let mut unchanged = true;
        let mut occupied_count = 0;
        for row_idx in 0..self.num_rows {
            let mut new_row = Vec::with_capacity(self.num_columns);
            for seat_idx in 0..self.num_columns {
                let new_state = match (
                    self.rows[row_idx][seat_idx],
                    count_occupied(&self, row_idx, seat_idx),
                ) {
                    (Spot::Floor, _) => Spot::Floor,
                    (Spot::Occupied, adjacent_count) if adjacent_count >= max_adjacent => {
                        unchanged = false;
                        Spot::Empty
                    }
                    (Spot::Empty, 0) => {
                        unchanged = false;
                        Spot::Occupied
                    }
                    (current_state, _) => current_state,
                };

                if new_state == Spot::Occupied {
                    occupied_count += 1;
                }

                new_row.push(new_state);
            }
            new_rows.push(new_row)
        }

        if unchanged {
            Some(occupied_count)
        } else {
            self.rows = new_rows;
            None
        }
    }

    fn count_adjacent(&self, row_idx: usize, seat_idx: usize) -> u32 {
        use std::cmp::min;

        let mut adjacent_count = 0;
        for i in row_idx.saturating_sub(1)..=min(row_idx + 1, self.num_rows - 1) {
            for j in seat_idx.saturating_sub(1)..=min(seat_idx + 1, self.num_columns - 1) {
                if !(i == row_idx && j == seat_idx) && self.rows[i][j] == Spot::Occupied {
                    adjacent_count += 1;
                }
            }
        }
        adjacent_count
    }

    fn count_visible(&self, row_idx: usize, seat_idx: usize) -> u32 {
        use std::convert::TryFrom;

        const STEPS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let (i, j) = (
            i32::try_from(row_idx).unwrap(),
            i32::try_from(seat_idx).unwrap(),
        );
        let mut visible = 0;
        for (dx, dy) in STEPS.iter() {
            let (mut i, mut j) = (i, j);
            loop {
                i += dx;
                j += dy;
                match self
                    .rows
                    .get(i as usize)
                    .and_then(|row| row.get(j as usize))
                {
                    Some(Spot::Occupied) => {
                        visible += 1;
                        break;
                    }
                    Some(Spot::Floor) => {}
                    Some(Spot::Empty) | None => break,
                }
            }
        }
        visible
    }
}

pub fn solve_part1(input: &Matrix) -> u32 {
    let mut matrix = input.clone();
    loop {
        if let Some(occupied_count) = matrix.apply_rules(Matrix::count_adjacent, 4) {
            return occupied_count;
        }
    }
}

pub fn solve_part2(input: &Matrix) -> u32 {
    let mut matrix = input.clone();
    loop {
        if let Some(occupied_count) = matrix.apply_rules(Matrix::count_visible, 5) {
            return occupied_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(37, solve_part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        assert_eq!(26, solve_part2(&input));
    }
}
