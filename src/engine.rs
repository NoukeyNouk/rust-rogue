use rand::Rng;
use text_io::read;
use text_io::try_read;
use crate::actions::Action;
use crate::player::Player;
use crate::items::Item;
use crate::map::Map;

struct World {
    player: Player,
    map: Map,
}

enum GameState {
    Normal,
    Research,
    Fight,
    Quit,
}


pub fn game_loop() {
    let mut player = Player::new();
    let mut map = Map::new();
    let mut game_state = GameState::Normal;
    let x = rand::thread_rng().gen_range(1..=30);
    let y = rand::thread_rng().gen_range(1..=30);
    player.set_coords(map.cut_coords((x, y)));
    map.place_player(player.coords());

    let x = rand::thread_rng().gen_range(1..=30);
    let y = rand::thread_rng().gen_range(1..=30);
    map.place_spawner(map.cut_coords((x, y)), 3);

    let mut world = World {
        player: player,
        map: map,
    };

    loop {
        match &game_state {
            GameState::Normal => handle_normal(&mut world, &mut game_state),
            GameState::Research => handle_research(&mut world, &mut game_state),
            GameState::Quit => break,
            _ => todo!(),
        }
    }
}

fn handle_normal(world: &mut World, game_state: &mut GameState) {
    world.map.print();
    let action = choose_action(&world.player);
    match action {
        Action::EquipWeapon(index) => world.player.equip_weapon(index),
        Action::SkipTurn => (),
        Action::Research => *game_state = GameState::Research,
        Action::Quit => *game_state = GameState::Quit,
        _ => todo!(),
    }
}

fn handle_research(world: &mut World, game_state: &mut GameState) {
    world.map.print();
    println!();
    let action_buffer: String = read!();
    for action in action_buffer.as_bytes().iter() {
        match *action as char {
            'h' | 'a' => move_player(world, "left"),
            'j' | 's' => move_player(world, "down"),
            'l' | 'd' => move_player(world, "right"),
            'k' | 'w' => move_player(world, "up"),
            'q' => {
                *game_state = GameState::Normal;
                return;
            }
            _ => println!("wasd or hjkl for moving, q - stop research"),
        }
    }
}


fn move_player(world: &mut World, direction: &str) {
    let source = world.player.coords();
    let mut target = source;
    match direction {
        "right" => target.1 += 1,
        "left" => target.1 -= 1,
        "up" => target.0 -= 1,
        "down" => target.0 += 1,
        _ => panic!("Unknown direction for moving player"),
    }
    if world.map.can_move(target) {
        world.player.set_coords(target);
        world.map.move_player(source, target);
    }
}

pub fn infinite_read() -> char {
    let mut command: char;
    loop {
        command = read_char();
        match command {
            'Y'| 'y' | 'n' | 'N' => break,
            _ => println!("put only 'y' or 'n'."),
        }
    }
    return command;
}

fn choose_action(player: &Player) -> Action {
    println!("\nYour turn!!");
    println!("Avaliable actions:");
    println!("0. exit dungeon.");
    println!("1. Attack.");
    println!("2. Heal.");
    println!("3. Equip weapon.");
    println!("4. Find stuff!!");

    let mut end = 0;
    let mut command: char;
    let mut chosen: Action = Action::SkipTurn;
    while end != 1 {
        end = 1;
        command = read_char();

        match command {
            '0' => chosen = Action::Quit,
            '1' => chosen = Action::Attack,
            '2' => chosen = Action::Heal,
            '3' => {
                player.print_inventory();
                if player.inventory.is_empty() {
                    chosen = Action::SkipTurn;
                    break;
                }
                let chosen_item = choose_item(&player.inventory);
                chosen = Action::EquipWeapon(chosen_item);
            }
            '4' => chosen = Action::Research,
            _ => {
                println!("Unknown command, try again.");
                end = 0;
            }
        }
    }
    chosen
}

fn choose_item(inventory: &Vec<Item>) -> usize {
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

pub fn read_char() -> char {
    let command: char;
    loop {
        let option: Result<char, _> = try_read!();
        match option {
            Ok(c) => {
                command = c;
                break;
            }
            Err(_) => println!("Unknown command"),
        }
    }
    command
}

fn random_coords() -> (usize, usize) {
    let x = rand::thread_rng().gen_range(1..=30);
}
