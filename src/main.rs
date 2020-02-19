use maze::create_grid;
use maze::parse_algorithm;
use maze::perform_alg;
extern crate clap;
use clap::Clap;
use maze::format;
use std::time::{Duration, Instant};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K.")]
struct Opts {
    #[clap(short = "a", long = "algorithm", default_value = "bta")]
    algorithm: String,

    #[clap(short = "x", long = "x", default_value = "5")]
    x: usize,

    #[clap(short = "y", long = "y", default_value = "5")]
    y: usize,

    #[clap(short = "s", long = "silent")]
    silent: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let alg = opts.algorithm.clone();
    let grid_build_instant = Instant::now();
    let g = create_grid(opts.x, opts.y);
    let grid_build_time = grid_build_instant.elapsed().as_millis();
    let alg_instant = Instant::now();
    match parse_algorithm(opts.algorithm) {
        Some(alg) => perform_alg(&g, alg),
        None => println!("Unknown algorithm supplied."),
    }
    let alg_time = alg_instant.elapsed().as_millis();
    if !opts.silent {
        println!("{}", format(&g));
    }
    println!(
        "grid:{}ms\nalg-{}:{}ms",
        grid_build_time.to_string(),
        alg,
        alg_time.to_string()
    )

    /*let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut d = Display::new(g, window);

    while let Some(event) = d.window.next() {
        d.update(event);
    }
    */
}
