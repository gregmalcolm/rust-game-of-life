extern mod cells;

use cells::Cells;

pub struct View {
    width: uint,
    height: uint,
    cells: Cells
}

impl View {
    pub fn new() -> View {
        let mut cells = Cells::new();
        cells.randomize(60, 26, 10);
        let view = View { width: 60,
                          height: 30,
                          cells: cells};
        view.clear_screen();
        view.render();
        view
    }

    fn clear_screen(&self) {
        print("\x1b[2J");
    }

    fn render(&self) {
        self.moveCursor(0,0);

        let live_cells = self.cells.live_cells();
        for cell in live_cells.iter() {
            self.moveCursor(cell.x as uint, cell.y as uint);
            print("*");
        }
    }

    fn moveCursor(&self, x: uint, y: uint) {
        print!("\x1b[{:u};{:u}H", y, x);
    }
}
