use maze::create_grid;
use maze::perform_bta;
fn main() {
    let g = create_grid(50, 50);
    perform_bta(&g);
    println!("{}", g.to_string())
}
