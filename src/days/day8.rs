use std::{cmp::max, collections::HashSet};

use crate::util;
use array2d::{self, Array2D};

const DAY_STR: &str = "Day 8";

fn vis_from_axis(grid: &Array2D<i32>) -> usize {
    let mut vis_trees: HashSet<(i32, i32)> = HashSet::new();
    let clen = grid.column_len() - 1;
    let rlen = grid.row_len() - 1;

    // left and right
    for row in 0..=clen {
        let (mut left_max, mut right_max): (&i32, &i32) = (&-1, &-1);
        for col in 0..=clen {
            let ltree = grid.get(row, col).unwrap_or(&0);
            let rtree = grid.get(row, clen - col).unwrap_or(&0);

            if ltree > left_max {
                left_max = ltree;
                vis_trees.insert((row as i32, col as i32));
            }

            if rtree > right_max {
                right_max = rtree;
                vis_trees.insert((row as i32, (clen - col) as i32));
            }
        }
    }

    // top and bottom
    for col in 0..=clen {
        let (mut top_max, mut bot_max): (&i32, &i32) = (&-1, &-1);
        for row in 0..=rlen {
            let ttree = grid.get(row, col).unwrap_or(&0);
            let btree = grid.get(rlen - row, col).unwrap_or(&0);

            if ttree > top_max {
                top_max = ttree;
                vis_trees.insert((row as i32, col as i32));
            }

            if btree > bot_max {
                bot_max = btree;
                vis_trees.insert(((rlen - row) as i32, col as i32));
            }
        }
    }

    return vis_trees.len();
}

fn scenic_score(grid: &Array2D<i32>) -> usize {
    let mut score = 0;

    for c in 0..grid.column_len() {
        for r in 0..grid.row_len() {
            let mut right = 0;
            let mut left = 0;
            let mut top = 0;
            let mut bot = 0;
            let cur = grid.get(c, r).unwrap();

            // right view
            for w in r + 1..grid.row_len() {
                let neigh = grid.get(c, w).unwrap();

                right += 1;
                if neigh >= cur {
                    break;
                }
            }

            // left view
            if r > 0 {
                for x in (0..=r - 1).rev() {
                    let neigh = grid.get(c, x).unwrap();

                    left += 1;
                    if neigh >= cur {
                        break;
                    }
                }
            }

            // top view
            if c > 0 {
                for y in (0..=c - 1).rev() {
                    let neigh = grid.get(y, r).unwrap();

                    top += 1;
                    if neigh >= cur {
                        break;
                    }
                }
            }

            // bottom view
            for z in c + 1..grid.column_len() {
                let neigh = grid.get(z, r).unwrap();

                bot += 1;
                if neigh >= cur {
                    break;
                }
            }
            score = max(score, top * left * bot * right);
        }
    }

    return score;
}

pub fn solve() -> (String, String) {
    let input_str = util::get_input("inputs/day8");
    let (p1, p2) = parts1_2(input_str);

    return (
        DAY_STR.to_string(),
        String::from(format!("\n\tPart 1: {:?}\n\tPart 2: {:?}", p1, p2)),
    );
}

pub fn parts1_2(input_str: String) -> (usize, usize) {
    let grid: Array2D<i32> = Array2D::from_rows(
        &input_str
            .split("\n")
            .map(|l| {
                l.chars()
                    .map(|n| n.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>(),
    );

    let trees_vis = vis_from_axis(&grid);
    let max_score = scenic_score(&grid);

    return (trees_vis, max_score);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parts1_2() {
        let input_str = super::util::get_input("inputs/day8_test");

        let (p1, p2) = super::parts1_2(input_str);

        assert_eq!(p1, 21);
        assert_eq!(p2, 8);
    }
}
