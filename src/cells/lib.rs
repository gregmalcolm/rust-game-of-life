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
    pub fn new_map(off_x: int, off_y: int, cells_map:~[&str]) -> Cells {
        let mut cells = Cells::new();
        cells.add_live_cell_map(off_x, off_y, cells_map);
        cells
    }

    pub fn add_live_cell(&mut self, x: int, y: int) {
        self.live_cells.push(Location {x: x, y: y});
    }

    pub fn neighbor_count(&self, x: int, y: int) -> uint {
        use std::num::abs;

        self.live_cells.iter().filter(|&cell|
            ((*cell).x != x || (*cell).y != y)
            && abs((*cell).x - x) <= 1 && abs((*cell).y - y) <= 1
        ).len()
    }

    pub fn is_alive(&self, x: int, y: int) -> bool {
        self.live_cells.iter().filter(|&cell|
            (*cell).x == x && (*cell).y == y
        ).len() > 0
    }

    pub fn is_alive_next_go(&self, x: int, y: int) -> bool {
        if self.is_alive(x, y) {
            self.is_still_alive(x, y)
        } else {
            !self.is_still_dead(x, y)
        }
    }

    fn add_live_cell_map(&mut self, off_x: int, off_y:int, cells_map:~[&str]) {
        let mut y = off_y;
        for row in cells_map.iter() {
            let mut x = off_x;
            for glyph in row.iter() {
                if glyph == '*' || glyph == 'A' {
                    self.add_live_cell(x, y);
                }
                x += 1;
            }
            y += 1;
        }
    }


    fn is_still_alive(&self, x: int, y: int) -> bool {
        match self.neighbor_count(x, y) {
            2..3 => true,
            _    => false
        }
    }

    fn is_still_dead(&self, x: int, y: int) -> bool {
        self.neighbor_count(x, y) != 3
    }
}

