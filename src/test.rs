#[test]
fn init() {
    assert_eq!(3, 2+1);
}

#[test]
fn puzzle_functions() {

    let mut puzzle_test = crate::Puzzle::new_from_input(3);

    // get blanc cell index OK
    assert_eq!(puzzle_test.get_blanck_cell_index(),0);
    assert_eq!(puzzle_test.grid[0],0);

    puzzle_test.grid[0] = 3;
    puzzle_test.grid[3] = 0;

    assert_ne!(puzzle_test.get_blanck_cell_index(),0);
    assert_ne!(puzzle_test.grid[0],0);

    assert_eq!(puzzle_test.get_blanck_cell_index(),3);
    assert_eq!(puzzle_test.grid[3],0);

    // get cell index
    assert_eq!(puzzle_test.get_index_cell(8),8);
    assert_eq!(puzzle_test.get_index_cell(0),3);

    assert_ne!(puzzle_test.get_index_cell(4),6);
    assert_eq!(puzzle_test.get_index_cell(3),0);

    println!("puzzle : {}",puzzle_test);
    // is cell adjacent (right, left)
    assert_eq!(puzzle_test.is_cell_movable(4),true);
    assert_eq!(puzzle_test.is_cell_movable(2),false);
    assert_eq!(puzzle_test.is_cell_movable(5),false);
    // is cell adjacent (up, down)
    assert_eq!(puzzle_test.is_cell_movable(3),true);
    assert_eq!(puzzle_test.is_cell_movable(1),false);
    assert_eq!(puzzle_test.is_cell_movable(6),true);
    assert_eq!(puzzle_test.is_cell_movable(2),false);
    assert_eq!(puzzle_test.is_cell_movable(7),false);

    // moving cell
    assert_eq!(puzzle_test.move_cell(2),false);
    assert_eq!(puzzle_test.move_cell(5),false);
    assert_eq!(puzzle_test.move_cell(4),true);
    println!("puzzle : {}",puzzle_test);
    assert_eq!(puzzle_test.move_cell(1),true);
    println!("puzzle : {}",puzzle_test);

    //cell adjacent
    assert_eq!(puzzle_test.is_cell_adjacent(3, 0),true);
    assert_eq!(puzzle_test.is_cell_adjacent(3, 4),true);
    assert_ne!(puzzle_test.is_cell_adjacent(3, 2),true);
    assert_ne!(puzzle_test.is_cell_adjacent(3, 1),true);

    assert_eq!(puzzle_test.is_cell_adjacent(7, 8),true);
    assert_eq!(puzzle_test.is_cell_adjacent(7, 6),true);
    assert_eq!(puzzle_test.is_cell_adjacent(7, 1),true);
    assert_eq!(puzzle_test.is_cell_adjacent(7, 5),false);

    // get_cell_adjacent_serie
    let test_vec:Vec<u8> = vec!(6,7,8);
    assert_eq!(puzzle_test.get_cell_adjacent_serie(),test_vec);

    // is_win
    assert_eq!(puzzle_test.is_win(),false);
    puzzle_test.grid = vec!(0,1,2,5,4,3,6,7,8);
    assert_eq!(puzzle_test.is_win(),true);
}