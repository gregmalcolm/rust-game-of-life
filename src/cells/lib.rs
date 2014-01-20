pub struct Coord {
    x: int,
    y: int
}

pub struct Cells {
    live_cells: ~[Coord]
}

impl Cells {
    pub fn add_live_cell(&mut self, x: int, y: int) {
        self.live_cells.push(Coord {x: x, y: y});
    }

    pub fn neighbor_count(&self, x: int, y: int) -> uint {
        self.live_cells.len()
    }
}

