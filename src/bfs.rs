use crate::vec2d::Vec2D;
use std::collections::VecDeque;
use std::ops::Add;

const MOVE_DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn bfs<'grid, T>(
    grid: &'grid Vec2D<T>,
    start: (isize, isize),
    end: (isize, isize),
) -> Option<i32>
where
    T: PartialOrd,
    &'grid T: Add<i32, Output = T>,
{
    let mut visited = Vec2D::new_sized_with(
        grid.positive_width(),
        grid.positive_height(),
        grid.negative_width(),
        grid.negative_height(),
        false,
    );

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((coordinate, path_length)) = queue.pop_front() {
        // We found the end
        if coordinate == end {
            return Some(path_length);
        }

        // Move
        for (move_row, move_column) in MOVE_DIRECTIONS {
            let next_coordinate = (coordinate.0 + move_row, coordinate.1 + move_column);

            let Some(next_value) = grid.at(next_coordinate.0, next_coordinate.1) else { continue; };
            let value = grid.at_unchecked(coordinate.0, coordinate.1) + 1;

            if &value >= next_value && !*visited.at_unchecked(next_coordinate.0, next_coordinate.1)
            {
                *visited.at_mut_unchecked(next_coordinate.0, next_coordinate.1) = true;
                queue.push_back((next_coordinate, path_length + 1))
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_find_shortest_path() {
        let grid = Vec2D::from(
            vec![
                0, 1, 1, 1, 1, 1, //
                0, 0, 0, 1, 1, 1, //
                0, 1, 0, 1, 1, 1, //
                0, 0, 1, 1, 1, 1, //,
                1, 0, 0, 1, 1, 1, //
                1, 0, 0, 0, 1, 1, //
            ],
            6,
            6,
        );

        if let Some(path_length) = bfs(&grid, (0, 0), (5, 3)) {
            assert_eq!(8, path_length);
        } else {
            panic!("No path? :(");
        }
    }
}
