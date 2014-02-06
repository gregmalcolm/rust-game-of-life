pub struct Location {
    x: int,
    y: int
}

pub struct Cells {
    live_cells:  ~[Location],
    width: int,
    height: int
}

impl Location {
    pub fn new (x: int, y: int) -> Location {
        Location { x: x, y: y}
    }
}

impl Cells {
    pub fn new() -> Cells {
        Cells { live_cells: ~[],
                width: 100,
                height: 100}
    }
    pub fn new_map(off_x: int, off_y: int, cells_map:~[&str]) -> Cells {
        let mut cells = Cells::new();
        cells.add_live_cell_map(off_x, off_y, cells_map);
        cells
    }

    pub fn left(&self) -> int {
        -(self.width / 2)
    }

    pub fn right(&self) -> int {
        (self.width / 2)
    }

    pub fn top(&self) -> int {
        -(self.height / 2)
    }

    pub fn bottom(&self) -> int {
        (self.height / 2)
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

    pub fn live_cells(&self, off_x: int, off_y: int,
                                    width: int, height: int)
    -> ~[Location] {
        let mut cells = ~[];
        let right = off_x + width;
        let bottom = off_y + height;
        for cell in self.live_cells.iter().filter(|&cell|
            (*cell).x >= off_x && (*cell).x <= right
            && (*cell).y >= off_y && (*cell).y <= bottom
        ) {
            cells.push(*cell)
        }
        cells
    }

    pub fn randomize(&mut self, chance_of_life: uint) {
        use std::rand::Rng;
        self.live_cells.truncate(0);
        for y in range(self.top(), self.bottom()) {
            for x in range(self.left(), self.right()) {
                let r = std::rand::rng().gen_range(0, 100) as uint;
                if r <= chance_of_life {
                    self.live_cells.push(Location{x: x, y: y});
                }
            }
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

