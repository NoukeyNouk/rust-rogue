use text_io::read;

pub mod engine;
pub mod player;
pub mod items;
pub mod enemies;
pub mod actions;
pub mod map;

use engine::game_loop;

fn main() {
    println!("It's time to explore The depths!");
    println!("0. not for me (exit).");
    println!("1. Let's GO!!");

    let command: char = read!();

    match command {
        '0' => (),
        '1' => {
            game_loop();
        }
        _ => println!("Unknown command."),
    }

    println!("Goodbye!");

}
