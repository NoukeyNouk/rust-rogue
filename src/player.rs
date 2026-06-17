use crate::items::Item;
use rand::Rng;

pub enum Action {
    // Left,
    // Right,
    // Down,
    // Up,
    SkipTurn,
    EquipWeapon(usize),
    Heal,
    Attack,
    Research,
}

pub struct Player {
    hp: i32,
    max_hp: i32,
    weapon: Option<Item>,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn new() -> Self {
        let mut player = Player {
            hp: 100,
            max_hp: 100,
            weapon: None,
            inventory: Vec::with_capacity(10),
        };

        player.inventory.push(Item::give_sword());
        player
    }

    pub fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Your Inventory is empty!!");
            return;
        }
        println!("Your inventory:");

        let mut i = 1;
        for item in &self.inventory {
            print!("{i}. ");
            item.print();
            i += 1;
        }
        print!("\n");
    }

    pub fn act(&mut self, action: Action) {
        match action {
            Action::EquipWeapon(index) => {
                if let Some(weapon) = self.weapon.take() {
                    self.inventory.push(weapon);
                    let len = self.inventory.len();
                    self.inventory.swap(len - 1, index);
                }
                self.weapon = self.inventory.pop();

                print!("New weapon [");
                self.weapon.as_ref().unwrap().print();
                print!("] equipped!");
            }
            Action::SkipTurn => (),
            Action::Research => println!("You may now think you found smthing."),
            Action::Heal | Action::Attack => todo!(),
        
        }
    }
}
