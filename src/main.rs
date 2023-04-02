use std::{
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use log::info;
use rand::random;

/// Conway's game of life
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

fn print(board: &Vec<Vec<bool>>) {
    for line in board {
        for cell in line {
            print!("{}", if *cell { '⚪' } else { '⚫' });
        }

        println!();
    }

    println!();
}

fn update(cell: &mut bool, alive_neighbours: u8) {
    if *cell && !(2..=3).contains(&alive_neighbours) {
        *cell = false;
    } else if !*cell && alive_neighbours == 3 {
        *cell = true;
    }
}

fn count_neighbours(board: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut alive_neighbours: u8 = 0;

    //println!("'{x}':'{y}'");

    // left

    if x > 0 && board[y][x - 1] {
        alive_neighbours += 1;
    }

    //println!("\tleft: {}", x > 0 && board[y][x - 1]);

    // right

    //println!("\tright: {}", x > 0 && board[y][x - 1]);

    if x < board[y].len() - 1 && board[y][x + 1] {
        alive_neighbours += 1;
    }

    // up

    //println!("\tup: {}", x > 0 && board[y][x - 1]);

    if y > 0 && board[y - 1][x] {
        alive_neighbours += 1;
    }

    // down

    //println!("\tdown: {}", x > 0 && board[y][x - 1]);

    if y < board.len() - 1 && board[y + 1][x] {
        alive_neighbours += 1;
    }

    // top left

    //println!("\ttop left: {}", x > 0 && board[y][x - 1]);

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

fn next(board: &mut Vec<Vec<bool>>) {
    let mut alive_neighbors = vec![vec![0; board[0].len()]; board.len()];

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
}

/// Assigns a cell to `ALIVE` with probability 0.5.
fn populate_rand(board: &mut [Vec<bool>]) {
    info!("No initial cells given; random populating...");

    for line in board.iter_mut() {
        for cell in line.iter_mut() {
            *cell = random::<bool>();
        }
    }
}

fn populate_cli(board: &mut [Vec<bool>], coordinates: &str, x: usize, y: usize) {
    let coords: Vec<(usize, usize)> = coordinates
        .split(',')
        .map(|s| s.split_once(':').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    assert!(coords.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap().0 < x);
    assert!(coords.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1 < y);

    info!("Populating board with given cells: {coords:?}...");

    for (x, y) in coords {
        board[y][x] = true;
    }
}

fn main() {
    let args = Args::parse();

    info!("Creating board...");

    assert!(0 < args.x);
    assert!(0 < args.y);

    let mut board: Vec<Vec<bool>> = vec![vec![false; args.x]; args.y];

    if args.coordinates.is_empty() {
        populate_rand(&mut board);
    } else {
        populate_cli(&mut board, &args.coordinates, args.x, args.y);
    }

    let interval = Duration::from_millis(300);
    let mut now = Instant::now();

    for _ in 0..args.generations {
        print!("{}[2J", 27 as char); // clear

        print(&board);
        next(&mut board);

        let elapsed = now.elapsed();
        thread::sleep(if elapsed < interval {
            interval - elapsed
        } else {
            interval
        });
        now = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("it_works am a test");
    }
}

/*
1:1,2:2,1:2,2:1

⚫⚫⚫⚫
⚫⚪⚪⚫
⚫⚪⚪⚫
⚫⚫⚫⚫

1:1,2:2,1:2,2:1,3:3,3:4,4:3,4:4

⚫⚫⚫⚫⚫⚫
⚫⚪⚪⚫⚫⚫
⚫⚪⚪⚫⚫⚫
⚫⚫⚫⚪⚪⚫
⚫⚫⚫⚪⚪⚫
⚫⚫⚫⚫⚫⚫
 */
