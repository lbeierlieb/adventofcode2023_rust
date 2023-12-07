use regex::Captures;
use regex::Regex;

#[derive(Debug)]
struct IndexToCoordMapper {
    width: usize,
    _height: usize,
}

impl IndexToCoordMapper {
    fn map(&self, index: usize) -> (usize, usize) {
        let x = index % (self.width + 1);
        let y = index / (self.width + 1);
        (x, y)
    }
}

#[derive(Debug)]
struct PartNumber {
    value: usize,
    x: usize,
    y: usize,
    width: usize,
}

impl PartNumber {
    fn from_captures(cap: &Captures, mapper: &IndexToCoordMapper) -> PartNumber {
        let regmatch = cap.get(0).unwrap();
        let start_index = regmatch.start();
        let end_index = regmatch.end();
        let (x, y) = mapper.map(start_index);
        PartNumber {
            value: (cap[0]).parse::<usize>().unwrap(),
            x,
            y,
            width: end_index - start_index,
        }
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

impl Symbol {
    fn from_captures(cap: &Captures, mapper: &IndexToCoordMapper) -> Symbol {
        let regmatch = cap.get(0).unwrap();
        let start_index = regmatch.start();
        let (x, y) = mapper.map(start_index);
        Symbol {
            value: cap[0].chars().next().unwrap(),
            x,
            y,
        }
    }

    fn is_next_to(&self, part: &PartNumber) -> bool {
        let x = self.x as isize;
        let y = self.y as isize;
        let ox = part.x as isize;
        let oy = part.y as isize;
        let olen = part.width as isize;
        x <= ox + olen && x >= ox - 1 && y <= oy + 1 && y >= oy - 1
    }

    fn get_gear_ratio(&self, parts: &[PartNumber]) -> Option<usize> {
        if self.value != '*' {
            return None;
        }
        let mut adjacents = parts.iter().filter(|part| self.is_next_to(part));
        let adj1 = adjacents.next()?;
        let adj2 = adjacents.next()?;
        Some(adj1.value * adj2.value)
    }
}

fn process(input: String, func: &dyn Fn(&[PartNumber], &[Symbol]) -> usize) -> u64 {
    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mapper = IndexToCoordMapper {
        width,
        _height: height,
    };

    let re_partnumbers = Regex::new(r"[0-9]+").unwrap();
    let partnumbers = re_partnumbers
        .captures_iter(&input)
        .map(|cap| PartNumber::from_captures(&cap, &mapper))
        .collect::<Vec<_>>();

    let re_symbols = Regex::new(r"[^.0-9\n]").unwrap();
    let symbols = re_symbols
        .captures_iter(&input)
        .map(|cap| Symbol::from_captures(&cap, &mapper))
        .collect::<Vec<_>>();

    func(&partnumbers, &symbols) as u64
}

pub fn task_one(input: String) -> u64 {
    process(input, &|partnumbers: &[PartNumber], symbols: &[Symbol]| {
        partnumbers
            .iter()
            .filter(|part| symbols.iter().any(|sym| sym.is_next_to(part)))
            .map(|part| part.value)
            .sum()
    })
}

pub fn task_two(input: String) -> u64 {
    process(input, &|partnumbers: &[PartNumber], symbols: &[Symbol]| {
        symbols
            .iter()
            .filter_map(|sym| sym.get_gear_ratio(&partnumbers))
            .sum()
    })
}
