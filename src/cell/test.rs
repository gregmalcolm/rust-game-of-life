extern mod cell;

mod is_alive_next_go {
    use cell::Cell;

    #[test]
    fn when_there_are_no_neighbors_then_cell_will_be_dead() {
        let cell = Cell { x: 0, y: 0 };
        assert!( !cell.is_alive_next_go() );
    }
}
