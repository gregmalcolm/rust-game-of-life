extern mod cells;

mod neighbor_count {
    use cells::Cells;

    #[test]
    fn when_there_are_no_live_neighbors_then_count_is_0() {
        let cells = Cells { live_cells: ~[] };
        assert!( cells.neighbor_count(0, 0) == 0 );
    }

    #[test]
    fn when_there_are_2_live_neighbors_then_count_is_2() {
        let mut cells = Cells { live_cells: ~[] };
        cells.add_live_cell(-1, -1);
        cells.add_live_cell(1, 0);
        assert!( cells.neighbor_count(0, 0) == 2 );
    }
}

//mod is_alive_next_go {
    //use cell::Cell;

    //#[test]
    //fn when_there_are_no_live_neighbors_then_next_cell_will_be_dead() {
        //let cell = Cell { x: 0, y: 0 };
        //assert!( !cell.is_alive_next_go() );
    //}
//}


