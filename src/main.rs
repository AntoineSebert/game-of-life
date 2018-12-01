use std::env;
use std::process;

// cls && cargo build && cargo run
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

    match args.len() {
        1 => println!("The world is grey, the mountains old. The forge's fire is ashen cold."),
        _ => {
            eprintln!("This program does not take arguments.");
            process::exit(1);
        },
    } 
}

fn display_world(&world: &[[bool; 20]; 20]) {
    for &element in world.iter() {
        println!("row");
    }
}

/*
 * Entry point.
 */
fn main() {
    check_args();
    let mut world: [[bool; 20]; 20] = [[false; 20]; 20];
    world[0][0] = true;
    display_world(&world);
}
