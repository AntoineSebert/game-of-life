use std::{
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use log::{info, warn};
use rand::random;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// World x size
    #[arg(short)]
    x: usize,

    /// World y size
    #[arg(short)]
    y: usize,

    /// World generations
    #[arg(long, short, default_value_t = 100)]
    generations: u32,

    /// CSV coordinates of initially alive cells (example: "a:b,c:d,e:f,g:h")
    #[arg(short, long, default_value_t = String::new())]
    coordinates: String,
}

/// A board as a 2D vector of booleans
pub type Board = Vec<Vec<bool>>;

/// Prints the board onto the console.
///
/// # Arguments
///
/// * `board` - A game board
fn print(board: &Board) {
    for line in board {
        for cell in line {
            print!("{} ", if *cell { '⚪' } else { '⚫' });
        }

        println!();
    }

    println!();
}

/// Changes the state of a cell depending on its neighbours and its own state.
///
/// # Arguments
///
/// * `cell` - A cell
/// * `alive_neighbours` - The cell's number of alive neighbours
fn update(cell: &mut bool, alive_neighbours: u8) {
    if *cell && !(2..=3).contains(&alive_neighbours) {
        *cell = false;
    } else if !*cell && alive_neighbours == 3 {
        *cell = true;
    }
}

/// Returns the number of alive neighbours of a cell.
///
/// # Arguments
///
/// * `board` - A game board
/// * `x` - Coordinate of a cell on the X axis
/// * `y` - Coordinate of a cell on the Y axis
///
/// # Panics
///
/// Will panic if [`x`] or [`y`] are outside of the bounds for [`board`].
fn count_neighbours(board: &Board, x: usize, y: usize) -> u8 {
    let mut alive_neighbours: u8 = 0;

    // left
    if x > 0 && board[y][x - 1] {
        alive_neighbours += 1;
    }

    // right
    if x < board[y].len() - 1 && board[y][x + 1] {
        alive_neighbours += 1;
    }

    // up
    if y > 0 && board[y - 1][x] {
        alive_neighbours += 1;
    }

    // down
    if y < board.len() - 1 && board[y + 1][x] {
        alive_neighbours += 1;
    }

    // top left
    if x > 0 && y > 0 && board[y - 1][x - 1] {
        alive_neighbours += 1;
    }

    // top right
    if x < board[y].len() - 1 && y > 0 && board[y - 1][x + 1] {
        alive_neighbours += 1;
    }

    // bottom left
    if x > 0 && y < board.len() - 1 && board[y + 1][x - 1] {
        alive_neighbours += 1;
    }

    // bottom right
    if x < board[y].len() - 1 && y < board.len() - 1 && board[y + 1][x + 1] {
        alive_neighbours += 1;
    }

    alive_neighbours
}

/// Computes the next generation for the given board and returns true if no further progress can happen.
///
/// # Arguments
///
/// * `board` - A game board
fn next(board: &mut Board) -> bool {
    let mut alive_neighbors = vec![vec![0; board[0].len()]; board.len()];
    let old = board.clone();

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            alive_neighbors[y][x] = count_neighbours(board, x, y);
        }
    }

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            update(&mut board[y][x], alive_neighbors[y][x]);
        }
    }

    old == *board
}

/// Assigns all cells of a board to `ALIVE` with probability 0.5.
///
/// # Arguments
///
/// * `board` - A game board
fn populate_rand(board: &mut [Vec<bool>]) {
    info!("No initial cells given; random populating...");

    for line in board.iter_mut() {
        for cell in line.iter_mut() {
            *cell = random::<bool>();
        }
    }
}

/// Assigns the cells of a board from a string containing pairs of coordinates $a \in [0; x[$ and $b \in [0; y[$.
///
/// # Arguments
///
/// * `board` - A game board
/// * `coordinates` - A string containing colon-separated coordinates as CSV
/// * `x` - A max value for the X axis
/// * `y` - A max value for the Y axis
///
/// # Notes
///
/// Out-of-bounds coordinates will be ignored with a warning message.
fn populate_cli(board: &mut [Vec<bool>], coordinates: &str, x: usize, y: usize) {
    let coords: Vec<(usize, usize)> = coordinates
        .split(',')
        .map(|s| s.split_once(':').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .filter(|(a, b)| {
            if *a >= x || *b >= y {
                warn!("Coordinates '{a}:{b}' out of bounds '{x}:{y}': ignored");
                false
            } else {
                true
            }
        })
        .collect();

    info!("Populating board with given cells: {coords:?}...");

    for (x, y) in coords {
        board[y][x] = true;
    }
}

/// Runs the main loop of the game.
///
/// # Arguments
///
/// * `board` - A game board
/// * `generations` - A generation limit
fn game_loop(board: &mut Board, generations: u32) {
    let interval = Duration::from_millis(300);
    let mut now = Instant::now();

    for _ in 0..generations {
        print!("{}[2J", 27 as char); // clear

        print(board);

        if next(board) {
            warn!("No changes from previous generation: terminating");
            break;
        }

        let elapsed = now.elapsed();

        thread::sleep(if elapsed < interval {
            interval - elapsed
        } else {
            interval
        });

        now = Instant::now();
    }
}

/// Conway's game of life
fn main() {
    let args = Args::parse();

    info!("Creating board...");

    assert!(0 < args.x);
    assert!(0 < args.y);

    let mut board: Board = vec![vec![false; args.x]; args.y];

    if args.coordinates.is_empty() {
        populate_rand(&mut board);
    } else {
        populate_cli(&mut board, &args.coordinates, args.x, args.y);
    }

    game_loop(&mut board, args.generations);
}
