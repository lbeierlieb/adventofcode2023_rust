#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Start,
    NonStart(Direction, Direction),
    Ground,
}

impl Pipe {
    fn parse(c: char) -> Pipe {
        match c {
            'S' => Pipe::Start,
            'L' => Pipe::NonStart(Direction::Up, Direction::Right),
            'J' => Pipe::NonStart(Direction::Up, Direction::Left),
            'F' => Pipe::NonStart(Direction::Down, Direction::Right),
            '7' => Pipe::NonStart(Direction::Down, Direction::Left),
            '-' => Pipe::NonStart(Direction::Left, Direction::Right),
            '|' => Pipe::NonStart(Direction::Up, Direction::Down),
            '.' => Pipe::Ground,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Field {
    size: usize,
    grid: Vec<Pipe>,
}

impl Field {
    fn parse(input: &str) -> Field {
        let lines: Vec<_> = input.lines().collect();
        let size = lines[0].chars().count();
        let grid = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(|c| Pipe::parse(c))
            .collect();
        Field { size, grid }
    }

    fn get_start_index(&self) -> usize {
        self.grid.iter().position(|p| *p == Pipe::Start).unwrap()
    }
}

pub fn task_one(input: String) -> u64 {
    dbg!(Field::parse(&input));
    0
}

pub fn task_two(input: String) -> u64 {
    0
}
