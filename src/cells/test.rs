extern mod cells;

mod neighbor_count {
    use cells::Cells;

    #[test]
    fn when_there_are_no_live_neighbors() {
        let subject = Cells::new();

        assert!( subject.neighbor_count(0, 0) == 0 );
    }

    #[test]
    fn when_there_are_2_live_neighbors() {
        let subject = Cells::new_map(-1, -1, ~["*  ",
                                               " A*"]);
        assert!( subject.neighbor_count(0, 0) == 2 );
    }

    #[test]
    fn only_counts_neighboring_cells() {
        let subject = Cells::new_map(0, -3, ~["  *  ",
                                              "     ",
                                              "* A *",
                                              " *   ",
                                              "  *  "]);

        assert!( subject.neighbor_count(2, -1) == 1 );
    }
}

mod is_alive {
    use cells::Cells;

    #[test]
    fn when_dead() {
        let subject = Cells::new();
        assert!( !subject.is_alive(0, 0) );
    }

    #[test]
    fn when_alive() {
        let subject = Cells::new_map(3, 2, ~["A"]);
        assert!( subject.is_alive(3, 2) );
    }
}

mod is_alive_next_go {

    mod given_focus_cell_is_alive {
        use cells::Cells;

        #[test]
        fn when_there_is_1_live_neighbor() {
            let subject = Cells::new_map(-1, -1, ~["   ",
                                                   " A ",
                                                   " * "]);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_2_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["  *",
                                                   " A ",
                                                   "*  "]);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_3_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["* *",
                                                   " A ",
                                                   "  *"]);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_4_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["   ",
                                                   "*A ",
                                                   "***"]);
            assert!( !subject.is_alive_next_go(0, 0) );
        }
    }

    mod given_focus_cell_is_dead {
        use cells::Cells;

        #[test]
        fn when_there_is_1_live_neighbor() {
            let subject = Cells::new_map(-1, -1, ~["   ",
                                                   " D*",
                                                   "   "]);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_2_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["*  ",
                                                   " D*",
                                                   "   "]);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_3_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["   ",
                                                   "*D*",
                                                   "*  "]);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_4_live_neighbors() {
            let subject = Cells::new_map(-1, -1, ~["*  ",
                                                   "*D*",
                                                   "  *"]);
            assert!( !subject.is_alive_next_go(0, 0) );
        }
    }
}

mod snapshot {
    use cells::Cells;

    #[test]
    fn snapshot_of_all_current_live_cells() {
        let subject = Cells::new_map(0, 0, ~[" * *   *",
                                                 "    *   ",
                                                 "*       "]);
        let cells = subject.live_cells();
        assert!(cells.len() == 5);
    }

}
