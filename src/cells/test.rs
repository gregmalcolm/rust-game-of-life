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
        let mut subject = Cells::new();
        subject.add_live_cell(-1, -1);
        subject.add_live_cell(1, 0);

        assert!( subject.neighbor_count(0, 0) == 2 );
    }

    #[test]
    fn only_counts_neighboring_cells() {
        let mut subject = Cells::new();
        subject.add_live_cell(1, 0);

        subject.add_live_cell(2, -3);
        subject.add_live_cell(2, 1);
        subject.add_live_cell(0, -1);
        subject.add_live_cell(4, -1);

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
        let mut subject = Cells::new();
        subject.add_live_cell(3, 2);
        assert!( subject.is_alive(3, 2) );
    }
}

mod is_alive_next_go {

    mod given_focus_cell_is_alive {
        use cells::Cells;

        fn setup() -> Cells {
            let mut subject = Cells::new();
            subject.add_live_cell(0, 0);
            subject
        }

        #[test]
        fn when_there_is_1_live_neighbor() {
            let mut subject = setup();
            subject.add_live_cell(0, 1);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_2_live_neighbors() {
            let mut subject = setup();
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(0, 1);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_3_live_neighbors() {
            let mut subject = setup();
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(0, 1);
            subject.add_live_cell(1, 0);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_4_live_neighbors() {
            let mut subject = setup();
            subject.add_live_cell(-1, -1);
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(-1, 1);
            subject.add_live_cell(0, 1);
            assert!( !subject.is_alive_next_go(0, 0) );
        }
    }

    mod given_focus_cell_is_dead {
        use cells::Cells;

        #[test]
        fn when_there_is_1_live_neighbor() {
            let mut subject = Cells::new();
            subject.add_live_cell(0, 1);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_2_live_neighbors() {
            let mut subject = Cells::new();
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(0, 1);
            assert!( !subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_3_live_neighbors() {
            let mut subject = Cells::new();
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(0, 1);
            subject.add_live_cell(1, 0);
            assert!( subject.is_alive_next_go(0, 0) );
        }

        #[test]
        fn when_there_are_4_live_neighbors() {
            let mut subject = Cells::new();
            subject.add_live_cell(-1, -1);
            subject.add_live_cell(-1, 0);
            subject.add_live_cell(-1, 1);
            subject.add_live_cell(0, 1);
            assert!( !subject.is_alive_next_go(0, 0) );
        }
    }
}
