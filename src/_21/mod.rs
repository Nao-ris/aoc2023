
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
enum Point {
    Galaxy,
    Empty,
}

fn print_grid(_grid: &Vec<Vec<Point>>) {
    /*println!("========================");
    for x in 0..grid.len() {
        println!();
        for y in 0..grid[0].len() {
            let char = match grid[x][y] {
                Point::Empty => '.',
                Point::Galaxy => '#',
            };
            print!("{char}");
        }
    }
    println!();*/
}

pub fn run() -> usize {
    let mut grid: Vec<Vec<Point>> = split_input_into_lines(INPUT)
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

    print_grid(&grid);

    let mut line = 0;
    while line < grid.len() {
        if grid[line].iter().all(|point| matches!(point, Point::Empty)) {
            grid.insert(line, vec![Point::Empty; grid[line].len()]);
            line += 1;
        }

        line += 1;
    }

    print_grid(&grid);

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
            for line in 0..grid.len() {
                grid[line].insert(column + 1, Point::Empty);
            }
            column += 1;
        }

        column += 1;
    }

    print_grid(&grid);

    let mut galaxies = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if matches!(grid[x][y], Point::Galaxy) {
                galaxies.push((x, y));
            }
        }
    }

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            result += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }

    result
}
