
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
enum Point {
    Galaxy,
    Empty,
}

fn new_line(line_index: usize, galaxies: &mut Vec<((usize, usize), (usize, usize))>) {
    for galaxy in galaxies.iter_mut() {
        if galaxy.0.0 > line_index {
            galaxy.1.0 += 999999;
        }
    }
}

fn new_column(column_index: usize, galaxies: &mut Vec<((usize, usize), (usize, usize))>) {
    for galaxy in galaxies.iter_mut() {
        if galaxy.0.1 > column_index {
            galaxy.1.1 += 999999;
        }
    }
}

pub fn run() -> usize {
    let grid: Vec<Vec<Point>> = split_input_into_lines(INPUT)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| {
                    match char {
                        '.' => Point::Empty,
                        '#' => Point::Galaxy,
                        _ => panic!("Unexpected char"),
                    }
                })
                .collect()
        })
        .collect();

    let mut galaxies = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if matches!(grid[x][y], Point::Galaxy) {
                galaxies.push(((x, y), (x, y)));
            }
        }
    }

    let mut line = 0;
    while line < grid.len() {
        if grid[line].iter().all(|point| matches!(point, Point::Empty)) {
            new_line(line, &mut galaxies);
        }

        line += 1;
    }

    let mut column = 0;
    while column < grid[0].len() {
        let mut all_empty = true;
        for line in 0..grid.len() {
            if matches!(grid[line][column], Point::Galaxy) {
                all_empty = false;
                break;
            }
        }

        if all_empty {
            new_column(column, &mut galaxies);
        }

        column += 1;
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            result += galaxies[i].1.0.abs_diff(galaxies[j].1.0) + galaxies[i].1.1.abs_diff(galaxies[j].1.1);
        }
    }

    result
}
