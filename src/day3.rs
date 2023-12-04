use color_eyre::Report;
use fnv::FnvHashSet;
use regex::Regex;
use tracing::info;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Cell {
    row: i32,
    col: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Number {
    cell: Cell,
    value: i32,
}

#[derive(Debug)]
struct Symbol {
    cell: Cell,
    symbol: char,
}

impl Number {
    fn occupied_cells(&self) -> Vec<Cell> {
        let mut n = self.value;
        let mut col = self.cell.col;
        let mut cells = Vec::new();
        while n > 0 {
            cells.push(Cell { row: self.cell.row, col });
            n /= 10;
            col += 1;
        }
        cells
    }
}

impl Cell {
    fn adjacent_cells(&self) -> FnvHashSet<Cell> {
        let mut set = FnvHashSet::default();
        for &drow in &[-1, 0, 1] {
            for &dcol in &[-1, 0, 1] {
                if drow == 0 && dcol == 0 { continue }
                let arow = self.row + drow;
                let acol = self.col + dcol;
                if arow >= 0 && acol >= 0 {
                    set.insert(Cell { row: arow, col: acol });
                }
            }
        }
        set
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let re = Regex::new(r"(\d+)|([^.])").unwrap();

    for (row, line) in input.lines().enumerate() {
        println!("{}", line);
        for caps in re.captures_iter(line) {
            if let Some(m) = caps.get(1) {
                numbers.push(Number {
                    cell: Cell { row: row as i32, col: m.start() as i32 },
                    value: m.as_str().parse().unwrap()
                })
            } else if let Some(m) = caps.get(2) {
                symbols.push(Symbol {
                    cell: Cell { row: row as i32, col: m.start() as i32 },
                    symbol: m.as_str().chars().next().unwrap()
                })
            } else {
                panic!("no match for group 1 or 2");
            }
        }
    }

    let mut part1 = 0;
    let mut adjacent_to_symbols = FnvHashSet::default();
    for symbol in &symbols {
        for cell in symbol.cell.adjacent_cells() {
            adjacent_to_symbols.insert(cell);
        }
    }

    for n in &numbers {
        if n.occupied_cells().iter().any(|c| adjacent_to_symbols.contains(c)) {
            part1 += n.value;
        }
    }


    info!(day=3, part=1, answer=part1);

    let mut part2 = 0;
    for symbol in symbols {
        if symbol.symbol == '*' {
            let adjacent_cells = symbol.cell.adjacent_cells();
            let mut number_count = 0;
            let mut number_product = 1;
            for n in &numbers {
                if n.occupied_cells().iter().any(|oc| adjacent_cells.contains(oc)) {
                    number_count += 1;
                    number_product *= n.value;
                }
            }
            if number_count == 2 {
                part2 += number_product;
            }
        }
    }

    info!(day=3, part=2, answer=part2);

    Ok(())
}
