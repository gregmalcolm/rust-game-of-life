extern mod cells;

mod neighbor_count {
    use cells::Cells;

    #[test]
    fn when_there_are_no_live_neighbors() {
        let cells = Cells::new();

        assert!( cells.neighbor_count(0, 0) == 0 );
    }

    #[test]
    fn when_there_are_2_live_neighbors() {
        let mut cells = Cells::new();
        cells.add_live_cell(-1, -1);
        cells.add_live_cell(1, 0);

        assert!( cells.neighbor_count(0, 0) == 2 );
    }

    #[test]
    fn only_counts_neighboring_cells() {
        let mut cells = Cells::new();
        cells.add_live_cell(1, 0);

        cells.add_live_cell(2, -3);
        cells.add_live_cell(2, 1);
        cells.add_live_cell(0, -1);
        cells.add_live_cell(4, -1);

        assert!( cells.neighbor_count(2, -1) == 1 );
    }
}

//mod is_alive_next_go {
    //use cell::Cells;

    //#[test]
    //fn when_there_are_no_live_neighbors_then_next_cell_will_be_dead() {
        //let cells = Cells::new();
        //assert!( !cell.is_alive_next_go() );
    //}
//}
