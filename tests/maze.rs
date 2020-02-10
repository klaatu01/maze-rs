use maze::create_grid;

#[test]
fn creates_grid_with_dims() {
    let grid = create_grid(50, 50);
    assert_eq!(grid.x, 50);
    assert_eq!(grid.y, 50);
}
