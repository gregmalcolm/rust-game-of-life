
#[test]
fn cell_is_alive_next_go_when_there_are_no_neighbors_then_cell_will_be_dead() {
    let cell = Cell { x: 0, y: 0 };
    assert!( !cell.is_alive_next_go() );
}

struct Cell {
    x: int,
    y: int
}

impl Cell {
    fn is_alive_next_go(&self) -> bool {
        false
    }
}

fn main() {
    println("Game of life!");
}
