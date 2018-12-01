use std::env;
use std::process;

// cls && cargo build && cargo run

/*
 * Constants
 */
const WORLD_X: usize = 20;
const WORLD_Y: usize = 20;

/*
 * Test the project.
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("it_works am a test");
    }
}

/*
 * If arguments are found, the program exit, otherwise print a sentence.
 */
fn check_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        eprintln!("This program does not take arguments.");
        process::exit(1);
    }
}

/*
 * Display the world in the console.
 */
fn display_world(&world: &[[bool; WORLD_X]; WORLD_Y]) {
    for &element in world.iter() {
        for &element_2 in element.iter() {
            if element_2 {
                print!("⚪ ");
            }
            else {
                print!("⚫ ");
            }
        }
        println!();
    }
    println!();
}

/*
 * Count the living neigbors of a cell.
 */
fn neighbors(/*world: &&mut [[bool; WORLD_X]; WORLD_Y], x: u8, y: u8*/) -> u8 {
    return 0;
}

/*
 * Bring the world to the next generation, according the game rules.
 */
fn next_generation(world: &mut &mut [[bool; WORLD_X]; WORLD_Y]) -> bool {
    let mut cell_alive: bool = false;
    let mut cells_to_update: Vec<&mut bool> = Vec::new();
    for mut element in world.iter_mut() {
        for mut element_2 in element.iter_mut() {
            let neighbors: u8 = neighbors();
            if 0 < neighbors {
                cell_alive = true;
            }
            if *element_2 {
                if neighbors == 2 || neighbors == 3 {
                    cells_to_update.push(element_2);
                }
            }
            else if neighbors == 3 {
                cells_to_update.push(element_2);
            }
        }
    }

    for mut element in cells_to_update.iter_mut() {
        *element = &mut !**element;
    }

    return cell_alive;
}

/*
 * Entry point.
 */
fn main() {
    check_args();
    let mut world: [[bool; WORLD_X ]; WORLD_Y] = [[false; WORLD_X]; WORLD_Y];
    world[0][0] = true;
    display_world(&world);
    while next_generation(&mut &mut world) {
        display_world(&world);
        /*
        assert!(
            process::Command::new("cls").status().or_else(
               |_| process::Command::new("clear")
            ).unwrap().success()
        );
        */
    }
}
