
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}
const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

impl Direction {
    fn reversed(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Start,
    Pipe((char, [Direction; 2])),
    Ground,
}

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
    pub tile: Tile,
}

struct Mouse {
    pub grid: Vec<Vec<Point>>,
    visited: Vec<(usize, usize)>,
    pub position: (usize, usize),
    pub previous_movement: Option<((usize, usize), Direction)>,
    number_of_steps: usize,
}

impl Mouse {
    fn new(grid: Vec<Vec<Point>>, mouse_starting_position: (usize, usize)) -> Self {
        Self {
            grid,
            visited: vec![],
            position: mouse_starting_position,
            previous_movement: None,
            number_of_steps: 0,
        }
    }

    fn current_point(&self) -> &Point {
        &self.grid[self.position.1][self.position.0]
    }

    fn point_in_direction(&self, direction: &Direction) -> Option<&Point> {
        let x = self.position.0;
        let y = self.position.1;
        let next_point = match direction {
            Direction::North => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            },
            Direction::West => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            },
            Direction::South => {
                if y < self.grid.len() - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            },
            Direction::East => {
                if x < self.grid[0].len() - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            },
        };

        next_point.map(|(x, y)| &self.grid[y][x])
    }

    fn can_go_from(&self, direction: &Direction) -> bool {
        match self.current_point().tile {
            Tile::Start => true,
            Tile::Pipe((_, opening_directions)) => opening_directions.iter().any(|opening_direction| direction == opening_direction),
            Tile::Ground => direction == &self.previous_movement.unwrap().1,
        }
    }

    fn can_go_towards(&self, direction: &Direction) -> bool {
        let next_point = self.point_in_direction(direction);
        next_point.map(|point| match point.tile {
            Tile::Start | Tile::Ground => true,
            Tile::Pipe((_, opening_directions)) => opening_directions.iter().any(|opening_direction| *direction == opening_direction.reversed()),
        })
        .unwrap_or(false)
    }

    fn start(&mut self, direction: &Direction) -> bool {
        if self.can_go_towards(&direction) {
            self.move_towards(direction.clone());
            true
        } else {
            false
        }
    }

    fn step(&mut self) -> bool {
        for direction in &DIRECTIONS {
            if self.previous_movement.unwrap().1 != direction.reversed() && self.can_go_from(direction) && self.can_go_towards(direction) {
                self.move_towards(direction.clone());
                return true;
            }
        }

        false
    }

    fn move_towards(&mut self, direction: Direction) {
        let (x, y) = {
            let current_point = self.current_point();
            (current_point.x, current_point.y)
        };
        let (next_x, next_y) = {
            let next_point = self.point_in_direction(&direction).unwrap();
            (next_point.x, next_point.y)
        };

        self.previous_movement = Some(((x, y), direction));
        self.position = (next_x, next_y);
        self.number_of_steps += 1;
        self.visited.push((next_x, next_y));
    }

    pub fn loop_around(&mut self) {
        let initial_position = self.position;
        for start_direction in DIRECTIONS {
            self.position = initial_position;
            self.number_of_steps = 0;
            self.visited = vec![initial_position];

            if self.start(&start_direction) {
                let mut current_tile = self.current_point().tile.clone();
                while current_tile != Tile::Start {
                    if self.step() {
                        current_tile = self.current_point().tile.clone();
                    } else {
                        break;
                    }
                }
                if current_tile == Tile::Start {
                    break;
                }
            }
        }
    }

    pub fn count_inner_cells(&self) -> usize {
        let mut inner_cells_count = 0;
        for line in self.grid.iter() {

            let mut vertical_open = false;

            let mut horizontal_first_direction = None;

            for point in line.iter() {
                    match point.tile {
                        Tile::Start => {
                            // To set manually deoending on S type of pipe
                            //horizontal_first_direction =  Some(Direction::South);
                        }
                        Tile::Pipe((char, _)) => {
                            if self.visited.contains(&(point.x, point.y)) {
                                match char {
                                    '-' => {},
                                    '|' => {
                                        vertical_open = !vertical_open;
                                    }
                                    'F' | '7' => {
                                        if horizontal_first_direction.is_some() {
                                            if horizontal_first_direction.unwrap() == Direction::North {
                                                vertical_open = !vertical_open;//Crossing
                                            }
                                            horizontal_first_direction = None;
                                        } else {
                                            horizontal_first_direction = Some(Direction::South);
                                        }
                                    }
                                    'L' | 'J' => {
                                        if horizontal_first_direction.is_some() {
                                            if horizontal_first_direction.unwrap() == Direction::South {
                                                vertical_open = !vertical_open;//Crossing
                                            }
                                            horizontal_first_direction = None;
                                        } else {
                                            horizontal_first_direction = Some(Direction::North);
                                        }
                                    }
                                    _ => panic!("Unexpected character"),
                                }
                            } else {
                                if vertical_open {
                                    inner_cells_count += 1;
                                }
                            }
                        }
                        Tile::Ground => {
                            if vertical_open {
                                inner_cells_count += 1;
                            }
                        }
                    };
            }
        }
        inner_cells_count
    }
}

pub fn run() -> usize {
    let mut mouse_position = None;
    let grid: Vec<Vec<Point>> = split_input_into_lines(INPUT)
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line
                .char_indices()
                .map(|(x, char)| {
                    let tile = match char {
                        '|' => Tile::Pipe(('|', [Direction::North, Direction::South])),
                        '-' => Tile::Pipe(('-', [Direction::West, Direction::East])),
                        'L' => Tile::Pipe(('L', [Direction::North, Direction::East])),
                        'J' => Tile::Pipe(('J', [Direction::North, Direction::West])),
                        '7' => Tile::Pipe(('7', [Direction::West, Direction::South])),
                        'F' => Tile::Pipe(('F', [Direction::East, Direction::South])),
                        '.' => Tile::Ground,
                        'S' => {
                            mouse_position = Some((x, y));
                            Tile::Start
                        },
                        _ => panic!("Unexpected character"),
                    };

                    Point { x, y, tile }
                })
                .collect()
        })
        .collect();
    let mouse_position = mouse_position.unwrap();

    let mut mouse = Mouse::new(grid, mouse_position);
    mouse.loop_around();

    mouse.count_inner_cells()
}
