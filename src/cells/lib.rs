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
        use std::num::abs;

        let mut count = 0;
        self.live_cells.iter().filter(|&cell|
            abs((*cell).x - x) <= 1 && abs((*cell).y - y) <= 1
        ).len()
    }
}

