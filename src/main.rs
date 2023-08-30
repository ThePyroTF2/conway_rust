macro_rules! scanln {
    () => {
        std::io::stdin().lines().next().unwrap().unwrap()
    };
}

struct Position {
    x: usize,
    y: usize,
}

struct Conway {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}
impl Conway {
    pub fn new(width: usize, height: usize) -> Self {
        Conway {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn set_bulk(&mut self, field: Vec<Vec<bool>>) -> Result<(), ()> {
        if field.len() != self.height {
            return Err(());
        }
        for row in &field {
            if row.len() != self.width {
                return Err(());
            }
        }
        self.cells = field;
        Ok(())
    }

    pub fn set(&mut self, position: Position, value: bool) -> Result<(), ()> {
        if position.x >= self.width || position.y >= self.height {
            return Err(());
        }
        self.cells[position.y][position.x] = value;
        Ok(())
    }

    pub fn toggle(&mut self, position: Position) -> Result<(), ()> {
        if position.x >= self.width || position.y >= self.height {
            return Err(());
        }
        self.cells[position.y][position.x] = !self.cells[position.y][position.x];
        Ok(())
    }

    pub fn tick(&mut self) {
        // Rules of Conway's Game of Life:
        // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // 2. Any live cell with two or three live neighbours lives on to the next generation.
        // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        //
        // Cells on the edge of the field should consider the opposite edge as their neighbor
        // (this will allow gliders to wrap around)

        let mut new_cells = vec![vec![false; self.width]; self.height];
        for (y, row) in new_cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let mut neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = (x as isize + dx) as usize % self.width;
                        let ny = (y as isize + dy) as usize % self.height;
                        if self.cells[ny][nx] {
                            neighbors += 1;
                        }
                    }
                }
                *cell = matches!(
                    (self.cells[y][x], neighbors),
                    (true, 2) | (true, 3) | (false, 3)
                );
            }
        }
    }
}
impl std::fmt::Display for Conway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", if *cell { "■" } else { "□" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> ! {
    clearscreen::clear().unwrap();
    let mut conway = Conway::new(10, 10);
    // Make a simple glider
    conway
        .set_bulk(vec![
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, true, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, true, false, false, false, false,
            ],
            vec![
                false, false, false, true, true, true, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
        ])
        .unwrap();
    loop {
        println!("{}", conway);
        conway.tick();
        scanln!();
        clearscreen::clear().unwrap();
    }
}
