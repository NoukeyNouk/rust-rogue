use text_io::read;
use text_io::try_read;
use crate::actions::Action;
use crate::player::Player;
use crate::items::Item;

pub fn infinite_read() -> char {
    let mut command: char;
    loop {
        command = read!();
        match command {
            'Y'| 'y' | 'n' | 'N' => break,
            _ => println!("put only 'y' or 'n'."),
        }
    }
    return command;
}

fn choose_action(player: &Player) -> Option<Action> {
    println!("\nYour turn!!");
    println!("Avaliable actions:");
    println!("0. exit dungeon.");
    println!("1. Attack.");
    println!("2. Heal.");
    println!("3. Equip weapon.");
    println!("4. Find stuff!!");

    let mut end = 0;
    let mut command: char;
    while end != 1 {
        end = 1;
        command = read!();
        match command {
            '0' => return None,
            '1' => return Some(Action::Attack),
            '2' => return Some(Action::Heal),
            '3' => {
                player.print_inventory();
                if player.inventory.is_empty() {
                    return Some(Action::SkipTurn);
                }
                let chosen = choose_item(&player.inventory);
                return Some(Action::EquipWeapon(chosen));
            }
            '4' => return Some(Action::Research),
            _ => {
                println!("Unknown command, try again.");
                end = 0;
            }
        }
    }
    None
}

fn choose_item(inventory: &Vec<Item>) -> usize { // can panic!!
    let mut index: usize;
    loop {
        index = match try_read!() {
            Ok(i) => i,
            Err(_) => 0,
        };
        if index > 0 && index <= inventory.len() {
            break;
        }
        println!("Try again.");
    }
    index - 1
}

pub fn game_loop() {
    let mut player = Player::new();
    loop {
        let Some(action) = choose_action(&player) else {
            return;
        };
        match action {
            Action::EquipWeapon(index) => player.equip_weapon(index),
            _ => todo!(),
        }
    }
}
