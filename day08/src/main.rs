#![feature(test)]
extern crate test;

use aoc2022::read_lines;

fn read_trees(filename: &str) -> Vec<Vec<usize>> {
    let mut trees = vec![];
    for line in read_lines(filename).unwrap() {
        let row = line.unwrap();
        if !row.is_empty() {
            trees.push(
                row.chars()
                    .map(|ch| ch.to_string().parse::<usize>().unwrap() + 1)
                    .collect(),
            );
        }
    }
    trees
}

fn get_outside_in_heights<'a>(tree_line: &mut dyn Iterator<Item = &'a usize>) -> Vec<usize> {
    tree_line
        .scan(0 as usize, move |height, &tree_height| {
            let current_height = *height;
            *height = (*height).max(tree_height);
            Some(current_height)
        })
        .collect()
}

fn get_inside_out_heights<'a>(tree_line: &mut dyn Iterator<Item = &'a usize>) -> Vec<usize> {
    tree_line
        .scan([0 as usize; 11], |heights, &tree_height| {
            let value = heights[tree_height];
            heights.iter_mut().enumerate().for_each(|(index, height)| {
                if index > tree_height {
                    *height += 1;
                } else {
                    *height = 1
                }
            });

            Some(value)
        })
        .collect()
}

fn cross_traverse_heights<'a, 'b>(
    heights: &'a Vec<Vec<usize>>,
    height_line_builder: fn(&mut dyn Iterator<Item = &'a usize>) -> Vec<usize>,
    height_merger: fn(usize, usize) -> usize,
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];

    for row in heights {
        let left_to_right_heights = height_line_builder(&mut row.iter()).into_iter();
        let right_to_left_heights = height_line_builder(&mut row.iter().rev()).into_iter().rev();
        let row_heights = left_to_right_heights
            .zip(right_to_left_heights)
            .map(|(a, b)| height_merger(a, b));

        result.push(row_heights.collect());
    }

    let get_col = |col: usize| heights.iter().map(move |row| &row[col]);
    for col in 0..heights[0].len() {
        let top_to_bottom_heights = height_line_builder(&mut get_col(col)).into_iter();
        let bottom_to_top_heights = height_line_builder(&mut get_col(col).rev())
            .into_iter()
            .rev();
        let col_heights = top_to_bottom_heights
            .zip(bottom_to_top_heights)
            .map(|(a, b)| height_merger(a, b));

        col_heights
            .enumerate()
            .for_each(|(row, height)| result[row][col] = height_merger(result[row][col], height));
    }

    result
}

fn count_visible_trees(trees: &Vec<Vec<usize>>) -> usize {
    let visible_heights = cross_traverse_heights(&trees, get_outside_in_heights, usize::min);
    let tree_heights = trees.into_iter().flatten();
    let view_heights = visible_heights.into_iter().flatten();

    view_heights
        .zip(tree_heights)
        .filter(|(view_height, tree_height)| *tree_height > view_height)
        .count()
}

fn compute_scenic_score(trees: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    cross_traverse_heights(&trees, get_inside_out_heights, |a, b| a * b)
}

fn main() {
    let trees = read_trees("input.txt");
    let visible_tree_count = count_visible_trees(&trees);

    println!("visible trees: {}", visible_tree_count);

    let scenic_score = compute_scenic_score(&trees);
    let best_scenic_score = *scenic_score.iter().flatten().max().unwrap();

    println!("best scenic score: {}", best_scenic_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_row_count() {
        let trees = read_trees("input_test.txt");
        assert_eq!(trees.len(), 5);
    }

    #[test]
    fn correct_col_count() {
        let trees = read_trees("input_test.txt");
        assert_eq!(trees[0].len(), 5);
    }

    #[test]
    fn correct_visibility_count() {
        let trees = read_trees("input_test.txt");
        let visible_tree_count = count_visible_trees(&trees);
        assert_eq!(visible_tree_count, 21);
    }

    #[test]
    fn correct_scenic_score() {
        let trees = read_trees("input_test.txt");
        let scenic_score = compute_scenic_score(&trees);
        assert_eq!(scenic_score[1][2], 4);
        assert_eq!(scenic_score[3][2], 8);
    }

    #[bench]
    fn visible_tree_perf(b: &mut test::Bencher) {
        let trees = read_trees("input_test.txt");
        b.iter(|| {
            let visible_tree_count = count_visible_trees(&trees);
        });
    }

    #[bench]
    fn scenic_score_perf(b: &mut test::Bencher) {
        let trees = read_trees("input_test.txt");
        b.iter(|| {
            let scenic_score = compute_scenic_score(&trees);
            let best_scenic_score = *scenic_score.iter().flatten().max().unwrap();        
        });
    }
}
