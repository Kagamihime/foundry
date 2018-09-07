extern crate rlife;

use rlife::Grid;

use std::fs;

#[test]
fn test_toroidal_load_file() {
    let grid = Grid::from_file("tests/input_files/sanity_check_toroidal.life").unwrap();

    // Check the `Grid` is correct
    assert_eq!("#Toroidal Life", grid.get_format());
    assert_eq!(true, grid.is_toroidal());
    assert_eq!(vec![2, 3], grid.get_survival());
    assert_eq!(vec![3], grid.get_birth());
    assert_eq!((5, 5), grid.get_grid_size());

    assert_eq!(0, grid.get_cell_state(0, 0));
    assert_eq!(0, grid.get_cell_state(0, 1));
    assert_eq!(0, grid.get_cell_state(0, 2));
    assert_eq!(0, grid.get_cell_state(0, 3));
    assert_eq!(0, grid.get_cell_state(0, 4));
    assert_eq!(0, grid.get_cell_state(1, 0));
    assert_eq!(0, grid.get_cell_state(1, 1));
    assert_eq!(0, grid.get_cell_state(1, 2));
    assert_eq!(0, grid.get_cell_state(1, 3));
    assert_eq!(0, grid.get_cell_state(1, 4));
    assert_eq!(0, grid.get_cell_state(2, 0));
    assert_eq!(255, grid.get_cell_state(2, 1));
    assert_eq!(255, grid.get_cell_state(2, 2));
    assert_eq!(255, grid.get_cell_state(2, 3));
    assert_eq!(0, grid.get_cell_state(2, 4));
    assert_eq!(0, grid.get_cell_state(3, 0));
    assert_eq!(0, grid.get_cell_state(3, 1));
    assert_eq!(0, grid.get_cell_state(3, 2));
    assert_eq!(0, grid.get_cell_state(3, 3));
    assert_eq!(0, grid.get_cell_state(3, 4));
    assert_eq!(0, grid.get_cell_state(4, 0));
    assert_eq!(0, grid.get_cell_state(4, 1));
    assert_eq!(0, grid.get_cell_state(4, 2));
    assert_eq!(0, grid.get_cell_state(4, 3));
    assert_eq!(0, grid.get_cell_state(4, 4));
}

#[test]
fn test_toroidal_next_gen() {
    let mut grid = Grid::from_file("tests/input_files/sanity_check_toroidal.life").unwrap();

    grid.next_gen();

    // Check the new `Grid` is correct
    assert_eq!("#Toroidal Life", grid.get_format());
    assert_eq!(true, grid.is_toroidal());
    assert_eq!(vec![2, 3], grid.get_survival());
    assert_eq!(vec![3], grid.get_birth());
    assert_eq!((5, 5), grid.get_grid_size());

    assert_eq!(0, grid.get_cell_state(0, 0));
    assert_eq!(0, grid.get_cell_state(0, 1));
    assert_eq!(0, grid.get_cell_state(0, 2));
    assert_eq!(0, grid.get_cell_state(0, 3));
    assert_eq!(0, grid.get_cell_state(0, 4));
    assert_eq!(0, grid.get_cell_state(1, 0));
    assert_eq!(0, grid.get_cell_state(1, 1));
    assert_eq!(255, grid.get_cell_state(1, 2));
    assert_eq!(0, grid.get_cell_state(1, 3));
    assert_eq!(0, grid.get_cell_state(1, 4));
    assert_eq!(0, grid.get_cell_state(2, 0));
    assert_eq!(0, grid.get_cell_state(2, 1));
    assert_eq!(255, grid.get_cell_state(2, 2));
    assert_eq!(0, grid.get_cell_state(2, 3));
    assert_eq!(0, grid.get_cell_state(2, 4));
    assert_eq!(0, grid.get_cell_state(3, 0));
    assert_eq!(0, grid.get_cell_state(3, 1));
    assert_eq!(255, grid.get_cell_state(3, 2));
    assert_eq!(0, grid.get_cell_state(3, 3));
    assert_eq!(0, grid.get_cell_state(3, 4));
    assert_eq!(0, grid.get_cell_state(4, 0));
    assert_eq!(0, grid.get_cell_state(4, 1));
    assert_eq!(0, grid.get_cell_state(4, 2));
    assert_eq!(0, grid.get_cell_state(4, 3));
    assert_eq!(0, grid.get_cell_state(4, 4));
}

#[test]
fn test_toroidal_save_file() {
    let mut grid = Grid::from_file("tests/input_files/sanity_check_toroidal.life").unwrap();

    grid.next_gen();
    grid.save_life_grid("tests/output_files/sanity_check_toroidal.life")
        .unwrap();

    let expected_result =
        fs::read_to_string("tests/output_files/sanity_check_toroidal_expected.life").unwrap();
    let actual_result =
        fs::read_to_string("tests/output_files/sanity_check_toroidal.life").unwrap();

    assert_eq!(expected_result, actual_result);

    fs::remove_file("tests/output_files/sanity_check_toroidal.life").unwrap();
}

#[test]
fn test_resizable_load_file() {
    let grid = Grid::from_file("tests/input_files/sanity_check_resizable.life").unwrap();

    // Check the `Grid` is correct
    assert_eq!("#Resizable Life", grid.get_format());
    assert_eq!(false, grid.is_toroidal());
    assert_eq!(vec![2, 3], grid.get_survival());
    assert_eq!(vec![3], grid.get_birth());
    assert_eq!((3, 5), grid.get_grid_size());

    assert_eq!(255, grid.get_cell_state(0, 0));
    assert_eq!(255, grid.get_cell_state(0, 1));
    assert_eq!(255, grid.get_cell_state(0, 2));
    assert_eq!(0, grid.get_cell_state(0, 3));
    assert_eq!(0, grid.get_cell_state(0, 4));
    assert_eq!(0, grid.get_cell_state(1, 0));
    assert_eq!(0, grid.get_cell_state(1, 1));
    assert_eq!(0, grid.get_cell_state(1, 2));
    assert_eq!(0, grid.get_cell_state(1, 3));
    assert_eq!(0, grid.get_cell_state(1, 4));
    assert_eq!(0, grid.get_cell_state(2, 0));
    assert_eq!(0, grid.get_cell_state(2, 1));
    assert_eq!(0, grid.get_cell_state(2, 2));
    assert_eq!(0, grid.get_cell_state(2, 3));
    assert_eq!(0, grid.get_cell_state(2, 4));
}

#[test]
fn test_resizable_next_gen() {
    let mut grid = Grid::from_file("tests/input_files/sanity_check_resizable.life").unwrap();

    grid.next_gen();

    // Check the new `Grid` is correct
    assert_eq!("#Resizable Life", grid.get_format());
    assert_eq!(false, grid.is_toroidal());
    assert_eq!(vec![2, 3], grid.get_survival());
    assert_eq!(vec![3], grid.get_birth());
    assert_eq!((3, 5), grid.get_grid_size());

    assert_eq!(0, grid.get_cell_state(0, 0));
    assert_eq!(0, grid.get_cell_state(0, 1));
    assert_eq!(255, grid.get_cell_state(0, 2));
    assert_eq!(0, grid.get_cell_state(0, 3));
    assert_eq!(0, grid.get_cell_state(0, 4));
    assert_eq!(0, grid.get_cell_state(0, 5));
    assert_eq!(0, grid.get_cell_state(0, 6));
    assert_eq!(0, grid.get_cell_state(1, 0));
    assert_eq!(0, grid.get_cell_state(1, 1));
    assert_eq!(255, grid.get_cell_state(1, 2));
    assert_eq!(0, grid.get_cell_state(1, 3));
    assert_eq!(0, grid.get_cell_state(1, 4));
    assert_eq!(0, grid.get_cell_state(1, 5));
    assert_eq!(0, grid.get_cell_state(1, 6));
    assert_eq!(0, grid.get_cell_state(2, 0));
    assert_eq!(0, grid.get_cell_state(2, 1));
    assert_eq!(255, grid.get_cell_state(2, 2));
    assert_eq!(0, grid.get_cell_state(2, 3));
    assert_eq!(0, grid.get_cell_state(2, 4));
    assert_eq!(0, grid.get_cell_state(2, 5));
    assert_eq!(0, grid.get_cell_state(2, 6));
    assert_eq!(0, grid.get_cell_state(3, 0));
    assert_eq!(0, grid.get_cell_state(3, 1));
    assert_eq!(0, grid.get_cell_state(3, 2));
    assert_eq!(0, grid.get_cell_state(3, 3));
    assert_eq!(0, grid.get_cell_state(3, 4));
    assert_eq!(0, grid.get_cell_state(3, 5));
    assert_eq!(0, grid.get_cell_state(3, 6));
    assert_eq!(0, grid.get_cell_state(4, 0));
    assert_eq!(0, grid.get_cell_state(4, 1));
    assert_eq!(0, grid.get_cell_state(4, 2));
    assert_eq!(0, grid.get_cell_state(4, 3));
    assert_eq!(0, grid.get_cell_state(4, 4));
    assert_eq!(0, grid.get_cell_state(4, 5));
    assert_eq!(0, grid.get_cell_state(4, 6));
}

#[test]
fn test_resizable_save_file() {
    let mut grid = Grid::from_file("tests/input_files/sanity_check_resizable.life").unwrap();

    grid.next_gen();
    grid.save_life_grid("tests/output_files/sanity_check_resizable.life")
        .unwrap();

    let expected_result =
        fs::read_to_string("tests/output_files/sanity_check_resizable_expected.life").unwrap();
    let actual_result =
        fs::read_to_string("tests/output_files/sanity_check_resizable.life").unwrap();

    assert_eq!(expected_result, actual_result);

    fs::remove_file("tests/output_files/sanity_check_resizable.life").unwrap();
}
