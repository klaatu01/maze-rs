use maze::create_grid;

#[test]
fn creates_grid_with_dims() {
    let grid = create_grid(10, 10);
    assert_eq!(grid.x, 10);
    assert_eq!(grid.y, 10);
}

#[test]
fn creates_grid_with_borders() {
    let grid = create_grid(3, 3);
    grid.apply_boarder();
    assert_eq!(grid.x, 3);
    assert_eq!(grid.y, 3);
    for x in 0..grid.x {
        for y in 0..grid.y {
            let val = grid.index(x, y).val.get();
            if x == 1 && y == 1 {
                assert_eq!(val, 0);
            } else {
                assert_eq!(val, 1);
            }
        }
    }
}
