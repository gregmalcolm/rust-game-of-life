pub struct Location {
    x: int,
    y: int
}

pub struct Cells {
    live_cells: ~[Location]
}

impl Location {
    pub fn new (x: int, y: int) -> Location {
        Location { x: x, y: y}
    }
}

impl Cells {
    pub fn new() -> Cells {
        Cells { live_cells: ~[] }
    }

    pub fn add_live_cell(&mut self, x: int, y: int) {
        self.live_cells.push(Location {x: x, y: y});
    }

    pub fn neighbor_count(&self, x: int, y: int) -> uint {
        self.live_cells.len()
    }
}

