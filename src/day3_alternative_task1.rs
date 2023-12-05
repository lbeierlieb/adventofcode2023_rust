// the taken approach is not easily adaptable to task2, thus hasn't been pursued further.


use regex::Regex;
use regex::Captures;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<char>,
}

impl Grid {
    fn has_nondot_neighbor(&self, part: &PartNumber) -> bool {
        self.get_neighbor_symbols(part).iter().any(|c| *c != '.')
    }

    fn get_neighbor_symbols(&self, part: &PartNumber) -> Vec<char> {
        let mut chars = Vec::new();
        let (startx, starty) = self.strindex_to_coord(part.start_index);
        let len = part.len();
        for xoffset in 0..(len+2) {
            for yoffset in [0, 2] {
                chars.push(self.get_symbol_at(startx as i32+xoffset as i32-1, starty as i32+yoffset as i32-1));
            }
        }
        chars.push(self.get_symbol_at(startx as i32-1, starty as i32));
        chars.push(self.get_symbol_at(startx as i32+len as i32, starty as i32));
        chars.into_iter().filter_map(|c| c).collect()
    }

    fn get_symbol_at(&self, x: i32, y: i32) -> Option<char> {
        self.coord_to_vecindex(x, y).map(|index| self.grid[index])
    }

    fn coord_to_vecindex(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(y as usize * self.width + x as usize)
        }
    }

    fn strindex_to_coord(&self, index: usize) -> (u32, u32) {
        let x = index % (self.width + 1);
        let y = index / (self.width + 1);
        (x as u32, y as u32)
    }
}

#[derive(Debug)]
struct PartNumber {
    value: u64,
    start_index: usize,
    end_index: usize,
}

impl PartNumber {
    fn from_captures(cap: &Captures) -> PartNumber {
        let regmatch = cap.get(0).unwrap();
        PartNumber {
            value: (cap[0]).parse::<u64>().unwrap(),
            start_index: regmatch.start(),
            end_index: regmatch.end(),
        }
    }

    fn len(&self) -> usize {
        self.end_index - self.start_index
    }
}

pub fn task_one(input: String) -> u64 {
    let width = input.find('\n').unwrap();
    let height = input.len()/(width+1);
    let gridvec = input.lines().flat_map(|line| line.chars()).collect();
    let grid = Grid { width, height, grid: gridvec };
    let re = Regex::new(r"[0-9]+").unwrap();

    let partnumbers = re.captures_iter(&input).map(|cap| PartNumber::from_captures(&cap)).collect::<Vec<_>>();
    let adjecents = partnumbers.iter().filter(|part| grid.has_nondot_neighbor(part));
    let actualpartnumbers = adjecents.map(|part| part.value);

    actualpartnumbers.sum()
}


pub fn task_two(input: String) -> u64 {
    todo!();
}
