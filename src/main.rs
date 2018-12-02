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
 * @param   world   the two dimensional array to display.
 */
fn display_world(&world: &[[bool; WORLD_X]; WORLD_Y]) {
    /*
    assert!(
        process::Command::new("cls").status().or_else(
            |_| process::Command::new("clear")
        ).unwrap().success()
    );
    */
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
 * Bring the world to the next generation, according the game rules.
 * @param   world   the two dimensional array to process.
 */
fn next_generation(world: &mut &mut [[bool; WORLD_X]; WORLD_Y]) -> bool {
    let mut cell_alive: bool = false;
    let mut cells_to_update: Vec<&mut bool> = Vec::new();

    for n in 0..WORLD_Y {
        for n_2 in 0..WORLD_X {
            let mut neighbors: u8 = 0;
            /*
            match n {
                0 => match n_2 {
                    0 => {
                        if world[n + 1][n_2 + 1] {
                            neighbors += 1;
                        }
                    },
                    WORLD_X => {
                    
                    },
                    _ => {
                    
                    },
                },
                WORLD_Y => match n_2 {
                    0 => {
                    
                    },
                    WORLD_X => {
                    
                    },
                    _ => {
                    
                    },
                },
                _ => match n_2 {
                    0 => {
                    
                    },
                    WORLD_X => {
                    
                    },
                    _ => {
                    
                    }, 
                },
            };
            */
            if 0 < neighbors {
                cell_alive = true;
            }
            let is_alive: bool = world[n][n_2].clone(); 
            if is_alive {
                if neighbors == 2 || neighbors == 3 {
                    cells_to_update.push(&mut world[n][n_2]);
                }
            }
            else if neighbors == 3 {
                cells_to_update.push(&mut world[n][n_2]);
            }
        }
    }

    for mut element in cells_to_update.iter_mut() {
        **element = !**element;
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
    }
    display_world(&world);
}
